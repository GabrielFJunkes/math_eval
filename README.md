# math_eval
Personal project to learn Rust and develop a parsing algorithm.

The core of this project is the implementation of the Shunting Yard Algorithm, which is for parsing mathematical expressions written in infix notation.
I implemented some tests to check if any new changes break the already defined rules. You can run the tests with `cargo test`.

Code example:
```Rust
use math_eval::Parser;

fn main() {
    let input = String::from("(1 + 2*10)/(2+5)");
    let mut parser = Parser::new(&input);
    if let Some(item) = parser.result(){
        println!("{} = {}", input, item);
    }
    
}
```
Running `cargo run` the result is:

`(1 + 2*10)/(2+5) = 3`
