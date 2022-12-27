use macroquad::prelude::*;
use macroquad::Window;
use std::thread;
pub fn open_window() {
    thread::spawn(|| {
        Window::from_config(
            Conf {
                sample_count: 4,
                window_title: "Dust".to_string(),
                high_dpi: true,
                ..Default::default()
            },
            solve_captcha(),
        );
    });
}

async fn solve_captcha() {
    let mut gif_animation = quad_gif::GifAnimation::load("test.gif".to_string()).await;
    let mut text: String = String::new();
    clear_background(BLACK);
    let mut word_length: f32 = 0.0;
    let fontsize:f32 = 48.0;
    while text.len() <= 5 {
        if is_key_pressed(KeyCode::Backspace) && !text.is_empty(){
            text.pop();
            word_length -= fontsize/2.1;
        }
        let key = get_char_pressed().unwrap_or('%').to_ascii_uppercase();
        if key != '%' && key.is_alphanumeric(){
            text.push(key);
            word_length += fontsize/2.1;
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
    // text
}
