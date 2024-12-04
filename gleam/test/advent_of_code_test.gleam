import day2/part_b
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

pub fn part_b_test() {
  part_b.is_safe([1, 2, 3, 4, 5]) |> should.equal(True)
}
