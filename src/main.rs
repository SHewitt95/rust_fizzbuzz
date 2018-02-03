/// Prints numbers depending on their divisibility with
/// passed-in arguments.
/// 
/// # Goal
/// 
/// Print "Fizz" when a number is divisible by `first_num`.
/// Print "Buzz" when a number is divisible by `second_num`.
/// Print "FizzBuzz" when a number is divisible by `first_num`
/// and `second_num`.
/// 
/// Otherwise, just print out the number.
/// 
/// # Arguments
/// * `first_num`: i32 variable. This is one of the numbers used
/// to determine divisibiity. Will be used for "Fizz."
/// 
/// * `second_num`: i32 variable. This is one of the numbers
/// used to determine divisibility. Will be used for "Buzz."
/// 
/// * `max`: i32 variable. This determines the highest number
/// (exclusively) `fizzbuzz` will calculate.
fn fizzbuzz(first_num: i32, second_num: i32, max: i32) {
    for x in 0..max {
        if (x % (first_num * second_num) == 0) & (x > 0) {
            println!("FizzBuzz: {}", x);
        } else if (x % first_num == 0) & (x > 0) {
            println!("Fizz: {}", x);
        } else if (x % second_num == 0) & (x > 0) {
            println!("Buzz: {}", x);
        } else {
            println!("{}", x);
        }
    }
}

fn main() {
    fizzbuzz(2, 7, 100);
}
