
pub enum Cell {
    Unknown,
    Black,
    White,
}

pub struct Picross {
    pub height: usize,
    pub length: usize,

    pub row_spec: Vec<Vec<usize>>, // row_spec[y] contains the list of hints for row y, from left to right
    pub col_spec: Vec<Vec<usize>>, // col_spec[x] contains the list of hints for col x, from top to bottom

    pub cells: Vec<Vec<Cell>>, // Used as cells[y][x]
}

impl Picross {
    pub fn parse(data: &mut Iterator<Item=&str>) -> Picross {
        Picross {
            height: 0,
            length: 0,

            row_spec: vec![],
            col_spec: vec![],

            cells: vec![],
        }
    }
}

#[test]
fn it_works() {
    let _ = Picross { height: 0, length: 0, cells: vec![], };
}
