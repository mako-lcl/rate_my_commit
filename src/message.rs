use std::env;

use serenity::all::{ChannelId, GuildId};

use crate::consts::{CHANNEL_ID, GUILD_ID, NO_CHANNEL_ID, NO_GUILD_ID};

pub fn verify_guild_and_channel(guild_id: Option<GuildId>, channel_id: ChannelId) -> bool {
    // -- Get guild and channel id from message
    let guild_id = match guild_id {
        Some(id) => id.to_string(),
        None => return false,
    };
    let channel_id = channel_id.to_string();

    // -- Get channel and guild from env and compare
    let env_guild_id = env::var(GUILD_ID).expect(NO_GUILD_ID);
    let env_channel_id = env::var(CHANNEL_ID).expect(NO_CHANNEL_ID);

    if guild_id != env_guild_id || channel_id != env_channel_id {
        return false;
    }

    true
}
