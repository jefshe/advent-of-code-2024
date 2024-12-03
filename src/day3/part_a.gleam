import gleam/int
import gleam/option
import gleam/regex

pub fn loop(match: List(regex.Match)) -> Int {
  case match {
    [m, ..rest] -> {
      let assert regex.Match(_, [option.Some(a), option.Some(b)]) = m
      let assert Ok(first) = int.parse(a)
      let assert Ok(second) = int.parse(b)
      first * second + loop(rest)
    }
    _ -> 0
  }
}

pub fn run(seq: String) {
  let assert Ok(re) = regex.from_string("mul\\((\\d+),(\\d+)\\)|")
  regex.scan(re, seq)
  |> loop
}
