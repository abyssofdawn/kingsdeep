use std::{borrow::{Borrow, BorrowMut}, cmp::min, collections::HashMap, fmt::format, hash::Hash, mem::offset_of, ops::{Add, Deref, Div, Mul, Sub}};

use bracket_lib::{color::{ColorPair, RGB, RGBA}, prelude::{to_char, to_cp437, BTerm, GameState, TextAlign, VirtualKeyCode, INPUT}};
use bracket_terminal::FontCharType;
use mlua::{chunk, FromLua, FromLuaMulti, Function, IntoLua, IntoLuaMulti, Lua, UserData, Value};

use super::*;


pub trait Move {
    fn move_delta(&mut self, to: Self) -> Self;
    fn move_abs(&mut self, to: Self) -> Self;
}

pub trait SetContext {
    fn draw_to_ctx(&self, state: &mut State, pos: Vec2<usize>, size: Vec2<usize>);
}

#[derive(Clone)]
pub struct Context {
    pub glyphs: Vec2d<FontCharType>,
    pub colors: Vec2d<ColorPair>
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LoadedTile {
    pub name: String,
    pub icon: FontCharType,
    pub sprite: Vec2d<FontCharType>, //should be 5x5, 25c long
}

#[derive(Clone, PartialEq, Eq)]
pub struct WorldState {
    player_pos: Vec2<isize>,
    old_player_pos: Vec2<isize>,
    player_rot: Direction,
    pub loaded_areas: HashMap<u16, LoadedArea>,
    delta: Vec2<isize>,
    last_move_time: isize,
    pub loaded_tiles: HashMap<u16, LoadedTile>,

}


pub struct MenuState {}

pub struct State {
    game_time: isize,
    scripts: LuaLoader,
    pub world: WorldState,
    pub win_size: Vec2<usize>,
    pub context: Context,
}




impl State {
    pub fn new() -> mlua::Result<Self> {
        let mut state: State = State {
            scripts: LuaLoader::new()?,
            world: WorldState { 
                player_pos: Vec2::<isize> {
                    x: 0, 
                    y: 0
                },
                old_player_pos: Vec2::<isize> {
                    x: 0, 
                    y: 0
                },
                loaded_areas: HashMap::new(),
                loaded_tiles: HashMap::new(),
                player_rot: Direction::NORTH,
                delta: Vec2 {x: 0, y: 0},
                last_move_time: 0,
            },
            game_time: 0,
            win_size: Vec2::new(80, 45),
            context: Context { glyphs: Vec2d::new((80, 45).into()), colors: Vec2d::new((80,45).into()) }
        };

        for x in 0..state.context.glyphs.map.capacity() { 
            state.context.glyphs.set_at(state.context.glyphs.pos_for_idx(x).into(), 0);
            state.context.colors.set_at(state.context.colors.pos_for_idx(x).into(), ColorPair { fg: RGB::from_u8(255, 255, 255).into(), bg: RGB::from_u8(0, 0, 0).into() });
        }


        Ok(state)
    }
    pub fn move_player(&mut self, delta: Vec2<isize>) {
        if self.world.last_move_time > 250 {
            let _ = self.scripts.state.load(
                chunk! {
                    move_player($delta)
                }
            ).exec();
            self.world.player_pos = self.scripts.state.globals().get("player_pos").unwrap();
            self.world.last_move_time = 1;
            self.world.delta = Vec2 { x: 0, y: 0 };
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let anim_lock = 250;
        self.game_time += ctx.frame_time_ms as isize;
        self.world.last_move_time += ctx.frame_time_ms as isize;
        ctx.cls();
        ctx.printer(0, 0, format!("#[{}]hello #[]world! {}. time total: {:}", "blue", to_char(glyph_for_dir(self.world.player_rot) as u8), self.game_time), TextAlign::Left, None);
        let _ = self.scripts.state.globals().set("player_pos", self.world.player_pos);

        let input = INPUT.lock();
        let mut delta =  0; 
        let mut dir = self.world.player_rot;




        for (i, key) in input.key_pressed_set().iter().enumerate() {
            ctx.print(50, 25 + i as i32, &format!("Key code: {:?}", key));
            {
                match *key {
                    VirtualKeyCode::A => {
                        dir = rot_ccw(self.world.player_rot);
                    },
                    VirtualKeyCode::S => {
                        delta += 1;
                    },
                    VirtualKeyCode::D => {
                        dir = rot_cw(self.world.player_rot);
                    },
                    VirtualKeyCode::W => {
                        delta += -1;
                    },
                    _ => {},
                }
            }
        }
        if self.world.last_move_time >= anim_lock {
            self.world.old_player_pos = self.world.player_pos;

            if dir != self.world.player_rot {
                self.world.last_move_time = 0;
                self.world.player_rot = dir;
            } else if delta != 0 {
                self.world.delta = match self.world.player_rot {
                    Direction::NORTH => {
                        Vec2::new(0, -delta)
                    },
                    Direction::EAST => {
                        Vec2::new(delta, 0)
                    },
                    Direction::SOUTH => {
                        Vec2::new(0, delta)
                    },
                    Direction::WEST => {
                        Vec2::new(-delta, 0)
                    },
                };
            }
        }
        if self.world.delta != (Vec2 {x: 0, y: 0}) {
            self.move_player(self.world.delta)
        }


        let range = 7;
        let size = 5;
        let uoffset = Vec2::<usize>::new(1, 1);
        let offset = Vec2::<isize>::new(1, 1);

        
        
        let dir = self.world.player_pos - self.world.old_player_pos;
        let render_pos = (self.world.old_player_pos * (size as isize) + dir * min::<isize>(((size as isize) * self.world.last_move_time) / anim_lock, size as isize) ) + offset + Vec2::<isize>::new(size as isize/2, size as isize/2);
        
        let size = self.win_size.clone() - uoffset;        

        ctx.set(
            20,
            20, 
            RGB::from_u8((self.game_time%128 + 128) as u8, ((self.game_time / 2) % 128 + 128) as u8, ((self.game_time/5) % 128 + 128) as u8 ), 
            RGBA::from_u8(255, 255, 255, 0), 
            24
        );
    }
}


pub struct LuaLoader {
    state: Lua
}


impl LuaLoader {
    fn new() -> mlua::Result<Self> {
        let state =  Lua::new();


        
        let move_player: Function = state.create_function(|lua, pos:  Vec2<isize> | {
            lua.globals().set::<&str, Vec2<isize>>(
                "player_pos", 
                lua.globals().get::<_, Vec2<isize>>("player_pos")
                    .unwrap()
                    .move_delta(pos))?;
            Ok(())
        })?;
        state.globals().set("move_player", move_player)?;

        Ok(Self { state })
    }
}