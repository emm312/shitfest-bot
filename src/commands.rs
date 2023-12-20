use mysql::prelude::Queryable;
use poise::CreateReply;

use crate::bot::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn random_meme(
    ctx: Context<'_>,
    #[description = "Gets a random meme from shitfest.net"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    _command: Option<String>,
) -> Result<(), Error> {
    let mut conn = ctx.data().db.get_conn().unwrap();
    let data: String = conn
        .query_first("SELECT name FROM hashtable WHERE (hidden=0) ORDER BY RAND() LIMIT 1")
        .unwrap()
        .unwrap();
    let msg = format!("https://shitfest.net/{}", data);
    ctx.say(msg).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn index(
    ctx: Context<'_>,
    #[description = "Gets the index of shitfest.net"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    Ok(())
}
