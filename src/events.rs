use crate::Error;
use poise::serenity_prelude as serenity;
use regex::Regex;
use time;


pub async fn borger(ctx: &serenity::Context, msg: &serenity::Message, m: &str) -> Result<(), Error> {
  if Regex::new(r"(?i)\S*b[ou]rger\b").unwrap().is_match(m) {
    let time = time::OffsetDateTime::now_utc()
      .to_offset(time::UtcOffset::from_hms(-5, 0, 0)?)
      .format(&time::format_description::parse("[hour repr:12 padding:none]:[minute] [period case:upper]")?)?;
    msg.reply(&ctx.http, format!("It is now {} in Borger, Texas.", time)).await?;
  }
  Ok(())
}