use bevy::prelude::*;
use rand::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Element {
    #[default]
    Path,
    Wall,
    Exit,
}

const NUM_LEVEL_MAX_AISLE: usize = 50;
const NUM_LEVEL_MIN_AISLE: usize = 5;
const NUM_EXTERNAL_TILE: usize = 5;
#[allow(dead_code)]
const ZERO_OFFSET: usize = NUM_EXTERNAL_TILE + 1 + (NUM_LEVEL_MAX_AISLE * 2 + 1);
const NUM_TILES: usize = 2 * NUM_EXTERNAL_TILE + 2 + (NUM_LEVEL_MAX_AISLE * 2 + 1) * 2 + 1;

type MapType = [[Element; NUM_TILES]; NUM_TILES];

pub struct LevelMap {
    map: MapType,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl LevelMap {
    pub fn new(num_aisles: usize) -> Option<Self> {
        if num_aisles < NUM_LEVEL_MIN_AISLE || num_aisles > NUM_LEVEL_MAX_AISLE {
            return None;
        }
        let mut map = [[Element::default(); NUM_TILES]; NUM_TILES];
        make_walls(&mut map);
        Some(LevelMap {
            map: map,
            width: 41,
            height: 41,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Element> {
        let row = self.map.get(y)?;
        if let Some(element) = row.get(x) {
            return Some(*element);
        }
        None
    }

    pub fn get_coordinates_for(&self, element: Element) -> Vec<Vec3> {
        let mut coordinates: Vec<Vec3> = Vec::new();
        for (y, row) in self.map.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                if cell == element {
                    coordinates.push(Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: 0.0,
                    })
                }
            }
        }
        coordinates
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn make_walls(map: &mut MapType) {
    let mut h = Vec::from_iter((0..39).step_by(2));
    let mut v = Vec::from_iter((0..39).step_by(2));
    v.shuffle(&mut thread_rng());
    h.shuffle(&mut thread_rng());

    for x in 0..41 {
        map[0][x] = Element::Wall;
        map[40][x] = Element::Wall;
    }
    for y in 0..41 {
        map[y][0] = Element::Wall;
        map[y][40] = Element::Wall;
    }

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
                        make_wall_up(map, x);
                    }
                }
                Direction::Down => {
                    if let Some(x) = v.pop() {
                        make_wall_down(map, x);
                    }
                }
                Direction::Left => {
                    if let Some(y) = h.pop() {
                        make_wall_left(map, y);
                    }
                }
                Direction::Right => {
                    if let Some(y) = h.pop() {
                        make_wall_right(map, y);
                    }
                }
            }
        }
    }
    map[39][40] = Element::Exit;
}

fn make_wall_down(map: &mut MapType, x: usize) {
    for y in (1..41).rev() {
        match map[y - 1][x] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_up(map: &mut MapType, x: usize) {
    for y in 0..40 {
        match map[y + 1][x] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_right(map: &mut MapType, y: usize) {
    for x in 0..40 {
        match map[y][x + 1] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_left(map: &mut MapType, y: usize) {
    for x in (1..41).rev() {
        match map[y][x - 1] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}
