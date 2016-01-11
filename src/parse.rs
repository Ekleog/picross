use std::borrow::Borrow;

use ::{Cell, Picross};

impl Picross {
    ///
    /// /!\ Intended for internal use only /!\
    ///
    /// Parses `s` according to the format [1,2,4...]
    ///
    /// # Panics
    ///
    /// Panics if `s` is not in the format [1,2,3...]
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(picross::Picross::get_specs("[2,1]"), vec![2, 1]);
    /// assert_eq!(picross::Picross::get_specs("[42]"), vec![42]);
    /// assert_eq!(picross::Picross::get_specs("[]"), vec![]);
    /// ```
    ///
    /// The following lines will all trigger a panic:
    ///
    /// ```should_panic
    /// picross::Picross::get_specs("[");
    /// ```
    ///
    /// ```should_panic
    /// picross::Picross::get_specs("(1,2)");
    /// ```
    ///
    /// ```should_panic
    /// picross::Picross::get_specs("[1, 2]");
    /// ```
    ///
    /// ```should_panic
    /// picross::Picross::get_specs("[a,2]");
    /// ```
    ///
    pub fn get_specs<T: Borrow<str>>(s: T) -> Vec<usize> {
        let s = s.borrow();

        if s.len() < 2 || s[0..1].to_string() != "[" || s[s.len() - 1 .. s.len()].to_string() != "]" {
            panic!("Expected '{}' to be of form [1,4,3...]", s);
        }

        let s = &s[1 .. s.len() - 1];

        if s.len() == 0 {
            return vec![];
        }

        s.split(',')
         .map(|x| x.parse::<usize>()
                   .ok()
                   .expect(&format!("Expected integer and found '{}' in '{}'", x, s)))
         .collect::<Vec<usize>>()
    }

    ///
    /// Parses a Picross struct from an iterator to strings
    ///
    /// Takes in first the height, then the length, then `height` row specifications, and
    /// finally `length` column specifications.
    ///
    /// Fills the picross board with `Cell::Unknown` values.
    ///
    /// # Panics
    ///
    /// Panics if `data` is not an iterator to a valid Picross string.
    ///
    /// # Examples
    ///
    /// The board (solution shown with it)
    ///
    /// ```text
    ///    |  1   1
    ///    |  1   1
    ///    |1211 1121
    ///    |121111121
    /// ---+---------
    /// 3 3|###   ###
    /// 1 1| #     #
    /// 1 1|  #   #
    /// 1 1|   # #
    ///   1|    #
    /// 1 1|   # #
    /// 1 1|  #   #
    /// 1 1| #     #
    /// 3 3|###   ###
    /// ```
    ///
    /// is generated and filled with Cell::Unknown by the following code:
    ///
    /// ```
    /// use picross::Picross;
    /// # use picross::Cell;
    /// # use picross::Cell::{Black, White};
    ///
    /// let data = vec![
    ///     "9",
    ///     "9",
    ///
    ///     "[3,3]",
    ///     "[1,1]",
    ///     "[1,1]",
    ///     "[1,1]",
    ///     "[1]",
    ///     "[1,1]",
    ///     "[1,1]",
    ///     "[1,1]",
    ///     "[3,3]",
    ///
    ///     "[1,1]",
    ///     "[2,2]",
    ///     "[1,1,1,1]",
    ///     "[1,1]",
    ///     "[1]",
    ///     "[1,1]",
    ///     "[1,1,1,1]",
    ///     "[2,2]",
    ///     "[1,1]",
    /// ];
    ///
    /// let mut picross = Picross::parse(&mut data.into_iter());
    ///
    /// # assert!(picross.height == 9);
    /// # assert!(picross.length == 9);
    /// # assert!(picross.cells[3][4] == Cell::Unknown);
    /// # assert!(picross.row_spec[4] == vec![1]);
    /// # assert!(picross.col_spec[7] == vec![2, 2]);
    /// #
    /// # picross.cells = vec![
    /// #   vec![Black, Black, Black, White, White, White, Black, Black, Black],
    /// #   vec![White, Black, White, White, White, White, White, Black, White],
    /// #   vec![White, White, Black, White, White, White, Black, White, White],
    /// #   vec![White, White, White, Black, White, Black, White, White, White],
    /// #   vec![White, White, White, White, Black, White, White, White, White],
    /// #   vec![White, White, White, Black, White, Black, White, White, White],
    /// #   vec![White, White, Black, White, White, White, Black, White, White],
    /// #   vec![White, Black, White, White, White, White, White, Black, White],
    /// #   vec![Black, Black, Black, White, White, White, Black, Black, Black],
    /// # ];
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub fn parse<T: Borrow<str>>(data: &mut Iterator<Item=T>) -> Picross {
        let mut res = Picross {
            height: 0,
            length: 0,

            row_spec: vec![],
            col_spec: vec![],

            possible_rows: vec![],
            possible_cols: vec![],

            cells: vec![],
        };

        res.height = data.next().expect("Expected to find a height!").borrow()
            .parse().ok().expect("Expected integer height!");
        res.length = data.next().expect("Expected to find a length!").borrow()
            .parse().ok().expect("Expected integer length!");


        res.cells = vec![vec![Cell::Unknown; res.length]; res.height];

        res.row_spec = data.map(Picross::get_specs).take(res.height).collect();
        res.col_spec = data.map(Picross::get_specs).take(res.length).collect();

        if res.row_spec.len() != res.height || res.col_spec.len() != res.length {
            panic!("Wrong number of specifications given!");
        }

        res
    }
}
