use anyhow::anyhow;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{
        id::UserId,
        prelude::{ReactionType, User},
    },
    utils::Colour,
};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Print the leaderboard"#]
#[usage("{how much score do you want}")]
#[example("")]
#[example("20")]
pub async fn leaderboard(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let scores = crate::airtable::get_scores().await?;

    let mut points = Vec::<(Option<User>, isize)>::new();

    for score in scores.iter() {
        points.push((
            UserId::from(score.id.parse::<u64>()?)
                .to_user(&ctx)
                .await
                .ok(),
            score.score,
        ));
    }
    points.sort_by_key(|(_user, score)| *score);
    points.reverse();
    let turbo_string = points
        .iter()
        .enumerate()
        .take(args.single::<usize>().unwrap_or(10))
        .fold(String::new(), |mut acc, (position, (user, score))| {
            if let Some(user) = user {
                acc.push_str(&format!(
                    "{}) {} – **{}** points\n",
                    position + 1,
                    user,
                    score
                ));
            } else {
                acc.push_str(&format!(
                    "{}) {} – **{}** points\n",
                    position + 1,
                    "Unknown",
                    score
                ));
            }
            acc
        });

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("LeaderBoard")
                    .description(turbo_string)
                    .thumbnail(msg.author.face())
                    .colour(Colour::from_rgb(
                        *msg.author.id.as_u64() as u8,
                        (*msg.author.id.as_u64() >> 8) as u8,
                        (*msg.author.id.as_u64() >> 16) as u8,
                    ))
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description = r#"Get the score of a specific user. If no user is specified it returns your score"#]
#[usage("{user}")]
#[example("")]
#[example("@Tamo")]
pub async fn score(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user: UserId = args.single().unwrap_or_else(|_| msg.author.id);
    let user = user.to_user(&ctx).await?;
    let author = &msg.author;

    let scores = crate::airtable::get_scores().await?;
    let score = scores
        .iter()
        .find(|score| score.id == user.id.as_u64().to_string())
        .map(|score| score.score)
        .unwrap_or(0);
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title(&user.name)
                    .thumbnail(user.face())
                    .field("Balance", format!("{} points", score), false)
                    .footer(|f| f.icon_url(author.face()).text(author.name.clone()))
                    .colour(Colour::from_rgb(
                        *author.id.as_u64() as u8,
                        (*author.id.as_u64() >> 8) as u8,
                        (*author.id.as_u64() >> 16) as u8,
                    ))
            })
        })
        .await?;
    Ok(())
}

#[command]
#[description = r#"Set the score of any user on the server. You must be a crème to run this command"#]
#[usage("{user} {score}")]
#[example("@Tamo 500")]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let has_role = msg
        .author
        .has_role(
            &ctx,
            msg.guild_id.expect("called in wrong channel"),
            crate::ROLE_CREMEUX,
        )
        .await
        .unwrap_or(false)
        || msg
            .author
            .has_role(&ctx, msg.guild_id.unwrap(), crate::ROLE_CREMISSIME)
            .await
            .unwrap_or(false);

    if !has_role {
        Err(anyhow!(
            "You do not have the right to use this command. It is reserved to Crémeux's members"
        ))?;
    }

    let user: UserId = args
        .single()
        .map_err(|_e| anyhow!("The first argument must be a valid user"))?;
    let score: isize = args
        .single()
        .map_err(|_e| anyhow!("The score must be an integer"))?;

    crate::airtable::update_score(
        &user
            .to_user(&ctx)
            .await
            .map_err(|e| anyhow!("Internal error: {}", e))?,
        |_| score,
    )
    .await?;
    msg.react(&ctx, "👍".parse::<ReactionType>().unwrap())
        .await?;
    Ok(())
}

#[command]
#[description = r#"Add a specific number of points to the score of any user on the server. You must be a crème to run this command"#]
#[usage("{user} {score}")]
#[example("@Tamo 500")]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let has_role = msg
        .author
        .has_role(
            &ctx,
            msg.guild_id.expect("called in wrong channel"),
            crate::ROLE_CREMEUX,
        )
        .await
        .unwrap_or(false)
        || msg
            .author
            .has_role(&ctx, msg.guild_id.unwrap(), crate::ROLE_CREMISSIME)
            .await
            .unwrap_or(false);

    if !has_role {
        Err(anyhow!(
            "You do not have the right to use this command. It is reserved to Crémeux's members"
        ))?;
    }

    let user: UserId = args
        .single()
        .map_err(|_e| anyhow!("The first argument must be a valid user"))?;
    let score: isize = args
        .single()
        .map_err(|_e| anyhow!("The score must be an integer"))?;

    crate::airtable::update_score(
        &user
            .to_user(&ctx)
            .await
            .map_err(|e| anyhow!("Internal error: {}", e))?,
        |current| current + score,
    )
    .await?;
    msg.react(&ctx, "👍".parse::<ReactionType>().unwrap())
        .await?;
    Ok(())
}
