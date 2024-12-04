import gleam/dict
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/regex
import gleam/string
import simplifile

fn increment(x: Option(Int)) -> Int {
  case x {
    Some(i) -> i + 1
    None -> 1
  }
}

pub fn counter(x: List(any)) -> dict.Dict(any, Int) {
  let dict = dict.from_list([])
  list.fold(x, dict, fn(dict, key) { dict.upsert(dict, key, increment) })
}
