
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

    fn get_specs(s: &str) -> Vec<usize> {
        if s.len() < 2 {
            panic!("Expected '{}' to be of form [1, 4, 3...]", s);
        }

        let s = &s[1 .. s.len() - 1];

        s.split(',')
         .map(|x| x.parse::<usize>()
                   .ok()
                   .expect(&format!("Expected integer and found '{}' in '{}'", x, s)))
         .collect::<Vec<usize>>()
    }

    fn fill_specs(size: usize, specs: &mut Vec<Vec<usize>>, data: &mut Iterator<Item=&str>) {
        for _ in 0..size {
            specs.push(Picross::get_specs(data.next().expect("Wrong number of specifications!")));
        }
    }

    pub fn parse(data: &mut Iterator<Item=&str>) -> Picross {
        let mut res = Picross {
            height: 0,
            length: 0,

            row_spec: vec![],
            col_spec: vec![],

            cells: vec![],
        };

        res.height = Picross::get_integer(data.next(), "height");
        res.length = Picross::get_integer(data.next(), "length");

        res.cells = vec![vec![Cell::Unknown; res.length]; res.height];

        Picross::fill_specs(res.height, &mut res.row_spec, data);
        Picross::fill_specs(res.length, &mut res.col_spec, data);

        res
    }
}

#[test]
fn it_works() {
    let data = vec![
        "9",
        "9",
        "[3,3]",
        "[1,1]",
        "[1,1]",
        "[1,1]",
        "[1]",
        "[1,1]",
        "[1,1]",
        "[1,1]",
        "[3,3]",
        "[1,1]",
        "[2,2]",
        "[1,1,1,1]",
        "[1,1]",
        "[1]",
        "[1,1]",
        "[1,1,1,1]",
        "[2,2]",
        "[1,1]",
    ];
    let picross = Picross::parse(&mut data.into_iter());
    assert!(picross.height == 9);
    assert!(picross.length == 9);
    assert!(picross.cells[3][4] == Cell::Unknown);
    assert!(picross.row_spec[2] == vec![1, 1]);
    assert!(picross.col_spec[7] == vec![2, 2]);
}
