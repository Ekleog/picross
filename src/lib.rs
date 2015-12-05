
#[derive(Clone, PartialEq)]
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
    fn get_integer(data: Option<&str>, name: &str) -> usize {
        match data {
            None    => panic!("Expected to find a {}!", name),
            Some(x) => x.parse().ok().expect(&format!("Expected integer {}!", name))
        }
    }

    pub fn parse(data: &mut Iterator<Item=&str>) -> Picross {
        let height = Picross::get_integer(data.next(), "height");
        let length = Picross::get_integer(data.next(), "length");

        let cells = vec![vec![Cell::Unknown; length]; height];

        Picross {
            height: height,
            length: length,

            row_spec: vec![],
            col_spec: vec![],

            cells: cells,
        }
    }
}

#[test]
fn it_works() {
    let data = vec![
        "42",
        "24",
    ];
    let picross = Picross::parse(&mut data.into_iter());
    assert!(picross.height == 42);
    assert!(picross.length == 24);
    assert!(picross.cells[37][13] == Cell::Unknown);
}
