import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{None, Some}
import gleam/regex
import gleam/result
import gleam/string
import simplifile
import utils/counter

fn split_lists(strings: List(String)) {
  let assert Ok(re) = regex.from_string("\\s+")
  list.map(strings, regex.split(re, _))
  |> list.map(list.map(_, int.parse))
  |> list.map(result.all)
  |> result.all
}

fn part_a(strings: List(String)) -> Int {
  let split = split_lists(strings)
  let assert [a, b] = case split {
    Ok(parsed) -> parsed |> list.transpose
    Error(_) -> [[], []]
  }

  let sorted_pairs =
    [list.sort(a, int.compare), list.sort(b, int.compare)] |> list.transpose

  list.fold(sorted_pairs, 0, fn(acc, row) {
    case row {
      [a, b] -> acc + int.absolute_value(a - b)
      _ -> acc
    }
  })
}

fn part_b(strings: List(String)) -> Int {
  let assert Ok([keys, values]) =
    split_lists(strings) |> result.map(list.transpose)
  let counts = counter.counter(values)

  list.fold(keys, 0, fn(acc, key) {
    case dict.get(counts, key) {
      Ok(count) -> acc + key * count
      Error(_) -> acc
    }
  })
}

pub fn run() {
  let assert Ok(output) =
    simplifile.read("data/day1_a.txt")
    |> result.map(string.split(_, "\n"))
    |> result.map(part_b)
  io.debug(output)
}
