use crate::playing_board::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
#[derive(Debug)]
pub struct Snake<'a> {
    body: Vec<&'a mut Cell>,
    direction: Direction,
    digesting: bool,
}

impl<'a> Snake<'a> {
    pub fn new(mut start: &'a mut Cell, direction: Direction) -> Self {
        *start = Cell::Snake;

        let mut body: Vec<&mut Cell> = vec![];
        body.push(start);

        return Self {
            body,
            direction,
            digesting: false,
        };
    }

    pub fn update(&mut self) {}
}
