/*
 *  Copyright © 2019 Gianmarco Garrisi
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! This crate lets you write an iterator into a LinesWriter, with each item on a line.
//!
//! The trait is implemented for all the types that implement `std::io::Write`.
//! Simply call write_lines on the writer and pass an iterator as argument.
//!
//! # Examples
//!
//! ```rust
//! use lines_writer::LinesWriter;
//!
//! fn main() {
//!     let str = "This is\na string\nto show\nhow do\nthis work";
//!     std::io::stdout().write_lines(str.lines()).unwrap();
//! }
//! ```
//!
//! ```rust
//! use lines_writer::LinesWriter;
//!
//! fn main() {
//!     let array = [10, 15, 63, 29];
//!     std::io::stdout().write_lines(&array).unwrap();
//! }
//! ```

use std::fmt::Display;
use std::io::{Result, Write};

/// This trait lets you write an iterator into its implementor with each item on a line.
pub trait LinesWriter {
    /// write the items of the iterator onto self
    fn write_lines<I>(&mut self, iterator: I) -> Result<usize>
    where
        I: IntoIterator,
        I::Item: Display;
}

impl<T> LinesWriter for T
where
    T: Write,
{
    fn write_lines<I>(&mut self, iterator: I) -> Result<usize>
    where
        I: IntoIterator,
        I::Item: Display,
    {
        let mut res = 0;
        for el in iterator {
            writeln!(self, "{}", el)?;
            res += 1;
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::LinesWriter;
        let str = "This is\na string\nto test\nthe trait implementation";
        let n = std::io::stdout().write_lines(str.lines()).unwrap();
        assert_eq!(n, 4);
    }
}
