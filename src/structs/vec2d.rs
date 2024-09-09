use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Vec2d<T: Copy> {
    pub map: Vec<Option<T>>,
    pub size: Vec2<usize>,
    iter_idx: Vec2<usize>,
}
 
impl<T: Copy> Vec2d<T> {
    pub fn new(size: Vec2<usize>) -> Self {
        let mut vec2d = Vec2d::<T> {
            map: Vec::with_capacity(size.area()),
            size,
            iter_idx: (0, 0).into()
        };
        for _ in 0..vec2d.map.capacity() {
            vec2d.map.push(None);
        }
        vec2d
    }

    fn is_valid(&self, at: Vec2<isize>) -> bool{
        if at.x < 0 || at.y < 0 { return false }
        at.x < self.size.x.try_into().unwrap() || at.y < self.size.y.try_into().unwrap()
    }

    pub fn set_at(&mut self, at: Vec2<isize>, val: T) {
        if !self.is_valid(at) {
            return;
        }
        self.map[at.x as usize + at.y as usize * self.size.x] = Some(val);
    }

    pub fn get_at(&self, at: Vec2<isize>) -> Option<T> {
        if !self.is_valid(at) {
            return None;
        }
        return self.map[at.x as usize + at.y as usize * self.size.x];
    }

    pub fn set_box_at(&mut self, at: Vec2<isize>, vals: Vec2d<T>) {
        for i in 0..vals.size.x {
            for j in 0..vals.size.y {
                self.set_at(at, vals.get_at((i as isize, j as isize).into()).unwrap());
            }
        }
    }

    pub fn pos_for_idx(&self, idx: usize) -> (isize, isize) {
        return ((idx as isize)%self.size.x as isize, (idx as isize)/self.size.x as isize);
    }

    pub fn fill(&mut self, val: T) {
        for i in 0..self.map.capacity() {
            self.map[i] = Some(val);
        }
    }

    pub fn set(&mut self, vals: Vec<T>) {
        if vals.len() == self.size.area() {
            for i in 0..vals.len() {
                self.set_at(self.pos_for_idx(i).into(), vals[i]);
            }
        }
    } 
}