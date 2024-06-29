use serenity::all::EventHandler;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::consts::{BOT_RESPONSE_EMOJI, BOT_RESPONSE_RATING};
use crate::message::verify_guild_and_channel;

use super::Bot;

#[async_trait]
impl EventHandler for Bot {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message author is the bot itself
        let bot_id = ctx.cache.current_user().id;

        // -- Ignore messages from the bot itself
        if msg.author.id == bot_id {
            return;
        }

        // -- Only respond to messages in the correct guild and channel
        if verify_guild_and_channel(msg.guild_id, msg.channel_id) {
            // -- Fetch commit message

            // Check if the message has embeds
            if let Some(embed) = msg.embeds.first() {
                // Use if let to check and get the description of the first embed
                if let Some(commit_message) = &embed.description {
                    println!("Embed description: {}", commit_message);

                    // -- Rate commit message
                    match self.commit_rater.rate_commit(commit_message.clone()).await {
                        Ok(rating) => {
                            {
                                // -- Create response message
                                let response_msg = format!(
                                    "{}\n{}{}{}{}",
                                    rating.comment,
                                    BOT_RESPONSE_RATING,
                                    rating.rating,
                                    BOT_RESPONSE_EMOJI,
                                    rating.emoji
                                );

                                // -- Send message
                                let cloned = response_msg.clone();
                                if let Err(why) = msg.reply(&ctx.http, response_msg).await {
                                    println!("Error sending message. Content: {cloned:?}\nError: {why:?}");
                                }

                                // -- Get emoji as char
                                if let Some(emoji) = rating.emoji.chars().next() {
                                    // -- React to message
                                    if let Err(why) = msg.react(ctx, emoji).await {
                                        println!(
                                            "Error sending emoji. Content: {cloned:?}\nError: {why:?}"
                                        );
                                    }
                                }
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
