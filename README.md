# lines_writer
This crate lets you write an iterator into a LinesWriter, with each item on a line.

The trait `LinesWriter` is implemented for all the types that implement `std::io::Write`.
Simply call write_lines on the writer and pass an iterator as argument.

# Examples

```rust
use lines_writer::LinesWriter;

fn main() {
    let str = "This is\na string\nto show\nhow do\nthis work";
    std::io::stdout().write_lines(str.lines()).unwrap();
}
```

```rust
use lines_writer::LinesWriter;

fn main() {
    let array = [10, 15, 63, 29];
    std::io::stdout().write_lines(&array).unwrap();
}
```

## Contributing
Feel free to contribute to this project with pull requests and/or issues.
All contribution should be under a license compatible with the GNU LGPLv3.

## Changes
0.1.0 First release
