/*
 *  Copyright © 2019 Gianmarco Garrisi
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
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

use std::io::{Result, Write};
use std::fmt::Display;

/// This trait lets you write an iterator into a writer, with each item on a line.
pub trait LinesWriter {
    /// write the items of the iterator onto self
    fn write_lines<I>(&mut self, iterator: I) -> Result<usize>
    where I: IntoIterator,
    I::Item: Display;
}

impl<T> LinesWriter for T
where T: Write {
    fn write_lines<I>(&mut self, iterator: I) -> Result<usize> 
    where I: IntoIterator,
    I::Item: Display {
        let mut res = 0;
        for el in iterator {
            writeln!(self, "{}", el)?;
            res+=1;
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
