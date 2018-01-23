pub fn reverse(thestring: &str) -> String {
    let mut result = String::from("");
    for c in thestring.chars() {
      let char = c.to_string();
      result = format!("{}{}", char, result);
    }
    return result;
}
