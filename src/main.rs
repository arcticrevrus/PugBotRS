use std::sync::Arc;
use tokio::sync::Mutex;
use poise::serenity_prelude::{self as serenity};
use crate::serenity::ChannelId;
use crate::commands::{*};
mod functions;
mod commands;

#[tokio::main]
async fn main() {
    let bot_commands = vec![add(), remove(), queue()];
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let discord_intents = serenity::GatewayIntents::non_privileged();
    let initial_data = functions::Data {
        tank_queue: Arc::new(Mutex::new(Vec::new())),
        healer_queue: Arc::new(Mutex::new(Vec::new())),
        dps_queue: Arc::new(Mutex::new(Vec::new())),
        listen_channel_id: Arc::new(Mutex::new(ChannelId(1188297258362482728)))
        //listen_channel_id: Arc::new(Mutex::new(ChannelId(1074778083753742357)))
    };

    let bot_client = poise::Framework::builder()
        .options(poise::FrameworkOptions{commands: bot_commands,
            event_handler: |ctx, event, framework, data| {
                Box::pin(functions::event_handler(ctx, event, framework, data))
            },
            ..Default::default()})
        .token(token)
        .intents(discord_intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move{
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(initial_data)
            })
        });
        

    bot_client.run().await.unwrap();
}

