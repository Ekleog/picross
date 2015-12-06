
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

    fn specs_to_strings(specs: &Vec<Vec<usize>>) -> Vec<String> {
        specs.iter()
             .map(|v| {
                 v.iter()
                  .map(|x| x.to_string())
                  .collect::<Vec<String>>()
                  .join(" ")
             })
             .collect()
    }

    fn max_len_non_empty(specs: &Vec<String>) -> usize {
        specs.iter()
             .map(|x| x.len())
             .max()
             .expect("Not supporting empty picross grids!")
    }

    pub fn to_string(&self) -> String {
        let row_spec = Picross::specs_to_strings(&self.row_spec);
        let col_spec = Picross::specs_to_strings(&self.col_spec);

        let max_rs_len = Picross::max_len_non_empty(&row_spec);
        let max_cs_len = Picross::max_len_non_empty(&col_spec);

        let line_begin = vec![" "; max_rs_len].join("");

        let mut res: String = "".to_string();

        for i in 0..max_cs_len {
            res = res + &line_begin + "|";
            for c in &col_spec {
                res.push(match c.chars().nth(max_cs_len - i - 1) {
                    Some(x) => x,
                    None    => ' '
                });
            }
            res.push('\n');
        }

        res = res + &vec!["-"; max_rs_len].join("") + "+" + &vec!["-"; self.length].join("") + "\n";

        for i in 0..self.height {
            res = res + &vec![" "; max_rs_len - row_spec[i].len()].join("") + &row_spec[i] + "|";
            res = res + &self.cells[i].iter().map(|c| match c {
                &Cell::Unknown => '?',
                &Cell::White   => ' ',
                &Cell::Black   => '#'
            }).collect::<String>();
            res.push('\n');
        }

        res
    }
}

#[test]
fn parsing_works() {
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

#[test]
fn displaying_works() {
    let picross = Picross {
        height: 3,
        length: 3,
        row_spec: vec![vec![1, 1], vec![], vec![1]],
        col_spec: vec![vec![1, 1], vec![1], vec![2]],
        cells: vec![vec![Cell::Unknown, Cell::White  , Cell::Black],
                    vec![Cell::White  , Cell::White  , Cell::Black],
                    vec![Cell::Black  , Cell::Unknown, Cell::Unknown]],
    };
    let res =
        "   |1  \n".to_string() +
        "   |   \n" +
        "   |112\n" +
        "---+---\n" +
        "1 1|? #\n" +
        "   |  #\n" +
        "  1|#??\n";
    assert!(picross.to_string() == res);
}
