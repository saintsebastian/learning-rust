pub fn reply (q: &'static str) -> &'static str{
  let q = q.trim();
  if q.is_empty() {
    "Fine. Be that way!"
  } else if q.chars().last() == Some('?') {
    "Sure."
  } else if q.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()){
    "Whoa, chill out!"
  }else {
    "Whatever."
  }
}
