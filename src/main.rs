mod fizz_buzz;

use fizz_buzz::fizz_buzz;

fn main() {
    println!("Hello, world!");
    for i in 1..101 {
        println!("{}", fizz_buzz(i));
    }
}
