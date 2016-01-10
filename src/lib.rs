use std::borrow::Borrow;

pub mod display;

/// The Cell type
#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Unknown,
    Black,
    White,
}

/// A Picross board
pub struct Picross {
    /// Height of the board
    pub height: usize,
    /// Length of the board
    pub length: usize,

    ///
    /// Specifications for the rows of the board
    ///
    /// # Examples
    ///
    /// The board
    ///
    /// ```text
    ///    |1212
    /// ---+----
    /// 2 1|## #
    ///   3| ###
    /// ```
    ///
    /// is represented with
    ///
    /// ```
    /// # use picross::{Picross, Cell};
    /// # let picross = Picross{
    /// #   height: 2,
    /// #   length: 4,
    /// #   cells: vec![vec![Cell::Black, Cell::Black, Cell::White, Cell::Black],
    /// #               vec![Cell::White, Cell::Black, Cell::Black, Cell::Black]],
    /// #   row_spec: vec![vec![2, 1], vec![3]],
    /// #   col_spec: vec![vec![1], vec![2], vec![1], vec![2]],
    /// # };
    /// assert_eq!(picross.row_spec, vec![vec![2, 1], vec![3]]);
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub row_spec: Vec<Vec<usize>>,

    ///
    /// Specifications for the columns of the board
    ///
    /// # Examples
    ///
    /// The board
    ///
    /// ```text
    ///  |1
    ///  |
    ///  |2 1
    /// -+---
    /// 1|#
    /// 1|  #
    /// 1|#
    /// 1|#
    /// ```
    ///
    /// is represented with
    ///
    /// ```
    /// # use picross::{Picross, Cell};
    /// # let picross = Picross{
    /// #   height: 4,
    /// #   length: 3,
    /// #   cells: vec![vec![Cell::Black, Cell::White, Cell::White],
    /// #               vec![Cell::White, Cell::White, Cell::Black],
    /// #               vec![Cell::Black, Cell::White, Cell::White],
    /// #               vec![Cell::Black, Cell::White, Cell::White]],
    /// #   row_spec: vec![vec![1], vec![1], vec![1], vec![1]],
    /// #   col_spec: vec![vec![1, 2], vec![], vec![1]],
    /// # };
    /// assert_eq!(picross.col_spec, vec![vec![1, 2], vec![], vec![1]]);
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub col_spec: Vec<Vec<usize>>,

    ///
    /// Status of the cells of the picross board
    ///
    /// # Examples
    ///
    /// The board
    ///
    /// ```text
    ///    |2 2
    /// ---+---
    ///   1|#
    /// 1 1|# #
    ///   1|  #
    /// ```
    ///
    /// is represented with
    ///
    /// ```
    /// # use picross::{Picross, Cell};
    /// # let picross = Picross{
    /// #   height: 3,
    /// #   length: 3,
    /// #   cells: vec![vec![Cell::Black, Cell::White, Cell::White],
    /// #               vec![Cell::Black, Cell::White, Cell::Black],
    /// #               vec![Cell::White, Cell::White, Cell::Black]],
    /// #   row_spec: vec![vec![1], vec![1, 1], vec![1]],
    /// #   col_spec: vec![vec![2], vec![], vec![2]],
    /// # };
    /// assert_eq!(
    ///     picross.cells,
    ///     vec![vec![Cell::Black, Cell::White, Cell::White],
    ///          vec![Cell::Black, Cell::White, Cell::Black],
    ///          vec![Cell::White, Cell::White, Cell::Black]]
    /// );
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub cells: Vec<Vec<Cell>>, // Used as cells[y][x]
}

///
/// Methods intended for public use:
///
/// - parse
/// - to_string
/// - is_valid
///
impl Picross {
    ///
    /// Checks if a Picross is valid
    ///
    /// # Examples
    ///
    /// Valid picross grid:
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let picross = Picross {
    ///     height: 3,
    ///     length: 3,
    ///     cells: vec![vec![Cell::Black, Cell::Black, Cell::Black],
    ///                 vec![Cell::White, Cell::Black, Cell::White],
    ///                 vec![Cell::Black, Cell::White, Cell::Black]],
    ///     row_spec: vec![vec![3], vec![1], vec![1, 1]],
    ///     col_spec: vec![vec![1, 1], vec![2], vec![1, 1]],
    /// };
    ///
    /// assert!(picross.is_valid());
    /// ```
    ///
    /// Invalid picross grid with blatantly wrong dimensions:
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let picross = Picross {
    ///     height: 42,
    ///     length: 24,
    ///     cells: vec![vec![Cell::Black]],
    ///     row_spec: vec![vec![1]],
    ///     col_spec: vec![vec![1]],
    /// };
    ///
    /// assert!(!picross.is_valid());
    /// ```
    ///
    /// Invalid picross grid, with row specifications not respected:
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let picross = Picross {
    ///     height: 2,
    ///     length: 2,
    ///     cells: vec![vec![Cell::Black, Cell::White],
    ///                 vec![Cell::White, Cell::Black]],
    ///     row_spec: vec![vec![1], vec![2]],
    ///     col_spec: vec![vec![1], vec![1]],
    /// };
    ///
    /// assert!(!picross.is_valid());
    /// ```
    ///
    /// Invalid picross grid, with column specifications not respected:
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let picross = Picross {
    ///     height: 2,
    ///     length: 2,
    ///     cells: vec![vec![Cell::Black, Cell::White],
    ///                 vec![Cell::Black, Cell::Black]],
    ///     row_spec: vec![vec![1], vec![2]],
    ///     col_spec: vec![vec![2], vec![2]],
    /// };
    ///
    /// assert!(!picross.is_valid());
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        // Check basic consistency of `cells`
        if self.height != self.cells.len() || self.cells.iter().any(|r| self.length != r.len()) {
            return false;
        }

        // Check basic consistency of `(col|row)_spec`
        if self.height != self.row_spec.len() || self.length != self.col_spec.len() {
            return false;
        }

        // Prepare an iterator that iterates over both lines and columns, coupled to specs
        let iter =
            // Iterate over rows and its specs
            self.row_spec.iter().zip(
                self.cells.iter().cloned()
            )
        .chain(
            // Then iterate over columns and its specs
            self.col_spec.iter().zip(
                (0..self.length).map(|x| {
                    self.cells.iter()
                              .map(|r| r[x].clone())
                              .collect::<Vec<Cell>>()
                })
            )
        );

        // Check specs are matched
        for (spec, line) in iter {
            let mut num_block = 0;
            let mut size_block = 0;
            for c in line {
                match c {
                    Cell::Unknown => return false,
                    Cell::Black   => size_block += 1,
                    Cell::White   => {
                        if size_block > 0 {
                            if num_block >= spec.len() || size_block != spec[num_block] {
                                return false;
                            }
                            num_block += 1;
                            size_block = 0;
                        }
                    }
                }
            }
            if size_block > 0 {
                if num_block >= spec.len() || size_block != spec[num_block] {
                    return false;
                }
                num_block += 1;
            }
            if num_block != spec.len() {
                return false;
            }
        };

        true
    }

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

    ///
    /// /!\ Intended for internal use only /!\
    ///
    /// Transforms a specification into a vector of strings that can be used to
    /// represent the specification
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(
    ///     picross::Picross::specs_to_strings(&vec![vec![1, 2], vec![], vec![42]]),
    ///     vec!["1 2", "", "42"]
    /// );
    /// ```
    ///
    pub fn specs_to_strings(specs: &Vec<Vec<usize>>) -> Vec<String> {
        specs.iter()
             .map(|v| {
                 v.iter()
                  .map(|x| x.to_string())
                  .collect::<Vec<String>>()
                  .join(" ")
             })
             .collect()
    }

    ///
    /// /!\ Intended for internal use only /!\
    ///
    /// Return the maximum length of the strings in `specs`, assuming the Picross grid
    /// is not empty
    ///
    /// # Panics
    ///
    /// Panics if the picross grid whose `(row|col)_spec` is `specs` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(
    ///     picross::Picross::max_len_non_empty(&vec![
    ///         "123 2".to_string(),
    ///         "1".to_string(),
    ///         "".to_string(),
    ///         "124".to_string()
    ///     ]),
    ///     5
    /// );
    /// ```
    ///
    pub fn max_len_non_empty(specs: &Vec<String>) -> usize {
        specs.iter()
             .map(|x| x.len())
             .max()
             .expect("Not supporting empty picross grids!")
    }
}
