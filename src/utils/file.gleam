import gleam/bool
import gleam/list
import gleam/regex
import gleam/string
import simplifile

pub fn chunk_words(path: String) -> List(List(String)) {
  let assert Ok(re) = regex.from_string("\\s+")
  let assert Ok(contents) = simplifile.read(path)
  string.split(contents, "\n")
  |> list.map(regex.split(re, _))
  |> list.filter(fn(line) { !list.is_empty(line) })
}
