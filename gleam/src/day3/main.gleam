import day3/part_b
import gleam/io
import simplifile

pub fn run() {
  let assert Ok(contents) = simplifile.read("data/day3_a.txt")
  part_b.run(contents)
  |> io.debug
}
