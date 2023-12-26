
use crate::*;
use crate::state::State;
use crate::util::Vec3i;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Cross, Circle, Empty
}

pub fn pos_to_id(pos: &Vec3i) -> i32 {
    return pos.x + pos.y * ROW_COUNT + pos.z * ROW_COUNT * ROW_COUNT;
}

pub fn get_block(pos: &Vec3i, state: &State) -> BlockType {
    return state.blocks[pos_to_id(&pos) as usize];
}

pub fn set_block(pos: &Vec3i, value: BlockType, state: &mut State) {
    state.blocks[pos_to_id(&pos) as usize] = value;
}

pub fn submit_click(pos: &Vec3i, state: &mut State) {
    let current_block = get_block(pos, state);
    let turn_type = if state.is_cross_turn { BlockType::Cross } else { BlockType::Circle };

    if current_block == BlockType::Empty {
        set_block(pos, turn_type, state);
        state.is_cross_turn = !state.is_cross_turn;
    }
    else {
        // Already something here, maybe give some feedback to player
    }
}
