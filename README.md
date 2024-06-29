# Rate my Commit
A simple rust discord bot that uses ChatGPT completion to rate git commits. Returns a rating (1-10), short comment and an emoji. Reads messages in a specified server & channel. 
Commits must be discord message embedded descriptions (setup github webhook for discord channel). Supply environment variables in docker-compose.yml.

# Docker deployment

```
docker compose up -d
```

## Docker-compose.yml environment

```
- CHAT_GPT_API_KEY=
- GPT_MODEL=
- DISCORD_TOKEN=
- GUILD_ID=
- CHANNEL_ID=
```

# Test

```
cargo test -- --nocapture
```

# Local Development
Expand .env environment file
```
cargo run -- --env
```

## Local .env Environment

```
CHAT_GPT_API_KEY=
GPT_MODEL=
DISCORD_TOKEN=
GUILD_ID=
CHANNEL_ID=
```
