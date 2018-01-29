fn main() {
    fib_function(40);
}

fn fib_function(x: i32) {
    let mut penultimate: i32 = 0;
    let mut previous: i32 = 1;
    for _number in 1..x {
        let current = previous + penultimate;
        println!("{}!", current);
        penultimate = previous;
        previous = current;
    }
}
