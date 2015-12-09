
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
    /// Returns an integer parsed from data
    ///
    /// # Panics
    ///
    /// Panics if `data` is not actually a `Some(&str)` representing an integer, with `name`
    /// in the error message to make things clearer.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(42, picross::Picross::get_integer(Some("42"), "test"));
    /// ```
    ///
    /// The following will panic:
    ///
    /// ```should_panic
    /// picross::Picross::get_integer(None, "testing number");
    /// // With error message "Expected to find a testing number!"
    /// ```
    ///
    /// ```should_panic
    /// picross::Picross::get_integer(Some("not an integer"), "stuff");
    /// // With error message "Expected integer stuff!"
    /// ```
    ///
    pub fn get_integer(data: Option<&str>, name: &str) -> usize {
        data.expect(&format!("Expected to find a {}!", name))
            .parse().ok().expect(&format!("Expected integer {}!", name))
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
    pub fn get_specs(s: &str) -> Vec<usize> {
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
    /// /!\ Intended for internal use only /!\
    ///
    /// Reads a `(row|col)_spec` of `size` elements into `specs` from `data`
    ///
    /// # Panics
    ///
    /// Panics if `data` does not have at least `size` elements to give, or if one of these
    /// elements does not comply with the `get_specs` specification.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut specs = vec![];
    /// picross::Picross::fill_specs(3, &mut specs, &mut vec!["[]", "[1,3]", "[42]"].into_iter());
    /// assert_eq!(specs, vec![vec![], vec![1, 3], vec![42]]);
    /// ```
    ///
    /// The following examples should panic:
    ///
    /// ```should_panic
    /// let mut specs = vec![];
    /// picross::Picross::fill_specs(3, &mut specs, &mut vec!["[]", "[1,3]"].into_iter());
    /// ```
    ///
    /// ```should_panic
    /// let mut specs = vec![];
    /// picross::Picross::fill_specs(3, &mut specs, &mut vec!["[]", "blah", "[42]"].into_iter());
    /// ```
    ///
    pub fn fill_specs(size: usize, specs: &mut Vec<Vec<usize>>, data: &mut Iterator<Item=&str>) {
        for _ in 0..size {
            specs.push(Picross::get_specs(data.next().expect("Wrong number of specifications!")));
        }
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

    ///
    /// Converts a Picross grid into a String
    ///
    /// # Panics
    ///
    /// Panics if `height` or `length` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let mut picross = Picross {
    ///     height: 3,
    ///     length: 3,
    ///     row_spec: vec![vec![1, 1], vec![1], vec![1]],
    ///     col_spec: vec![vec![1, 1], vec![], vec![2]],
    ///     cells: vec![vec![Cell::Unknown, Cell::White  , Cell::Black],
    ///                 vec![Cell::White  , Cell::White  , Cell::Black],
    ///                 vec![Cell::Black  , Cell::Unknown, Cell::Unknown]],
    /// };
    ///
    /// let res =
    ///     "   |1  \n".to_string() +
    ///     "   |   \n" +
    ///     "   |1 2\n" +
    ///     "---+---\n" +
    ///     "1 1|? #\n" +
    ///     "  1|  #\n" +
    ///     "  1|#??\n";
    ///
    /// assert!(picross.to_string() == res);
    ///
    /// # picross.cells[0][0] = Cell::Black;
    /// # picross.cells[2][1] = Cell::White;
    /// # picross.cells[2][2] = Cell::White;
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub fn to_string(&self) -> String {
        let row_spec = Picross::specs_to_strings(&self.row_spec);
        let col_spec = Picross::specs_to_strings(&self.col_spec);

        let max_rs_len = Picross::max_len_non_empty(&row_spec);
        let max_cs_len = Picross::max_len_non_empty(&col_spec);

        let line_begin = vec![" "; max_rs_len].join("");

        let mut res: String = "".to_string();

        // Write the header: column specs
        for i in 0..max_cs_len {
            res = res + &line_begin + "|";
            for c in &col_spec {
                res.push(c.chars().nth(max_cs_len - i - 1).unwrap_or(' '));
            }
            res.push('\n');
        }

        // Write header separator
        res = res + &vec!["-"; max_rs_len].join("") + "+" + &vec!["-"; self.length].join("") + "\n";

        for i in 0..self.height {
            // Write row specs
            res = res + &vec![" "; max_rs_len - row_spec[i].len()].join("") + &row_spec[i] + "|";

            // Write actual content
            res = res + &self.cells[i].iter().map(|c| match *c {
                Cell::Unknown => '?',
                Cell::White   => ' ',
                Cell::Black   => '#'
            }).collect::<String>();

            // Okay, let's continue
            res.push('\n');
        }

        res
    }
}
