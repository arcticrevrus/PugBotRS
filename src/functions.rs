use std::sync::Arc;
use tokio::sync::Mutex;
use poise::serenity_prelude::{self as serenity, MessageComponentInteraction};
use crate::serenity::ChannelId;

pub struct Data {
    pub tank_queue: Arc<Mutex<Vec<Player>>>,
    pub healer_queue: Arc<Mutex<Vec<Player>>>,
    pub dps_queue: Arc<Mutex<Vec<Player>>>,
    pub listen_channel_id: Arc<Mutex<ChannelId>>
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Roles {
    Tank,
    Healer,
    DPS,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: serenity::User,
    pub role: Roles,
}

pub async fn channel_check(ctx: Context<'_>) -> bool {
    let data = ctx.data();
    let listen_channel_id = *data.listen_channel_id.lock().await; // Lock and dereference the channel ID
    if ctx.channel_id() == listen_channel_id {
        return true
    } else {
        let channel_name = match ctx.serenity_context().http.get_channel(listen_channel_id.0).await {
            Ok(channel) => channel.guild().unwrap().name, // Assuming it's a guild channel
            Err(_) => listen_channel_id.to_string(), // Fallback to channel ID if name cannot be fetched
        };
        ctx.send(|m| {
            m.content(format!("This command must be sent in #{}", channel_name))
            .ephemeral(true)
        }).await.unwrap();
        return false
    }
}

pub async fn create_ephemeral_response(ctx: Context<'_>, interaction: &Arc<MessageComponentInteraction>, input_message: String) -> Result<(), Error> {
    let content = interaction.create_interaction_response(ctx.serenity_context(), |response|{
        response
            .kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(input_message).ephemeral(true))
    }).await?;
    Ok(content)
}

pub async fn queue_check(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let tank_queue = data.tank_queue.lock().await;
    let healer_queue = data.healer_queue.lock().await;
    let dps_queue = data.dps_queue.lock().await;

    if tank_queue.len() >= 1 && healer_queue.len() >= 1 && dps_queue.len() >= 3 {
        let mut game_found: String = "Game found! The players are: ".to_owned();
        game_found.push_str(&add_players_to_game_found(tank_queue, healer_queue, dps_queue));
        ctx.say(game_found.trim_end_matches(", ")).await?;
    }
    Ok(())
}

fn add_players_to_game_found(
    tank_queue: tokio::sync::MutexGuard<'_, Vec<Player>>,
    healer_queue: tokio::sync::MutexGuard<'_, Vec<Player>>,
    dps_queue: tokio::sync::MutexGuard<'_, Vec<Player>>
 ) -> String {
    let mut final_queue = String::new();
    final_queue.push_str(&add_tank_to_game_found(tank_queue));
    final_queue.push_str(&add_healer_to_game_found(healer_queue));
    final_queue.push_str(&add_dps_to_game_found(dps_queue));
    return final_queue
}



fn add_tank_to_game_found(mut tank_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let Some(tank) = &tank_queue.pop() else { return "Error adding tank to queue".to_owned() };
    let mut tank_string = String::new();
    tank_string.push_str(&format_game_found_output(tank));
    return tank_string
}

fn add_healer_to_game_found(mut healer_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let Some(healer) = &healer_queue.pop() else { return "Error adding healer to queue".to_owned() };
    let mut healer_string = String::new();
    healer_string.push_str(&format_game_found_output(healer));
    return healer_string
}

fn add_dps_to_game_found(mut dps_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let mut dps_string = String::new();
    for _ in 0 .. 3 {
        let Some(dps) = &dps_queue.pop() else { return "Error adding healer to queue".to_owned() };
            dps_string.push_str(&format_game_found_output(dps))
    }
    return dps_string
}

fn format_game_found_output(player: &Player) -> String {
    let mut player_string = String::new();
    player_string.push_str("<@");
    player_string.push_str(&player.name.id.to_string());
    player_string.push_str(">, ");
    return player_string
}

pub async fn remove_player_from_queue(ctx: Context<'_>) {
    let data = ctx.data();
    let mut tank_queue = data.tank_queue.lock().await;
    let mut healer_queue = data.healer_queue.lock().await;
    let mut dps_queue = data.dps_queue.lock().await;

    tank_queue.retain(|p| p.name.id != ctx.author().id);
    healer_queue.retain(|p| p.name.id != ctx.author().id);
    dps_queue.retain(|p| p.name.id != ctx.author().id);
}

pub async fn print_current_queue(ctx: Context<'_>) -> String {
    let data = ctx.data();
    let tank_queue = data.tank_queue.lock().await;
    let healer_queue = data.healer_queue.lock().await;
    let dps_queue = data.dps_queue.lock().await;

    let message = format!(
        " The current Queue is: 
        >>> <:tank:444634700523241512> : {}
<:heal:444634700363857921> : {}
<:dps:444634700531630094> :  {}",
        concat_queue(tank_queue).trim_end_matches(", "),
        concat_queue(healer_queue).trim_end_matches(", "),
        concat_queue(dps_queue).trim_end_matches(", ")
        );
    return message
}

pub fn concat_queue(queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let mut queue_cat = String::new();
    for player in queue.iter() {
        queue_cat.push_str(&{format!("{}, ", &player.name.name)});
    }
    return queue_cat
}