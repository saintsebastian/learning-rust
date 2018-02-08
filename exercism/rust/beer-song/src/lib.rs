pub fn verse(n: i32) -> String {
    match n > 1 {
    false => match n {
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
      },
    true => make_verse(n)
    }
}

fn make_verse(n: i32) -> String {
    let last = match n {
        2 => String::from(" bottle of beer on the wall.\n"),
        _ => String::from(" bottles of beer on the wall.\n"),
    };
    let first = [ n.to_string(), String::from(" bottles of beer"), String::from(" on the wall"), String::from(", "), n.to_string()].concat();
    let result = [first, String::from(" bottles of beer"), String::from(".\nTake one down and pass it around, "), (n-1).to_string(), last].concat();
    result
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = Vec::<String>::new();
    for _number in (end..start+1).rev() {
        println!("{}!", _number.to_string());
        song.push(verse(_number));
    }
    let full = song.join("\n");
    full
}
