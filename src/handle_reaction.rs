use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::{
        channel::{Reaction, ReactionType},
        id::ChannelId,
    },
    utils::Colour,
};

pub async fn handle_reaction(ctx: Context, reaction: Reaction) -> CommandResult {
    use ReactionType::*;
    let points = match reaction.emoji {
        Unicode(ref e) if e == "🥇" => 60_isize,
        Unicode(ref e) if e == "🥈" => 30,
        Unicode(ref e) if e == "🥉" => 10,
        Unicode(ref e) if e == "🎖" => 210,
        _ => return Ok(()),
    };

    if let Ok(user) = reaction.user(&ctx).await {
        if user
            .has_role(
                &ctx,
                reaction.guild_id.expect("called in wrong channel"),
                crate::ROLE_CREMEUX,
            )
            .await
            .unwrap()
            || user
                .has_role(&ctx, reaction.guild_id.unwrap(), crate::ROLE_CREMISSIME)
                .await
                .unwrap()
        {
            let winner = reaction.message(&ctx).await?.author;

            let res = crate::airtable::update_score(&winner, |score| score + points).await?;

            let channel: ChannelId = crate::SALON_BOT.into();
            channel
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Transaction Complete")
                            .description(format!("{} added {} points to {}", user, points, winner))
                            .field(
                                format!("{}'s Balance", winner.name),
                                format!("{} points", res),
                                false,
                            )
                            .thumbnail(winner.face())
                            .footer(|f| f.icon_url(user.face()).text(user.name))
                            .colour(Colour::from_rgb(
                                *winner.id.as_u64() as u8,
                                (*winner.id.as_u64() >> 8) as u8,
                                (*winner.id.as_u64() >> 16) as u8,
                            ))
                    })
                })
                .await?;
        }
    }

    Ok(())
}
