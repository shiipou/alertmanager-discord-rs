use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};
use serenity::{ builder::*, model::prelude::* };
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::env;
use env_logger;
use log::{debug, info, error};

#[derive(Debug, Serialize, Deserialize)]
struct AlertPayload {
    alerts: Option<Vec<Alert>>,
    status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alert {
    status: Option<String>,
    labels: Option<AlertLabels>,
    annotations: Option<AlertAnnotations>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AlertLabels {
    alertname: Option<String>,
    namespace: Option<String>,
    pod: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AlertAnnotations {
    description: Option<String>,
}

struct AppState {
    discord: Arc<serenity::all::Http>,
    guild: PartialGuild,
    channel: Channel,
    guild_channel: GuildChannel,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize the logger
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let channel_id = ChannelId::new(env::var("CHANNEL_ID")
        .expect("CHANNEL_ID must be set")
        .parse()
        .expect("CHANNEL_ID must be a valid number"));

    let discord = Arc::new(serenity::all::Http::new(&token));

    let channel = channel_id.to_channel(&discord).await
    .map_err(|e| error!("Failed to get channel: {}", e))
    .unwrap();

    let guild: PartialGuild = discord.get_guild(channel.clone().guild().unwrap().guild_id).await.unwrap();

    let guild_channel = channel.clone().guild().unwrap();

    let state = Arc::new(AppState {
        discord,
        guild,
        channel,
        guild_channel
    });

    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    info!("Server running on port 4000");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AlertPayload>,
) -> &'static str {
    // debug!("Received payload {:#?}", payload);

    let alerts = payload.alerts.unwrap_or_default();
    let status = payload.status.unwrap_or("unknown".to_string());

    for alert in alerts {
        // debug!("Alert {:#?}", alert);
        let thread_name = format!("{} - {}",
            alert.labels
                .as_ref()
                .and_then(|l| l.alertname.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("unknown"),
            alert.labels
                .as_ref()
                .and_then(|l| l.namespace.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("unknown")
        );
        debug!("Thread's name {}", thread_name);
        handle_thread(&state, &thread_name, &alert, alert.status.as_deref().unwrap_or(&status)).await;
    }

    "OK"
}

async fn handle_thread(state: &AppState, thread_name: &str, alert: &Alert, status: &str) {
    let guild = &state.guild;
    let channel = &state.channel;
    let guild_channel = &state.guild_channel;

    let mut threads = guild.get_active_threads(&state.discord).await.unwrap().threads;
    let mut inactive_public_threads = channel.id().get_archived_public_threads(&state.discord, None, None).await.unwrap().threads;
    threads.append(&mut inactive_public_threads);
    let all_threads = threads.to_vec();
    let mut channel_threads = all_threads.iter().filter(|thread| thread.parent_id.unwrap() == guild_channel.id);

    let filtered_thread = channel_threads
        .find(|thread| thread.name() == thread_name );

    if filtered_thread.is_some() {
        debug!("Found name {}=={}", thread_name, filtered_thread.clone().unwrap().name);
    } else {
        debug!("Didn't fount name {}", thread_name);
    }

    let message = format!("**Title:** {}\n**Status:** {}\n**Description:** {}",
        thread_name,
        status,
        alert.annotations
            .as_ref()
            .and_then(|a| a.description.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("No description provided")
    );

    match filtered_thread {
        Some(thread) => {
            match status {
                "resolved" => {
                    thread.clone().edit_thread(&state.discord, EditThread::new().archived(true)).await
                        .map_err(|e| error!("Failed to archive thread: {}", e))
                        .unwrap();
                },
                _ => {
                    if thread.clone().thread_metadata.unwrap().archived {
                        let mut t = thread.clone();
                        t.edit_thread(&state.discord, EditThread::new().archived(true)).await
                            .map_err(|e| error!("Failed to archive thread: {}", e))
                            .unwrap();
                    };
                    let msg = CreateMessage::new().content(message);
                    thread.send_message(&state.discord, msg).await
                        .map_err(|e| error!("Failed to send message: {}", e))
                        .unwrap();
                }
            };
        }
        _ => {
            match guild_channel.kind {
                ChannelType::Forum => {
                    guild_channel.create_forum_post(
                        &state.discord,
                        CreateForumPost::new(thread_name, CreateMessage::new().content(message))
                    ).await
                        .map_err(|e| error!("Failed to create forum post: {}", e))
                        .unwrap();
                },
                _ => {
                    let new_message = guild_channel.send_message(
                        &state.discord,
                        CreateMessage::new().content(format!("**Title:** {}", thread_name))
                    ).await
                        .map_err(|e| error!("Failed to send initial message: {}", e))
                        .unwrap();

                    let thread = guild_channel.create_thread_from_message(
                        &state.discord,
                        new_message,
                        CreateThread::new(thread_name)
                    ).await
                        .map_err(|e| error!("Failed to create thread: {}", e))
                        .unwrap();

                    thread.send_message(&state.discord, CreateMessage::new().content(message)).await
                        .map_err(|e| error!("Failed to send thread message: {}", e))
                        .unwrap();
                }
            };
        }
    };
}
