use bracket_terminal::FontCharType;

use super::*;

pub fn rot_cw(dir: Direction) -> Direction {
    match dir {
        Direction::NORTH => {
            return Direction::EAST;
        },
        Direction::EAST => {
            return Direction::SOUTH;
        },
        Direction::SOUTH => {
            return Direction::WEST;
        },
        Direction::WEST => {
            return Direction::NORTH;
        },
    }
}

pub fn rot_ccw(dir: Direction) -> Direction {
    match dir {
        Direction::NORTH => {
            return Direction::WEST;
        },
        Direction::EAST => {
            return Direction::NORTH;
        },
        Direction::SOUTH => {
            return Direction::EAST;
        },
        Direction::WEST => {
            return Direction::SOUTH;
        },
    }
}

pub fn glyph_for_dir(dir: Direction) -> FontCharType {
    match dir {
        Direction::NORTH => {
            return 24;
        },
        Direction::EAST => {
            return 26;
        },
        Direction::SOUTH => {
            return 25;
        },
        Direction::WEST => {
            return 27;
        },
    }
}

pub fn global_index_for_coords_in_box(rect_pos: (isize, isize), rect_size: (usize, usize), coords: (usize, usize), full_size: (usize, usize)) -> usize {
    (coords.0 + rect_size.0 * coords.1.wrapping_add_signed(rect_pos.0) + full_size.0.wrapping_mul(rect_pos.1.try_into().unwrap())).try_into().unwrap()
}

pub fn index_for_coords_in_box(rect_size: (u32, u32), coords: (u32, u32)) -> u32 {
    coords.0 + coords.1 * rect_size.0
}