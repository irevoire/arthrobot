use serenity::{client::Context, model::channel::{Reaction, ReactionType}};


pub async fn handle_reaction(ctx: Context, reaction: Reaction) -> anyhow::Result<()> {
    use ReactionType::*;
    let points = match reaction.emoji {
        Unicode(ref e) if e == "🥇" => 100_isize,
        Unicode(ref e) if e == "🥈" => 50,
        Unicode(ref e) if e == "🥉" => 10,
        _ => return Ok(()),
    };

    if let Ok(user) = reaction.user(&ctx).await {
        if user.has_role(&ctx, reaction.guild_id.unwrap(), 739887430345162862).await.unwrap() {
            let winner = reaction.message(&ctx).await.unwrap().author;

            let data = ctx.data.read().await;
            let mut score = data.get::<crate::score::Score>().unwrap().lock().await;
            *score.entry(winner.id).or_insert(0) += points;
            println!("{} gave {} points to {}", user.name, points, winner.name);
        }
    }

    Ok(())

}
