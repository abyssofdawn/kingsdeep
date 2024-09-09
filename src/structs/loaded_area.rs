use std::hash::Hash;

use bracket_lib::color::ColorPair;
use bracket_terminal::FontCharType;

use super::*;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct LoadedArea {
    pub name: String,
    pub size: Vec2<usize>,
    pub tiles: Vec2d<FontCharType>, //indexes of loadedtiles
    
}

impl SetContext for LoadedArea {
    fn draw_to_ctx(&self, state: &mut State, pos: Vec2<usize>, size: Vec2<usize>) {
        let mut ctx = &state.context;

        
    }
}

