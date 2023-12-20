use mysql::{Opts, Pool, OptsBuilder};
pub use poise::serenity_prelude as serenity;
use shitfest_bot::{bot::Data, secrets};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let url = shitfest_bot::secrets::get_db_url();
    let db_name = shitfest_bot::secrets::get_db_name();
    let username = shitfest_bot::secrets::get_db_username();
    let db_password = shitfest_bot::secrets::get_db_password();
    let pool = Pool::new(OptsBuilder::new()
        .ip_or_hostname(Some(url))
        .db_name(Some(db_name))
        .user(Some(username))
        .pass(Some(db_password))).unwrap();

    let token = secrets::get_bot_token();

    let options = poise::FrameworkOptions {
        commands: vec![shitfest_bot::commands::help()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            case_insensitive_commands: true,
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        on_error: |err| Box::pin(shitfest_bot::bot::on_error(err)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Running command `{}`", ctx.command().name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: None,
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Bot ready! Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db: pool })
            })
        })
        .options(options)
        .build();

    let intents = serenity::GatewayIntents::all();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
