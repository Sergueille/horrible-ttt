
use crate::*;
use crate::state::State;
use crate::util::Vec3i;

pub struct GameInfo {
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

    pub state: GameState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Cross, Circle, None
}

pub enum GameState {
    Turn(BlockType), GameWon(BlockType),
}

pub fn pos_to_id(pos: &Vec3i) -> i32 {
    return ((pos.x + ROW_COUNT) % ROW_COUNT)
         + ((pos.y + ROW_COUNT) % ROW_COUNT) * ROW_COUNT 
         + ((pos.z + ROW_COUNT) % ROW_COUNT) * ROW_COUNT * ROW_COUNT;
}

pub fn get_block(pos: &Vec3i, state: &State) -> BlockType {
    return state.game.blocks[pos_to_id(&pos) as usize];
}

pub fn set_block(pos: &Vec3i, value: BlockType, state: &mut State) {
    state.game.blocks[pos_to_id(&pos) as usize] = value;
}

pub fn submit_click(pos: &Vec3i, state: &mut State) {
    let current_block = get_block(pos, state);

    let block_type = match state.game.state {
        GameState::Turn(block_type) => block_type,
        GameState::GameWon(_) => panic!(),
    };

    if current_block == BlockType::None {
        set_block(pos, block_type, state);

        let victory = check_for_victory(state);
        match victory {
            BlockType::None => {
                if block_type == BlockType::Cross {
                    state.game.state = GameState::Turn(BlockType::Circle);
                }
                else {
                    state.game.state = GameState::Turn(BlockType::Cross);
                }
            },
            someone => state.game.state = GameState::GameWon(someone),
        };
    }
    else {
        // Already something here, maybe give some feedback to player
    }
}

pub fn check_for_victory(state: &State) -> BlockType {
    let deltas = [
        [1, 0, 0],
        [1, 1, 0],
        [1, 1, 1],
        [1, -1, 1],
        [1, -1, -1],
        [1, 1, -1],
        [1, 0, -1],
        [1, -1, 0],
        [0, 1, 0],
        [0, 0, 1],
        [0, 1, 1],
        [0, -1, 1],
        [0, 1, -1],
    ];

    for x in 0..ROW_COUNT {
        for y in 0..ROW_COUNT {
            for z in 0..ROW_COUNT {
                let block_type = get_block(&util::vec3i(x, y, z), state);

                if block_type == BlockType::None {
                    continue;
                }

                for j in 0..13 {
                    let mut okay = true;
                    for i in 0..COUNT_TO_WIN {
                        if get_block(&util::vec3i(x + deltas[j][0] * i, y + deltas[j][1] * i, z + deltas[j][2] * i), state) != block_type {
                            okay = false;
                            break;
                        }
                    }

                    if okay {
                        return block_type;
                    }
                }
            }
        }
    }

    return BlockType::None;
}
