// region: --- Environment
pub const CHAT_GPT_API_KEY: &str = "CHAT_GPT_API_KEY";
pub const GPT_MODEL: &str = "GPT_MODEL";
pub const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
pub const GUILD_ID: &str = "GUILD_ID";
pub const CHANNEL_ID: &str = "CHANNEL_ID";
// endregion: --- Environment

// region: --- CLI
pub const CLI_NAME: &str = "rate-my-commit";
pub const CLI_VERSION: &str = "1.0";
pub const CLI_AUTHOR: &str = "Ludwig Chen Li";
pub const CLI_ABOUT: &str = "Uses GPT to rate git commits";
// endregion: --- CLI

// region: --- Error Messages
pub const _NO: &str = " not found";
pub const NO_CHAT_GPT_API_KEY: &str = "ChatGPT api key not found";
pub const NO_GPT_MODEL: &str = "gpt model not found";
pub const NO_DISCORD_TOKEN: &str = "Discord bot token not found";
pub const NO_GUILD_ID: &str = "Guild id not found";
pub const NO_CHANNEL_ID: &str = "Channel id not found";
pub const SERENITY_CREATE_CLIENT_FAILED: &str = "Error creating serenity client";
// endregion: --- Error Messages

// region: --- Headers
pub const HEADER_AUTHORIZATION: &str = "Authorization";
pub const HEADER_BEARER: &str = "Bearer ";
// endregion: --- Headers

// region: --- GPT
pub const GPT_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const GPT_MSG_ROLE_USER: &str = "user";
pub const COMPLETION_PROMPT_TEMPLATE: &str = "
Rate the following git commit message on a scale from 1 to 10 and give it an emoji.
A commit message that is vague, not very verbose and has little clarity should be a 1, while a message that is very clear and expressive should be a 10.
Also briefly comment on what you think about the commit without talking about the author.
Please only use unicode emojis.
Return a json with fields \"rating\" and \"emoji\" and \"comment\".
Message:

";
// endregion: --- GPT

// region: --- Discord
pub const BOT_RESPONSE_RATING: &str = "I give this commit a rating of ";
pub const BOT_RESPONSE_EMOJI: &str = " and a ";
// endregion: --- Discord
