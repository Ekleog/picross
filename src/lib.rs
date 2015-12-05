
enum Cell {
    Unknown,
    Black,
    White,
}

struct Picross {
    height: usize,
    length: usize,

    cells: Vec<Vec<Cell>>, // Used as cells[y][x]
}

impl Picross {
    fn parse(string: &str) -> Picross {
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
