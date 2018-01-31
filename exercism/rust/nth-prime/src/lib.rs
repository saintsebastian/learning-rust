pub fn nth(target: u32) -> Result<u64, String> {
    match target > 0 {
    true => match target {
        1 => Ok(2),
        2 => Ok(3),
        _ => Ok(trial(target))
    },
    false => Err(String::from("invalid input")),
  }
}

fn trial(target: u32) -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(target as usize);
    primes.push(2);
    primes.push(3);
    let mut next_checked = *primes.last().unwrap() + 2;
    while primes.len() < target as usize {
      if primes.iter().all(|&i| next_checked % i != 0) {
          primes.push(next_checked)
      }
     next_checked += 2;
    }
    *primes.last().unwrap()
}
