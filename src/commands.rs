// use crate::Error;
// use poise::serenity_prelude as serenity;

/// Registers application commands in this guild or globally.
#[poise::command(
  prefix_command,
  category = "Roles"
)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
}