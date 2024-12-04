fn safe_pair(a: Int, b: Int, is_desc) -> Bool {
  case is_desc {
    True -> a > b && a - b <= 3
    False -> a < b && b - a <= 3
  }
}

fn is_safe_impl(arr: List(Int), is_desc: Bool, can_remove: Bool) -> Bool {
  case arr, can_remove {
    [one, two, ..rest], False ->
      safe_pair(one, two, is_desc)
      && is_safe_impl([two, ..rest], is_desc, False)
    [one, two, ..rest], True ->
      safe_pair(one, two, is_desc)
      && is_safe_impl([two, ..rest], is_desc, True)
      || is_safe_impl([one, ..rest], is_desc, False)
    _, _ -> True
  }
}

pub fn is_safe(arr: List(Int)) -> Bool {
  case arr {
    [one, two, three, ..rest] ->
      is_safe_impl(arr, one > two, True)
      || is_safe_impl([one, three, ..rest], one > three, False)
      || is_safe_impl([two, three, ..rest], two > three, False)
    _ -> True
  }
}
