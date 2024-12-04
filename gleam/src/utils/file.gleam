import gleam/int
import gleam/list
import gleam/regex
import gleam/result
import gleam/string
import simplifile

pub fn chunk_ints(path: String) -> Result(List(List(Int)), Nil) {
  list.map(chunk_words(path), list.map(_, int.parse))
  |> list.map(result.all)
  |> result.all
}

pub fn chunk_words(path: String) -> List(List(String)) {
  let assert Ok(re) = regex.from_string("\\s+")
  let assert Ok(contents) = simplifile.read(path)
  string.split(contents, "\n")
  |> list.map(regex.split(re, _))
  |> list.filter(fn(line) {
    // remove empty lines
    case line {
      [] -> False
      [""] -> False
      _ -> True
    }
  })
}
