use std::iter;
use std::fmt::{Formatter, Display, Result, Write};

use ::{Cell, Picross};

impl Picross {
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

impl Display for Picross {
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
    /// assert!(format!("{}", picross) == res);
    ///
    /// # picross.cells[0][0] = Cell::Black;
    /// # picross.cells[2][1] = Cell::White;
    /// # picross.cells[2][2] = Cell::White;
    /// # assert!(picross.is_valid());
    /// ```
    ///
    fn fmt(&self, f: &mut Formatter) -> Result {
        let row_spec = Picross::specs_to_strings(&self.row_spec);
        let col_spec = Picross::specs_to_strings(&self.col_spec);

        let max_rs_len = Picross::max_len_non_empty(&row_spec);
        let max_cs_len = Picross::max_len_non_empty(&col_spec);

        let line_begin = vec![" "; max_rs_len].join("");

        // Write the header: column specs
        for i in 0..max_cs_len {
            try!(f.write_str(&line_begin));
            try!(f.write_char('|'));
            for c in &col_spec {
                try!(f.write_char(c.chars().nth(max_cs_len - i - 1).unwrap_or(' ')));
            }
            try!(f.write_char('\n'));
        }

        // Write header separator
        try!(f.write_str(&iter::repeat('-').take(max_rs_len).collect::<String>()));
        try!(f.write_char('+'));
        try!(f.write_str(&iter::repeat('-').take(self.length).collect::<String>()));
        try!(f.write_char('\n'));

        for i in 0..self.height {
            // Write row specs
            try!(f.write_str(&iter::repeat(' ').take(max_rs_len - row_spec[i].len()).collect::<String>()));
            try!(f.write_str(&row_spec[i]));
            try!(f.write_char('|'));

            // Write actual content
            try!(f.write_str(&self.cells[i].iter().map(|c| match *c {
                Cell::Unknown => '?',
                Cell::White   => ' ',
                Cell::Black   => '#'
            }).collect::<String>()));

            // Okay, let's continue
            try!(f.write_char('\n'));
        }

        Ok(())
    }
}
