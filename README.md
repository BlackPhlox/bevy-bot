# Bevy Bot

The sources files for bevy discord bot which is deployed on the bevy [discord]().

Based on [Example 6 from serenity](https://github.com/serenity-rs/serenity/tree/current/examples/e06_sample_bot_structure).

## Discord Bot Development

- Go to the [Discord Developer Portal](https://discord.com/developers/applications), create or login with a discord user.
- Create a new bot, copy the token generated under _Bot_ into a `.env` file (See [`.env.example`](/.env.example)).
- Run the executable by running `cargo run --release`