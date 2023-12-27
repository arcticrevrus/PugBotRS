use poise::serenity_prelude::{self as serenity};
use crate::functions::{*};



#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Add to queue"] role: Option<String>,
) -> Result<(), Error> {
    if channel_check(ctx).await {
        if let Some(role_) = role {
            let player = create_player(ctx, role_).await;
            match player {
                Ok(player) => {
                    push_to_queue(ctx, player).await?;
                },
                Err(e) => {ctx.say(format!("{}",e)).await?;}
            }
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
                    create_ephemeral_response(ctx, "Added to Queue as Tank!".to_owned()).await?;
                    let player = create_player(ctx, "tank".to_owned()).await;
                    match player {
                        Ok(player) => {
                            push_to_queue(ctx, player).await?;
                        },
                        Err(e) => {ctx.say(format!("{}",e)).await.unwrap();}
                    }
                } else if interaction.data.custom_id == "add_healer" {
                    create_ephemeral_response(ctx, "Added to Queue as Healer!".to_owned()).await?;
                    let player = create_player(ctx, "healer".to_owned()).await;
                    match player {
                        Ok(player) => {
                            push_to_queue(ctx, player).await?;
                            queue_check(ctx).await.unwrap();
                        },
                        Err(e) => {ctx.say(format!("{}",e)).await.unwrap();}
                    }
                } else if interaction.data.custom_id == "add_dps" {
                    create_ephemeral_response(ctx, "Added to Queue as DPS!".to_owned()).await?;
                    let player = create_player(ctx, "dps".to_owned()).await;
                    match player {
                        Ok(player) => {
                            push_to_queue(ctx, player).await?;
                        },
                        Err(e) => {ctx.say(format!("{}",e)).await.unwrap();}
                    }
                }
            }
        }
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
    ctx.say(print_current_queue(ctx).await).await.unwrap();
    Ok(())
}