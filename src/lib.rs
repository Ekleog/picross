pub mod display;
pub mod parse;

/// The Cell type
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Unknown,
    Black,
    White,
}

/// A Picross board
#[derive(Clone, Debug)]
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
    /// Sets row `row` to values `vals`
    ///
    /// # Examples
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let mut picross = Picross {
    ///     height: 3,
    ///     length: 3,
    ///     cells: vec![vec![Cell::Black, Cell::Black, Cell::Black],
    ///                 vec![Cell::Unknown, Cell::Unknown, Cell::White],
    ///                 vec![Cell::Black, Cell::White, Cell::Black]],
    ///     row_spec: vec![vec![3], vec![1], vec![1, 1]],
    ///     col_spec: vec![vec![1, 1], vec![2], vec![1, 1]],
    /// };
    ///
    /// picross.set_row(1, vec![Cell::White, Cell::Black, Cell::White]);
    ///
    /// assert!(picross.is_valid());
    /// ```
    ///
    pub fn set_row(&mut self, row: usize, vals: Vec<Cell>) {
        self.cells[row] = vals;
    }

    ///
    /// Sets col `col` to values `vals`
    ///
    /// # Examples
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let mut picross = Picross {
    ///     height: 3,
    ///     length: 3,
    ///     cells: vec![vec![Cell::Unknown, Cell::Black, Cell::Black],
    ///                 vec![Cell::White, Cell::Black, Cell::White],
    ///                 vec![Cell::Unknown, Cell::White, Cell::Black]],
    ///     row_spec: vec![vec![3], vec![1], vec![1, 1]],
    ///     col_spec: vec![vec![1, 1], vec![2], vec![1, 1]],
    /// };
    ///
    /// picross.set_col(0, vec![Cell::Black, Cell::White, Cell::Black]);
    ///
    /// assert!(picross.is_valid());
    /// ```
    ///
    pub fn set_col(&mut self, col: usize, vals: Vec<Cell>) {
        for i in 0..self.length {
            self.cells[i][col] = vals[i];
        }
    }

    ///
    /// Transposes the cells
    ///
    /// # Examples
    ///
    /// ```
    /// use picross::{Picross, Cell};
    ///
    /// let mut picross = Picross {
    ///     height: 3,
    ///     length: 3,
    ///     cells: vec![vec![Cell::Black, Cell::Black, Cell::Unknown],
    ///                 vec![Cell::White, Cell::Black, Cell::White],
    ///                 vec![Cell::Black, Cell::White, Cell::Black]],
    ///     row_spec: vec![vec![3], vec![1], vec![1, 1]],
    ///     col_spec: vec![vec![1, 1], vec![2], vec![1, 1]],
    /// };
    ///
    /// assert_eq!(
    ///     picross.transpose(),
    ///     vec![vec![Cell::Black, Cell::White, Cell::Black],
    ///          vec![Cell::Black, Cell::Black, Cell::White],
    ///          vec![Cell::Unknown, Cell::White, Cell::Black]]
    /// );
    /// # picross.cells[0][2] = Cell::Black;
    /// # assert!(picross.is_valid());
    /// ```
    ///
    pub fn transpose(&self) -> Vec<Vec<Cell>> {
        (0..self.length).map(|x| {
            self.cells.iter()
                      .map(|r| r[x].clone())
                      .collect::<Vec<Cell>>()
        }).collect::<Vec<Vec<Cell>>>()
    }

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
        let transpose = self.transpose();
        let iter = self.row_spec.iter().zip(self.cells.iter())
            .chain(self.col_spec.iter().zip(transpose.iter()));

        // Check specs are matched
        for (spec, line) in iter {
            let mut num_block = 0;
            let mut size_block = 0;
            for c in line {
                match c {
                    &Cell::Unknown => return false,
                    &Cell::Black   => size_block += 1,
                    &Cell::White   => {
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
}
