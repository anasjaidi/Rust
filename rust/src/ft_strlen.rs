fn ft_strlen(str: String) -> u32 {
  let mut i : u32 = 0;
  for c in str.chars() {
    i += 1;
  }
  i
}