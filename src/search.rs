use crate::maze::*;
use std::collections::{HashSet, VecDeque};

pub const UP: (i8, i8) = (0, 1);
pub const DOWN: (i8, i8) = (0, -1);
pub const RIGHT: (i8, i8) = (1, 0);
pub const LEFT: (i8, i8) = (-1, 0);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Direction {
    pub x: i8,
    pub y: i8,
    _private: (), // forbids member init
}

impl Direction {
    pub const UP: Self = Self {
        x: UP.0,
        y: UP.1,
        _private: (),
    };
    pub const DOWN: Self = Self {
        x: DOWN.0,
        y: DOWN.1,
        _private: (),
    };
    pub const RIGHT: Self = Self {
        x: RIGHT.0,
        y: RIGHT.1,
        _private: (),
    };
    pub const LEFT: Self = Self {
        x: LEFT.0,
        y: LEFT.1,
        _private: (),
    };
}

impl TryFrom<(i8, i8)> for Direction {
    type Error = String;

    fn try_from(value: (i8, i8)) -> Result<Self, Self::Error> {
        match value {
            UP | DOWN | RIGHT | LEFT => Ok(Self {
                x: value.0,
                y: value.1,
                _private: (),
            }),
            _ => Err(format!(
                "provided value {value:?} does not represent one of the four possible directions"
            )),
        }
    }
}

pub trait StepSearch {
    fn step_goal<T: Maze>(&mut self, maze: &T) -> Option<(usize, usize)>;

    fn step_home<T: Maze>(&mut self, maze: &T) -> Option<(usize, usize)>;
}

pub struct BFS {
    searched: HashSet<(usize, usize)>,
    edges: VecDeque<(usize, usize)>,
    current: usize,
}

impl BFS {
    pub fn new(home: (usize, usize)) -> Self {
        Self {
            current: 0,
            searched: HashSet::new(),
            edges: VecDeque::from([home]),
        }
    }
}

impl StepSearch for BFS {
    fn step_goal<T: Maze>(&mut self, maze: &T) -> Option<(usize, usize)> {
        let e = match self.edges.pop_front() {
            Some(v) => v,
            None => panic!("no path found"),
        };
        for n in [UP, DOWN, RIGHT, LEFT]
            .into_iter()
            .map(|v| (e.0 as isize + v.0 as isize, e.1 as isize + v.1 as isize))
        {
            if self.searched.contains(&(n.0 as usize, n.1 as usize)) {
                continue;
            }

            match maze.get(n.0, n.1) {
                Room::Empty => {
                    self.searched.insert((n.0 as usize, n.1 as usize));
                    self.edges.push_back((n.0 as usize, n.1 as usize));
                }
                Room::Goal => return None,
                Room::Home | Room::Wall => {}
            }
        }

        Some(e)
    }

    fn step_home<T: Maze>(&mut self, maze: &T) -> Option<(usize, usize)> {
        todo!()
    }
}
