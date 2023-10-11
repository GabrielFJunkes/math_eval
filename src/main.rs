use math_eval::Parser;

fn main() {
    // Define expression as input String
    let input = String::from("(1 + 2*10)/(2+5)");
    // Create Parser with input String
    let mut parser = Parser::new(&input);
    // Calls parser.result() to evaluate the expression
    if let Some(item) = parser.result(){
        assert_eq!(item, "3");
    }
}
