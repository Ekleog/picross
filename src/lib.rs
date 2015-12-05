
pub enum Cell {
    Unknown,
    Black,
    White,
}

pub struct Picross {
    pub height: usize,
    pub length: usize,


    pub cells: Vec<Vec<Cell>>, // Used as cells[y][x]
}

impl Picross {
    pub fn parse(data: &mut Iterator<Item=&str>) -> Picross {
        Picross {
            height: 0,
            length: 0,
            cells: vec![],
        }
    }
}

#[test]
fn it_works() {
    let _ = Picross { height: 0, length: 0, cells: vec![], };
}
