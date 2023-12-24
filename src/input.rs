use crate::state;
use crate::util;

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

// Called in reset_input()
pub fn reset_info(info: &mut ButtonInfo) {
    info.down = false;
    info.up = false;
}

pub fn handle_input<T>(event: &winit::event::Event<T>, control_flow: &mut winit::event_loop::ControlFlow, state: &mut state::State) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => { // Close window
                *control_flow = winit::event_loop::ControlFlow::Exit;
            },
            winit::event::WindowEvent::CursorMoved { position, .. } => { // Get mouse position
                state.mouse_coords_pixels = util::vec2i(position.x as i32, state.resolution.y as i32 - position.y as i32);
                state.mouse_coords_normalized = [state.mouse_coords_pixels.x as f32 / state.resolution.y as f32, state.mouse_coords_pixels.y as f32 / state.resolution.y as f32];
                state.mouse_ray = util::get_mouse_ray(&state);

            },
            winit::event::WindowEvent::MouseInput { device_id: _, state: action, button, .. } => { // Get mouse buttons
                match button {
                    winit::event::MouseButton::Left => update_info(&action, &mut state.lmb),
                    winit::event::MouseButton::Right => update_info(&action, &mut state.rmb),
                    winit::event::MouseButton::Middle => update_info(&action, &mut state.mmb),
                    winit::event::MouseButton::Other(_) => {},
                }
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase: _, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        if *y > 0.0 {
                            state.wheel_up = true;
                        } 
                        else if *y < 0.0 {
                            state.wheel_down = true;
                        }
                    },
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        if pos.y > 0.0 {
                            state.wheel_up = true;
                        } 
                        else if pos.y < 0.0 {
                            state.wheel_down = true;
                        }
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
}


// Call this every frame, before getting events
pub fn reset_input(state: &mut state::State) {
    reset_info(&mut state.lmb);
    reset_info(&mut state.rmb);
    reset_info(&mut state.mmb);

    state.wheel_up = false;
    state.wheel_down = false;
}
