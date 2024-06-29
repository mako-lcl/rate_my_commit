use serenity::{all::GatewayIntents, Client};

use crate::{consts::SERENITY_CREATE_CLIENT_FAILED, rating::CommitRater};

pub mod handler;

pub struct Bot {
    pub discord_token: String,
    pub commit_rater: CommitRater,
}

impl Bot {
    pub async fn new(discord_token: String) -> Self {
        let commit_rater = CommitRater::default();

        Bot {
            commit_rater,
            discord_token,
        }
    }

    pub async fn start(self) {
        // Set gateway intents, which decides what events the bot will be notified about
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        // Create a new instance of the Client, logging in as a bot. This will automatically prepend
        // your bot token with "Bot ", which is a requirement by Discord for bot users.
        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(self)
            .await
            .expect(SERENITY_CREATE_CLIENT_FAILED);

        // Finally, start a single shard, and start listening to events.
        //
        // Shards will automatically attempt to reconnect, and will perform exponential backoff until
        // it reconnects.
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    }
}
