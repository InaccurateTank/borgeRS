const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = "Borgerbot";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const REPO: &'static str = env!("CARGO_PKG_REPOSITORY");

use poise::serenity_prelude as serenity;
use std::env;

mod events;
use events::*;
// mod commands;
// use commands::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays command help prompt.
#[poise::command(
  prefix_command,
  track_edits,
  slash_command,
  category = "General"
)]
async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"]
  #[autocomplete = "poise::builtins::autocomplete_command"]
   command: Option<String>,
) -> Result<(), Error> {
  let config = poise::builtins::HelpConfiguration {
    extra_text_at_bottom: "\
Type ~help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
    ..Default::default()
  };
  poise::builtins::help(ctx, command.as_deref(), config).await?;
  Ok(())
}

/// Displays information about the bot.
#[poise::command(
  prefix_command,
  slash_command,
  category = "General",
  ephemeral
)]
async fn about(
  ctx:Context<'_>
) -> Result<(), Error> {
  ctx.send(|b| {
    b.content("")
    .embed(|e|{
      e.title(format!("Hello I'm {}!", NAME))
        .color(serenity::utils::Color::new(663366))
        .description(DESCRIPTION)
        .field("Author", AUTHORS.replace(":", "\n"), true)
        .field("Version", VERSION, true)
        .field("Repository", REPO, false)
        .footer(|f| {
          f.text("Made with incompetence")
        })
    })
  }).await?;
  Ok(())
}

/// Registers application commands in this guild or globally.
#[poise::command(
  prefix_command,
  hide_in_help,
  category = "General"
)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
  let options = poise::FrameworkOptions {
    commands: vec![
      help(),
      about(),
      register()
    ],
    prefix_options: poise::PrefixFrameworkOptions {
      prefix: Some("~".into()),
      edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(3600))),
      ..Default::default()
    },
    listener: |ctx, event, _framework, _data| {
      Box::pin(async move {
        match event {
          poise::Event::Ready { data_about_bot: ready } => {
            println!("{} is connected!", ready.user.name);
            ctx.set_activity(serenity::Activity::watching("Everything")).await;
          },
          poise::Event::Message { new_message: msg } => {
            if !msg.is_own(&ctx.cache) {
              let m = msg.content.as_str();
              borger(ctx, msg, &m).await?;
            }
          },
          _ => ()
        }
        Ok(())
      })
    },
    ..Default::default()
  };
  // let token = env::var("DISCORD_TOKEN").expect("Expected Enviroment Variable DISCORD_TOKEN");
  let token = "OTc2MzQwMTM3NjMxOTYxMDg4.G3mnlL.x-6VGeXi0WQsnSaMDxsLMiOgHjoCWkIR3z2SPk";
  let intents = serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;
  let framework = poise::Framework::build()
    .options(options)
    .token(token)
    .intents(intents)
    .user_data_setup(move |_ctx, _ready, _framework| {
      Box::pin(async move {
        Ok(Data {})
      })
    });

  framework.run().await.unwrap();
}