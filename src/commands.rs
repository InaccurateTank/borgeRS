use crate::{Context, Error};

fn byte_to_emoji(value: u8) -> String {
  let mut buffer = String::new();
  let mut value = value;
  if value == 0 {
    buffer.push_str("❤️");
  }
  loop {
    let (emoji, subtract) = match value {
      200..=255 => ("🫂", 200),
      50..=199 => ("💖", 50),
      10..=49 => ("✨", 10),
      5..=9 => ("🥺", 5),
      1..=4 => (",", 1),
      0 => break,
    };
    buffer.push_str(emoji);
    value -= subtract;
  }
  buffer.push_str("👉👈");
  buffer
}

/// Use your words I don't speak bottom.
#[poise::command(
  slash_command,
  category = "General"
)]
pub async fn bottomify(
  ctx: Context<'_>,
  #[description = "Text to translate"]
  plead: String
) -> Result<(), Error> {
  let result = plead.bytes().map(|t| byte_to_emoji(t)).collect::<String>();
  ctx.send(|c| {
    c.content(result)
  }).await?;
  Ok(())
}