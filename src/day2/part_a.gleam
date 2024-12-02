import gleam/io
import utils/file

pub fn run() {
  let chunks = file.chunk_words("data/day2_ex.txt")
  io.debug(chunks)
}
