use serenity::model::prelude::*;
use std::sync::Arc;
use std::env;
use log::error;

pub struct AppState {
    pub discord: Arc<serenity::all::Http>,
    pub guild: PartialGuild,
    pub channel: Channel,
    pub guild_channel: GuildChannel,
}

pub async fn initialize_discord() -> Arc<AppState> {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let channel_id = ChannelId::new(
        env::var("CHANNEL_ID")
            .expect("CHANNEL_ID must be set")
            .parse()
            .expect("CHANNEL_ID must be a valid number")
    );

    let discord = Arc::new(serenity::all::Http::new(&token));

    let channel = channel_id.to_channel(&discord).await
        .map_err(|e| error!("Failed to get channel: {}", e))
        .unwrap();

    let guild = discord.get_guild(channel.clone().guild().unwrap().guild_id).await.unwrap();
    let guild_channel = channel.clone().guild().unwrap();

    Arc::new(AppState {
        discord,
        guild,
        channel,
        guild_channel
    })
}
