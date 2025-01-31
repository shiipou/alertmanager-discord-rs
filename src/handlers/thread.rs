use serenity::builder::*;
use serenity::model::prelude::*;
use log::error;
use crate::discord::AppState;
use crate::models::Alert;

pub async fn handle_thread(state: &AppState, thread_name: &str, alert: &Alert, status: &str) {
    let guild = &state.guild;
    let channel = &state.channel;
    let guild_channel = &state.guild_channel;

    let mut threads = guild.get_active_threads(&state.discord).await.unwrap().threads;
    let mut inactive_public_threads = channel.id().get_archived_public_threads(&state.discord, None, None).await.unwrap().threads;
    threads.append(&mut inactive_public_threads);
    let all_threads = threads.to_vec();
    let mut channel_threads = all_threads.iter().filter(|thread| thread.parent_id.unwrap() == guild_channel.id);

    let filtered_thread = channel_threads
        .find(|thread| thread.name() == thread_name);

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
        Some(thread) => handle_existing_thread(state, thread, status, &message).await,
        None => create_new_thread(state, thread_name, &message).await,
    };
}

async fn handle_existing_thread(state: &AppState, thread: &GuildChannel, status: &str, message: &str) {
    match status {
        "resolved" => {
            thread.clone().edit_thread(&state.discord, EditThread::new().archived(true)).await
                .map_err(|e| error!("Failed to archive thread: {}", e))
                .unwrap();
        },
        _ => {
            if thread.clone().thread_metadata.unwrap().archived {
                thread.clone().edit_thread(&state.discord, EditThread::new().archived(true)).await
                    .map_err(|e| error!("Failed to archive thread: {}", e))
                    .unwrap();
            };
            let msg = CreateMessage::new().content(message);
            thread.send_message(&state.discord, msg).await
                .map_err(|e| error!("Failed to send message: {}", e))
                .unwrap();
        }
    }
}

async fn create_new_thread(state: &AppState, thread_name: &str, message: &str) {
    match state.guild_channel.kind {
        ChannelType::Forum => {
            state.guild_channel.create_forum_post(
                &state.discord,
                CreateForumPost::new(thread_name, CreateMessage::new().content(message))
            ).await
                .map_err(|e| error!("Failed to create forum post: {}", e))
                .unwrap();
        },
        _ => {
            let new_message = state.guild_channel.send_message(
                &state.discord,
                CreateMessage::new().content(format!("**Title:** {}", thread_name))
            ).await
                .map_err(|e| error!("Failed to send initial message: {}", e))
                .unwrap();

            let thread = state.guild_channel.create_thread_from_message(
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
    }
}
