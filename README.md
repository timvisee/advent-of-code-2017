# Advent of Code 2017
My go on the advent of code challenge in 2017.

This project includes a runner/manager,
that allows easy selection of the solution to run.

This runner automatically reads challenge input from the appropriate files,
and allows easy output saving and selection.

This runner is also useful to add new solutions for challenges without much
effort.

## Requirements
- `Rust 1.20`

## Usage
```
# First, clone the project
git clone https://github.com/timvisee/advent-of-code-2017 advent-of-code-2017
cd advent-of-code-2017

# Build the tool using Rust Cargo
cargo build --release
cd target/release

# Run the tool
./advent-of-code-2017-runner
./advent-of-code-2017-runner --help
./advent-of-code-2017-runner --day 1 --part 2 --save

# Or run it directly though Cargo
cargo run --release -- --day 1 --part 2 --save
```

## Resources
[Advent of Code website](https://adventofcode.com/2017)

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
