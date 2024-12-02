fn safe_pair(a: Int, b: Int, is_desc) -> Bool {
  case is_desc {
    True -> a > b && a - b <= 3
    False -> a < b && b - a <= 3
  }
}

fn is_safe_impl(arr: List(Int), is_desc: Bool) -> Bool {
  case arr {
    [one, two, ..rest] ->
      safe_pair(one, two, is_desc) && is_safe_impl([two, ..rest], is_desc)
    _ -> True
  }
}

pub fn is_safe(arr: List(Int)) -> Bool {
  case arr {
    [one, two, ..rest] -> is_safe_impl([one, two, ..rest], one > two)
    _ -> True
  }
}
