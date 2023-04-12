#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Room {
    Empty,
    Wall,
    Home(u16),
    Goal(u16),
}

pub trait Maze {
    /// when out of bounds, return `Room::wall` instead of panicking
    fn get(&self, x: isize, y: isize) -> Room;
}
