use rand::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Element {
    #[default]
    Path,
    Wall,
    Exit,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn make_map() -> [[Element; 41]; 41] {
    let mut map = [[Element::default(); 41]; 41];
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
                        make_wall_up(&mut map, x);
                    }
                }
                Direction::Down => {
                    if let Some(x) = v.pop() {
                        make_wall_down(&mut map, x);
                    }
                }
                Direction::Left => {
                    if let Some(y) = h.pop() {
                        make_wall_left(&mut map, y);
                    }
                }
                Direction::Right => {
                    if let Some(y) = h.pop() {
                        make_wall_right(&mut map, y);
                    }
                }
            }
        }
    }
    map[39][40] = Element::Exit;
    map
}

fn make_wall_down(map: &mut [[Element; 41]; 41], x: usize) {
    for y in (1..41).rev() {
        match map[y - 1][x] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_up(map: &mut [[Element; 41]; 41], x: usize) {
    for y in 0..40 {
        match map[y + 1][x] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_right(map: &mut [[Element; 41]; 41], y: usize) {
    for x in 0..40 {
        match map[y][x + 1] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}

fn make_wall_left(map: &mut [[Element; 41]; 41], y: usize) {
    for x in (1..41).rev() {
        match map[y][x - 1] {
            Element::Path => {
                map[y][x] = Element::Wall;
            }
            _ => {}
        }
    }
}
