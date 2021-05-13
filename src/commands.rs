use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::User,
    utils::Colour,
};
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
    let turbo_string = points.iter().enumerate().take(10).fold(
        String::new(),
        |mut acc, (position, (user, score))| {
            acc.push_str(&format!(
                "{}) {} – **{}** points\n",
                position + 1,
                user,
                score
            ));
            acc
        },
    );

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("LeaderBoard")
                    .description(turbo_string)
                    .thumbnail(msg.author.face())
                    .colour(Colour::DARK_ORANGE)
            })
        })
        .await?;

    Ok(())
}
