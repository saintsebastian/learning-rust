pub fn raindrops(n: i32) -> String {
    let mut result = "".to_owned();
    if n%3 == 0 {
        result = result + "Pling";
    }
    if n%5 == 0 {
        result = result + "Plang";
    }
    if n%7 == 0 {
        result = result + "Plong";
    }
    if result != "" {
        result
    } else {
        n.to_string()
    }
}
