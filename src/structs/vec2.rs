use std::{convert, ops::{Add, Div, Mul, Sub}};

use mlua::{FromLua, Lua, UserData, Value};
use super::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2::<T> { 
            x,
            y
        }
    }
}

impl<T: Mul<Output = T> + Copy> Vec2<T> {
    pub fn area(&self) -> T {
        self.x * self.y
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Move for Vec2<T> {
    fn move_abs(&mut self, to: Vec2<T>) -> Self {
        self.x = to.x;
        self.y = to.y;
        self.clone()
    }
    fn move_delta(&mut self, to: Vec2<T>) -> Self {
        self.x = to.x + self.x;
        self.y = to.y + self.y;
        self.clone()
    }
}

impl<T: std::ops::Add<Output = T>> Add for Vec2<T> {
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::<T> { x: self.x + rhs.x, y: self.y + rhs.y }
    }
    type Output = Vec2<T>;
} 

impl<T: std::ops::Sub<Output = T>> Sub for Vec2<T> {
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::<T> { x: self.x - rhs.x, y: self.y - rhs.y }
    }
    type Output = Vec2<T>;
} 

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    fn mul(self, rhs: T) -> Self::Output {
        Vec2::<T> { x: self.x * rhs, y: self.y * rhs }
    }
    type Output = Vec2<T>;
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    fn div(self, rhs: T) -> Self::Output {
        Vec2::<T> { x: self.x / rhs, y: self.y / rhs }
    }
    type Output = Vec2<T>;
}

impl FromLua<'_> for Vec2<isize> {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::UserData(ud) => { Ok(*ud.borrow::<Self>()?) },
            _ => unreachable!(),
        }
    }
}

impl UserData for Vec2<isize> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| { Ok(this.x) });
        fields.add_field_method_set("x", |_, this, val| {
            this.x = val;
            Ok(())
        });

        fields.add_field_method_get("y", |_, this| { Ok(this.y) });
        fields.add_field_method_set("y", |_, this, val| {
            this.y = val;
            Ok(())
        });
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Vec2::<T>::new(value.0, value.1)
    }
}

impl<T> Into<(T, T)> for Vec2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}