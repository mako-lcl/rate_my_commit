extern crate dotenv;

use clap::Parser;
use dotenv::dotenv;
use rate_my_commit::{
    Args, Bot, CHANNEL_ID, CHAT_GPT_API_KEY, DISCORD_TOKEN, GPT_MODEL, GUILD_ID, NO_CHANNEL_ID,
    NO_CHAT_GPT_API_KEY, NO_DISCORD_TOKEN, NO_GPT_MODEL, NO_GUILD_ID,
};
use std::env;

#[tokio::main]
async fn main() {
    // region: --- Env
    // -- CLI
    let args = Args::parse();
    if args.env {
        // -- Init dotenv
        dotenv().ok();
    }

    // -- Check env
    let gpt_api_key = env::var(CHAT_GPT_API_KEY).expect(NO_CHAT_GPT_API_KEY);
    println!("Found gpt api key: {}", gpt_api_key);
    let gpt_model = env::var(GPT_MODEL).expect(NO_GPT_MODEL);
    println!("Found gpt model: {}", gpt_model);
    let discord_token = env::var(DISCORD_TOKEN).expect(NO_DISCORD_TOKEN);
    println!("Found discord token: {}", discord_token);
    let discord_guild_id = env::var(GUILD_ID).expect(NO_GUILD_ID);
    println!("Found guild id: {}", discord_guild_id);
    let discord_channel_id = env::var(CHANNEL_ID).expect(NO_CHANNEL_ID);
    println!("Found channel id: {}", discord_channel_id);
    // endregion: --- Env

    // region: --- Bot
    let bot = Bot::new(discord_token).await;

    bot.start().await;
    // endregion: --- Bot
}
