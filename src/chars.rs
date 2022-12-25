use show_image::event::VirtualKeyCode;

pub fn convertchar(character: &Option<VirtualKeyCode>) -> &str {
    match character {
        Some(VirtualKeyCode::Space) => "Space",
        // todo - add A-Z 0-9
        _ => "",
    }
}
