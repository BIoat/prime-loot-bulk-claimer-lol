use macroquad::prelude::*;
use std::{
    fs,
    io::{self, Read, Write},
};
pub async fn solve_captcha(bytes: Vec<u8>) -> Result<String, ()> {
    let string = String::from_utf8(bytes).unwrap();
    let filename = string.clone();
    let mut gif_animation = quad_gif::GifAnimation::load(string).await;
    let mut text: String = String::new();
    clear_background(BLACK);
    let mut word_length: f32 = 0.0;
    let fontsize: f32 = 48.0;
    while text.len() <= 6 {
        if is_key_pressed(KeyCode::Backspace) && !text.is_empty() {
            text.pop();
            word_length -= fontsize / 2.1;
        }
        let key = get_char_pressed().unwrap_or('%');
        if key != '%' && key.is_alphanumeric() && key.is_ascii() {
            text.push(key);
            word_length += fontsize / 2.1;
        }
        gif_animation.draw_at(screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 35.0);
        draw_text(
            &text,
            screen_width() / 2.0 - (word_length / 2.0),
            screen_height() / 2.0 + 70.0,
            fontsize,
            WHITE,
        );

        gif_animation.tick();
        gif_animation.tick();
        next_frame().await;
    }
    std::fs::remove_file(filename).unwrap();
    Ok(text)
}
fn read_stdin() -> Result<Vec<u8>, ()> {
    let mut buffer = Vec::new();

    if atty::is(atty::Stream::Stdin) {
        println!("no captcha image buffer emitted");
        panic!("no stdin");
    }
    if io::stdin().read_to_end(&mut buffer).is_ok() {}
    Ok(buffer)
}

// #[tokio::main]
#[macroquad::main("Captcha Solver [ manual ]")]
async fn main() -> io::Result<()> {
    let stdi = read_stdin().unwrap();
    let result = solve_captcha(stdi).await;

    let unwrapped = match result {
        Ok(a) => a,
        _ => "error".to_string(),
    };
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(unwrapped.as_bytes())?;

    Ok(())
}
