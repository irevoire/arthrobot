use serenity::{framework::standard::{macros::command, Args, CommandResult}, model::prelude::User, utils::Colour};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Print the leaderboard"#]
#[usage("{how much score do you want}")]
#[example("")]
#[example("20")]
pub async fn leaderboard(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let score = data
        .get::<crate::score::Score>()
        .unwrap()
        .lock()
        .await
        .clone();

    let mut points = Vec::<(User, isize)>::new();

    for (id, score) in score.iter() {
        points.push((id.to_user(&ctx).await?, *score));
    }
    points.sort_by_key(|(_user, score)| score.clone());

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title("LeaderBoard")
                .fields(
                    points
                        .iter()
                        .take(10)
                        .enumerate()
                        .map(|(position, (user, score))| {
                            (
                                format!("{}) ", position + 1),
                                format!("{} – **{}** points", user, score),
                                false,
                            )
                        }),
                )
                .thumbnail(msg.author.face())
                .colour(Colour::DARK_ORANGE)
        })
    }).await?;

    Ok(())
}
