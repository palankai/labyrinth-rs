use rand::prelude::*;

use crate::utils::collections::{DynamicMap, DynamicMapEntry};

use super::Element;

pub struct SimpleLevel {
    exterior_offset: i32,
    skirt_offset: i32,

    map: DynamicMap<Element>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SimpleLevel {
    pub fn generate(num_half_asiles: i32, num_skirt_asiles: i32) -> SimpleLevel {
        let exterior_offset = num_half_asiles * 2 + 1;
        let skirt_offset = exterior_offset + num_skirt_asiles * 2;

        let mut level = SimpleLevel {
            map: DynamicMap::new(),
            exterior_offset,
            skirt_offset,
        };
        level.generate_level();

        level
    }

    pub fn tiles(&self) -> Vec<DynamicMapEntry<Element>> {
        self.map.all()
    }

    fn generate_level(&mut self) {
        self.generate_walls();
        self.make_exit();
        self.make_player();
    }

    fn generate_walls(&mut self) {
        self.make_outer_skirt();
        self.make_exterior_wall();
        self.make_internal_walls();
    }

    fn make_exit(&mut self) {
        let mut i = Vec::from_iter((-self.exterior_offset + 1..self.exterior_offset).step_by(2));
        thread_rng();
        i.shuffle(&mut thread_rng());
        let i = i.pop().unwrap();

        let mut d = vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];
        d.shuffle(&mut thread_rng());
        match d.pop().unwrap() {
            Direction::Up => {
                self.map.set(i, self.exterior_offset, Element::Exit);
            }
            Direction::Down => {
                self.map.set(i, -self.exterior_offset, Element::Exit);
            }
            Direction::Left => {
                self.map.set(-self.exterior_offset, i, Element::Exit);
            }
            Direction::Right => {
                self.map.set(self.exterior_offset, i, Element::Exit);
            }
        }
    }

    fn make_player(&mut self) {
        self.map.set(0, 0, Element::Player);
    }

    fn make_outer_skirt(&mut self) {
        for i in -self.skirt_offset..self.skirt_offset + 1 {
            self.map.set(i, -self.skirt_offset, Element::Wall);
            self.map.set(i, self.skirt_offset, Element::Wall);
            self.map.set(-self.skirt_offset, i, Element::Wall);
            self.map.set(self.skirt_offset, i, Element::Wall);
        }
    }

    fn make_exterior_wall(&mut self) {
        for i in -self.exterior_offset..self.exterior_offset + 1 {
            self.map.set(i, -self.exterior_offset, Element::Wall);
            self.map.set(i, self.exterior_offset, Element::Wall);
            self.map.set(-self.exterior_offset, i, Element::Wall);
            self.map.set(self.exterior_offset, i, Element::Wall);
        }
    }

    fn make_internal_walls(&mut self) {
        let mut h = Vec::from_iter((-self.skirt_offset + 2..self.skirt_offset - 1).step_by(2));
        let mut v = Vec::from_iter((-self.skirt_offset + 2..self.skirt_offset - 1).step_by(2));

        h.shuffle(&mut thread_rng());
        v.shuffle(&mut thread_rng());

        while !v.is_empty() && !h.is_empty() {
            let mut d = vec![
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ];
            d.shuffle(&mut thread_rng());
            for dir in d {
                match dir {
                    Direction::Up => {
                        if let Some(x) = v.pop() {
                            self.make_wall_up(x);
                        }
                    }
                    Direction::Down => {
                        if let Some(x) = v.pop() {
                            self.make_wall_down(x);
                        }
                    }
                    Direction::Left => {
                        if let Some(y) = h.pop() {
                            self.make_wall_left(y);
                        }
                    }
                    Direction::Right => {
                        if let Some(y) = h.pop() {
                            self.make_wall_right(y);
                        }
                    }
                }
            }
        }
    }

    fn make_wall_up(&mut self, x: i32) {
        for i in -self.skirt_offset + 1..self.skirt_offset - 1 {
            if self.map.is_empty(x, i + 1) {
                self.map.set(x, i, Element::Wall)
            }
        }
        self.map.set(x, -self.skirt_offset, Element::Wall);
        self.map.set(x, self.skirt_offset, Element::Wall);
    }

    fn make_wall_down(&mut self, x: i32) {
        for i in (-self.skirt_offset + 2..self.skirt_offset).rev() {
            if self.map.is_empty(x, i - 1) {
                self.map.set(x, i, Element::Wall)
            }
        }
        self.map.set(x, -self.skirt_offset, Element::Wall);
        self.map.set(x, self.skirt_offset, Element::Wall);
    }

    fn make_wall_right(&mut self, y: i32) {
        for i in -self.skirt_offset + 1..self.skirt_offset - 1 {
            if self.map.is_empty(i + 1, y) {
                self.map.set(i, y, Element::Wall)
            }
        }
        self.map.set(-self.skirt_offset, y, Element::Wall);
        self.map.set(self.skirt_offset, y, Element::Wall);
    }

    fn make_wall_left(&mut self, y: i32) {
        for i in (-self.skirt_offset + 2..self.skirt_offset).rev() {
            if self.map.is_empty(i - 1, y) {
                self.map.set(i, y, Element::Wall)
            }
        }
        self.map.set(-self.skirt_offset, y, Element::Wall);
        self.map.set(self.skirt_offset, y, Element::Wall);
    }
}
