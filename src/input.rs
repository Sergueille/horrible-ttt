
// Is a button pressed?
pub struct ButtonInfo {
    pub down: bool, // The button went from unpressed to pressed
    pub up: bool, // The button went from pressed to unpressed
    pub hold: bool, // Th button is pressed
}

pub fn get_info() -> ButtonInfo {
    return ButtonInfo { 
        down: false,
        up: false,
        hold: false,
    }
}

// Updates values with an event
pub fn update_info(action: &winit::event::ElementState, info: &mut ButtonInfo) {
    match action {
        winit::event::ElementState::Pressed => {
            info.down = true;
            info.hold = true;
        },
        winit::event::ElementState::Released => {
            info.up = true;
            info.hold = false;
        },
    }
}

// Call this every frame, before getting events
pub fn reset_info(info: &mut ButtonInfo) {
    info.down = false;
    info.up = false;
}
