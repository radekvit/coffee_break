fn fizz_buzz(number: i32) {
    for i in 1..=number {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    // You've been solving difficult problems all day
    fizz_buzz(100);

    // Have a break while compiling
    coffee_break::coffee_break!(40 minutes);
}
