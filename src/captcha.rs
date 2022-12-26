use macroquad::prelude::*;


pub async fn solve_captcha(gif_filename: &str) -> String{
    let mut gif_animation = quad_gif::GifAnimation::load(gif_filename.to_string()).await;
    let mut text: String = String::new();
    clear_background(BLACK);
    let mut word_length: f32 = 0.0;
    let mut lastkey: KeyCode = KeyCode::F1;
    while text.len()<=5{
        if is_key_pressed(KeyCode::Backspace) {
            text.pop();
            word_length-=16.0-2.0;
        }
        if get_last_key_pressed().is_some() && get_last_key_pressed().unwrap() != lastkey {
            lastkey = get_last_key_pressed().unwrap();
            match lastkey {
                KeyCode::Escape => panic!("ESC quits the program"),
                KeyCode::A =>{text.push('A');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::B =>{text.push('B');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::C =>{text.push('C');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::D =>{text.push('D');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::E =>{text.push('E');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::F =>{text.push('F');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::G =>{text.push('G');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::H =>{text.push('H');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::I =>{text.push('I');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::J =>{text.push('J');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::K =>{text.push('K');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::L =>{text.push('L');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::M =>{text.push('M');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::N =>{text.push('N');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::O =>{text.push('O');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::P =>{text.push('P');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::Q =>{text.push('Q');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::R =>{text.push('R');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::S =>{text.push('S');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::T =>{text.push('T');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::U =>{text.push('U');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::V =>{text.push('V');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::W =>{text.push('W');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::X =>{text.push('X');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::Y =>{text.push('Y');word_length+=16.0-2.0}, // YANDERE DEV
                KeyCode::Z =>{text.push('Z');word_length+=16.0-2.0}, // YANDERE DEV

                KeyCode::Key0 =>{text.push('0');word_length+=16.0-2.0},
                KeyCode::Key1 =>{text.push('1');word_length+=16.0-2.0},
                KeyCode::Key2 =>{text.push('2');word_length+=16.0-2.0},
                KeyCode::Key3 =>{text.push('3');word_length+=16.0-2.0},
                KeyCode::Key4 =>{text.push('4');word_length+=16.0-2.0},
                KeyCode::Key5 =>{text.push('5');word_length+=16.0-2.0},
                KeyCode::Key6 =>{text.push('6');word_length+=16.0-2.0},
                KeyCode::Key7 =>{text.push('7');word_length+=16.0-2.0},
                KeyCode::Key8 =>{text.push('8');word_length+=16.0-2.0},
                KeyCode::Key9 =>{text.push('9');word_length+=16.0-2.0},


                _ => (),
            }
        }
        gif_animation.draw_at(screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 35.0);
        draw_text(
            &text,
            screen_width() / 2.0 - (word_length / 2.0),
            screen_height() / 2.0 + 70.0,
            32.0,
            GREEN,
        );

        gif_animation.tick();
        gif_animation.tick();
        next_frame().await;
    }
    text

}


#[macroquad::main("3D")]
async fn main() {
    return;
}

