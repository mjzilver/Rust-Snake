
const WIDTH: u16 = 10;
const HEIGHT: u16 = 20;

#[derive(Clone)]
pub struct Board<'a> {
    pub data: Vec<Vec<Cell>>,
    pub current: Vec<&'a Cell>,
}
#[derive(Clone, Debug)]
pub enum Cell {
    Blue,
    Red,
    Yellow,
    Purple,
    Orange,
    Brown,
    Black,
    White,
    Green,
}

impl Board<'_> {
    pub fn init() -> Board<'static> {
        let data = vec![vec![Cell::Black; WIDTH as usize]; HEIGHT as usize];
        let current = vec![];
        Board { data, current }
    }

    pub fn print(&self) {
        self.data.iter().for_each(|row| {
            row.iter().for_each(|cell| match cell {
                Cell::Blue => print!("🟦"),
                Cell::Red => print!("🟥"),
                Cell::Yellow => print!("🟨"),
                Cell::Purple => print!("🟪"),
                Cell::Orange => print!("🟧"),
                Cell::Brown => print!("🟫"),
                Cell::Black => print!("⬛"),
                Cell::White => print!("⬜"),
                Cell::Green => print!("🟩"),
            });
            println!();
        });
    }
}
