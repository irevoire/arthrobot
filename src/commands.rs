use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Print the leaderboard"#]
#[usage("{how much score do you want}")]
#[example("")]
#[example("20")]
pub async fn leaderboard(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    msg.reply(&ctx, "hello").await?;
    Ok(())
}

