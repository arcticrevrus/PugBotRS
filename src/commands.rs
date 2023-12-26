use poise::serenity_prelude::{self as serenity};
use crate::functions::{*};



#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Add to queue"] role: Option<String>,
) -> Result<(), Error> {
    let data = ctx.data();
    let listen_channel_id = *data.listen_channel_id.lock().await; // Lock and dereference the channel ID
    if ctx.channel_id() ==  listen_channel_id {
        if let Some(role_str) = role {
            let role_enum = match role_str.as_str() {
                "Tank" => Roles::Tank,
                "Healer" => Roles::Healer,
                "DPS" => Roles::DPS,
                _ => {
                    ctx.say("Invalid role. Please choose Tank, Healer, or DPS.").await?;
                    return Ok(());
                }
            };
        
            let player = Player {
                name: ctx.author().clone(),
                role: role_enum,
            };
        
        
            match player.role {
                Roles::Tank => {
                    let mut queue = ctx.data().tank_queue.lock().await;
                    queue.push(player);
                },
                Roles::Healer => {
                    let mut queue = ctx.data().healer_queue.lock().await;
                    queue.push(player);
                },
                Roles::DPS => {
                    let mut queue = ctx.data().dps_queue.lock().await;
                    queue.push(player);
                },
            }
            ctx.say(format!("Player {} added to the queue as {}.", ctx.author().name, role_str)).await?;
            queue_check(ctx).await?;
        } else {

        let response = ctx.send(|m| {
            m.content("Click a button to join the queue.")
            .ephemeral(true)
            .components(|c| {
                c.create_action_row(|row| {
                    row.create_button(|button| {
                        button
                            .style(serenity::ButtonStyle::Primary)
                            .label("Tank")
                            .custom_id("add_tank")
                    });
                    row.create_button(|button| {
                        button
                            .style(serenity::ButtonStyle::Success)
                            .label("Healer")
                            .custom_id("add_healer")
                    });
                    row.create_button(|button| {
                        button
                            .style(serenity::ButtonStyle::Danger)
                            .label("DPS")
                            .custom_id("add_dps")
                    })
                })
            })
        }).await?;

        let message = response.message().await?;

        if let Some(interaction) = &message
            .await_component_interaction(ctx.serenity_context())
            .timeout(std::time::Duration::from_secs(60))
            .await
        {
            if interaction.data.custom_id == "add_tank" {
                interaction.create_interaction_response(ctx.serenity_context(), |response| {
                    response
                        .kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Added to queue as Tank!").ephemeral(true))
                })
                .await?;
                let player = Player {
                    name: ctx.author().clone(),
                    role: Roles::Tank,
                };
                let mut queue = ctx.data().tank_queue.lock().await;
                queue.push(player);
            } else if interaction.data.custom_id == "add_healer" {
                interaction.create_interaction_response(ctx.serenity_context(), |response| {
                    response
                        .kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Added to queue as Healer!").ephemeral(true))
                })
                .await?;
                let player = Player {
                    name: ctx.author().clone(),
                    role: Roles::Healer,
                };
                let mut queue = ctx.data().healer_queue.lock().await;
                queue.push(player);
            } else if interaction.data.custom_id == "add_dps" {
                interaction.create_interaction_response(ctx.serenity_context(), |response| {
                    response
                        .kind(serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Added to queue as DPS!").ephemeral(true))
                })
                .await?;
                let player = Player {
                    name: ctx.author().clone(),
                    role: Roles::DPS,
                };
                let mut queue = ctx.data().dps_queue.lock().await;
                queue.push(player);

            }
        }
    }
    } else {
        // Attempt to fetch the channel name
        let channel_name = match ctx.serenity_context().http.get_channel(listen_channel_id.0).await {
            Ok(channel) => channel.guild().unwrap().name, // Assuming it's a guild channel
            Err(_) => listen_channel_id.to_string(), // Fallback to channel ID if name cannot be fetched
        };
        ctx.send(|m| {
            m.content(format!("Commands must be sent in #{}", channel_name))
            .ephemeral(true)
        })
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>
) -> Result<(), Error> {
    remove_player_from_queue(ctx).await;
    ctx.say("You have been removed from all queues.").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn queue(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say(print_current_queue(ctx).await).await?;
    Ok(())
}

