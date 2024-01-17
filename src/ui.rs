use gl_matrix::common::*;
use crate::{state::State, text, draw};

pub enum Align {
    Start, Middle, End,
}

pub enum Direction {
    Vertical, Horizontal
}

pub enum UILink {
    None,
    Element(Box<UIElement>),
    Text(Box<TextInfo>)
}

pub struct Rect {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

pub struct UIElement {
    pub name: String,
    pub layout_info: LayoutInfo,
    pub background_color: Vec4,
    pub first_child: UILink,
    pub sibling: Option<Box<UIElement>>,
}

pub struct LayoutInfo {
    pub min_size: Vec2,
    pub sec_align: Align,
    pub main_align: Align,
    pub align_direction: Direction,
    pub padding: f32,
}

pub struct TextInfo {
    pub content: String,
    pub size: f32,
    pub color: Vec4,
}

// OPTI: cache size in the struct (sizeX, sizeY, innerX, innerY)
pub fn get_size(el: &UIElement, state: &State) -> Vec4 {
    let mut res = [0.0, 0.0, 0.0, 0.0];

    // Get size of inner content
    match &el.first_child {
        UILink::None => {},
        UILink::Element(ref cur) => {
            let mut current = cur;
            loop {
                let size = get_size(&current, state);
                match el.layout_info.align_direction {
                    Direction::Vertical => {
                        res[2] = size[2].max(res[2]);
                        res[3] += size[3];
                    },
                    Direction::Horizontal => {
                        res[2] += size[2];
                        res[3] = size[3].max(res[3]);
                    },
                }

                match current.sibling {
                    Some(ref sibling) => current = sibling,
                    None => break,
                }
            }
        }
        UILink::Text(ref info) => {
            text_size(&*info, state);
        }
    }

    res[0] = res[2] + 2.0 * el.layout_info.padding; // Padding
    res[1] = res[3] + 2.0 * el.layout_info.padding;

    res[0] = res[0].max(el.layout_info.min_size[0]); // Expand to min_size
    res[1] = res[1].max(el.layout_info.min_size[1]);

    return res;
}

pub fn draw(el: &UIElement, draw_corner: Vec2, state: &mut State) -> Vec4 {
    let size = get_size(el, state);

    let main_axis;
    let sec_axis;
    match el.layout_info.align_direction {
        Direction::Horizontal => {
            main_axis = 0;
            sec_axis = 1;
        },
        Direction::Vertical => {
            main_axis = 1;
            sec_axis = 0;
        }
    }

    // TODO: actually draw the element
    let center = [
        draw_corner[0] + size[0],
        draw_corner[1] + size[1],
        0.0, // FIXME: what to put here?
    ];
    draw::draw_screen_billboard(center, [size[0], size[1]], 0.0, [].into_iter(), draw::TexArg::None, "default_color", state);

    let pos_main;

    match el.layout_info.main_align {
        Align::Start => {
            pos_main = draw_corner[main_axis] + el.layout_info.padding;
        },
        Align::Middle => {
            pos_main = draw_corner[main_axis] + size[main_axis] / 2.0 - size[main_axis + 2] / 2.0;
        },
        Align::End => {
            pos_main = draw_corner[main_axis] + size[main_axis] - el.layout_info.padding - size[main_axis + 2];
        },
    }

    let cur = &el.first_child;

    match cur {
        UILink::None => {},
        UILink::Element(ref el) => {
            let mut cur = el;
            let mut corner = [0.0, 0.0];
            corner[main_axis] = pos_main;

            loop {
                let child_size = get_size(&cur, state);

                corner[sec_axis] = draw_corner[sec_axis] + get_sec_axis_alignement(el, &size, &[child_size[0], child_size[1]], sec_axis);
                draw(cur, corner, state);

                corner[main_axis] += child_size[main_axis];

                match cur.sibling {
                    Some(ref sibling) => cur = sibling,
                    None => break,
                };
            }

            
        },
        UILink::Text(ref info) => {
            let txt_size = text_size(info, state);
            let mut corner = [0.0, 0.0];
            corner[main_axis] = pos_main;
            corner[sec_axis] = draw_corner[sec_axis] + get_sec_axis_alignement(el, &size, &txt_size, sec_axis);

            text_draw(info, corner, state);
            corner[main_axis] += txt_size[main_axis];
        },
    }

    return size;
}

fn text_draw(info: &TextInfo, corner: Vec2, state: &mut State) {
    text::draw_text(&info.content, corner, info.size, info.color, state);
}

fn text_size(info: &TextInfo, state: &State) -> Vec2 {
    return [
        text::get_text_width(&info.content, info.size, state),
        text::LINE_HEIGHT * info.size, 
    ];
}

fn get_sec_axis_alignement(parent: &UIElement, parent_size: &Vec4, child_size: &Vec2, axis: usize) -> f32 {
    return match parent.layout_info.sec_align {
        Align::Start => parent.layout_info.padding,
        Align::Middle => parent_size[axis] / 2.0 - child_size[axis] / 2.0,
        Align::End => parent_size[axis] - child_size[axis] - parent.layout_info.padding,
    };
}
