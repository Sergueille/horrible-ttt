
use crate::*;
use crate::state::State;
use crate::util::Vec3i;

pub struct GameState {
    pub cube_transform_matrix: Mat4,
    pub cube_rotation: Quat,
    pub cube_size: f32,
    pub blocks: [game::BlockType; (ROW_COUNT * ROW_COUNT * ROW_COUNT) as usize],
    pub cube_rotation_velocity: Quat,
    
    pub last_mouse_sphere_intersection: Option<Vec3>,
    pub start_mouse_sphere_intersection: Option<Vec3>,
    pub mouse_sphere_radius: f32,
    pub drag_start_rotation: Quat,

    pub last_face_id: i32,
    pub depth: i32,

    pub is_cross_turn: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Cross, Circle, Empty
}

pub fn pos_to_id(pos: &Vec3i) -> i32 {
    return pos.x + pos.y * ROW_COUNT + pos.z * ROW_COUNT * ROW_COUNT;
}

pub fn get_block(pos: &Vec3i, state: &State) -> BlockType {
    return state.game.blocks[pos_to_id(&pos) as usize];
}

pub fn set_block(pos: &Vec3i, value: BlockType, state: &mut State) {
    state.game.blocks[pos_to_id(&pos) as usize] = value;
}

pub fn submit_click(pos: &Vec3i, state: &mut State) {
    let current_block = get_block(pos, state);
    let turn_type = if state.game.is_cross_turn { BlockType::Cross } else { BlockType::Circle };

    if current_block == BlockType::Empty {
        set_block(pos, turn_type, state);
        state.game.is_cross_turn = !state.game.is_cross_turn;
    }
    else {
        // Already something here, maybe give some feedback to player
    }
}
