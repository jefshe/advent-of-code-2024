import day2/part_b
import gleam/io
import gleam/list
import gleam/result
import utils/file

pub fn run() {
  use chunks <- result.map(file.chunk_ints("data/day2_a.txt"))
  chunks
  |> list.filter(part_b.is_safe)
  |> list.length
  |> io.debug
}
