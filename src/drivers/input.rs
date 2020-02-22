use winit::event::{ElementState, VirtualKeyCode};

pub fn get_keys(code: Option<VirtualKeyCode>, state: ElementState, keys: &mut [u8; 16]) {
    match code {
        Some(VirtualKeyCode::Key1) => match state {
            ElementState::Pressed => keys[0] = 1,
            ElementState::Released => keys[0] = 0,
        },
        Some(VirtualKeyCode::Key2) => match state {
            ElementState::Pressed => keys[1] = 1,
            ElementState::Released => keys[1] = 0,
        },
        Some(VirtualKeyCode::Key3) => match state {
            ElementState::Pressed => keys[2] = 1,
            ElementState::Released => keys[2] = 0,
        },
        Some(VirtualKeyCode::Key4) => match state {
            ElementState::Pressed => keys[3] = 1,
            ElementState::Released => keys[3] = 0,
        },
        Some(VirtualKeyCode::W) => match state {
            ElementState::Pressed => keys[4] = 1,
            ElementState::Released => keys[4] = 0,
        },
        Some(VirtualKeyCode::Q) => match state {
            ElementState::Pressed => keys[5] = 1,
            ElementState::Released => keys[5] = 0,
        },
        Some(VirtualKeyCode::E) => match state {
            ElementState::Pressed => keys[6] = 1,
            ElementState::Released => keys[6] = 0,
        },
        Some(VirtualKeyCode::R) => match state {
            ElementState::Pressed => keys[7] = 1,
            ElementState::Released => keys[7] = 0,
        },
        Some(VirtualKeyCode::A) => match state {
            ElementState::Pressed => keys[8] = 1,
            ElementState::Released => keys[8] = 0,
        },
        Some(VirtualKeyCode::S) => match state {
            ElementState::Pressed => keys[9] = 1,
            ElementState::Released => keys[9] = 0,
        },
        Some(VirtualKeyCode::D) => match state {
            ElementState::Pressed => keys[10] = 1,
            ElementState::Released => keys[10] = 0,
        },
        Some(VirtualKeyCode::F) => match state {
            ElementState::Pressed => keys[11] = 1,
            ElementState::Released => keys[11] = 0,
        },
        Some(VirtualKeyCode::Z) => match state {
            ElementState::Pressed => keys[12] = 1,
            ElementState::Released => keys[12] = 0,
        },
        Some(VirtualKeyCode::X) => match state {
            ElementState::Pressed => keys[13] = 1,
            ElementState::Released => keys[13] = 0,
        },
        Some(VirtualKeyCode::C) => match state {
            ElementState::Pressed => keys[14] = 1,
            ElementState::Released => keys[14] = 0,
        },
        Some(VirtualKeyCode::V) => match state {
            ElementState::Pressed => keys[15] = 1,
            ElementState::Released => keys[15] = 0,
        },
        _ => (),
    };
}
