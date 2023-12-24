
use std::vec;

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
