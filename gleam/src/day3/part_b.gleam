import gleam/int
import gleam/option
import gleam/regex

pub fn loop(matches: List(regex.Match), enabled: Bool) -> Int {
  case matches {
    [m, ..rest] -> {
      case m, enabled {
        regex.Match("don't()", _), _ -> loop(rest, False)
        regex.Match("do()", _), _ -> loop(rest, True)
        _, False -> loop(rest, enabled)
        _, True -> {
          let assert regex.Match(_, [option.Some(a), option.Some(b)]) = m
          let assert Ok(first) = int.parse(a)
          let assert Ok(second) = int.parse(b)
          first * second + loop(rest, enabled)
        }
      }
    }
    _ -> 0
  }
}

pub fn run(seq: String) {
  let assert Ok(re) =
    regex.from_string("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)")
  regex.scan(re, seq)
  |> loop(True)
}
