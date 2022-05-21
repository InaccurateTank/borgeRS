use crate::Error;
use poise::serenity_prelude as serenity;
use regex::Regex;
use time;
use rand::prelude::*;


pub async fn borger(ctx: &serenity::Context, msg: &serenity::Message, m: &str) -> Result<(), Error> {
  if Regex::new(r"(?i)\S*b[ou]rger\b").unwrap().is_match(m) {
    let time = time::OffsetDateTime::now_utc()
      .to_offset(time::UtcOffset::from_hms(-5, 0, 0)?)
      .format(&time::format_description::parse("[hour repr:12 padding:none]:[minute] [period case:upper]")?)?;
    msg.reply(&ctx.http, format!("It is now {} in Borger, Texas.", time)).await?;
  }
  Ok(())
}

pub async fn v(ctx: &serenity::Context, msg: &serenity::Message, m: &str) -> Result<(), Error> {
  if Regex::new(r"(?i)\S*vore\b").unwrap().is_match(m) {
    let rng: u8 = rand::thread_rng().gen_range(0..1);
    let mut img: &str = "";
    match rng {
      0 => img = "https://i.imgur.com/59urJXr.png",
      1 => img = "https://i.imgur.com/59urJXr.png",
      _ => println!("Rand Fell Through?")
    }
    msg.reply(&ctx.http, img).await?;
  }
  Ok(())
}