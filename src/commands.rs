use poise::serenity_prelude::{self as serenity};
use crate::functions::{*};
use crate::serenity::ButtonStyle::{*};


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
            let message = "Click a button to join the queue.".to_owned();
            let tank_button = Button {
                style: Primary,
                label: "Tank".to_owned(),
                id: "add_tank".to_owned()
            };
            let healer_button = Button {
                style: Success,
                label: "Healer".to_owned(),
                id: "add_healer".to_owned()
            };
            let dps_button = Button {
                style: Danger,
                label: "DPS".to_owned(),
                id: "add_dps".to_owned()
            };
            let components = vec![tank_button, healer_button, dps_button];
            create_ephemeral_response(ctx, message, Some(components)).await?;
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