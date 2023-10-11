use math_eval::Parser;

fn main() {
    let input = String::from("(1 + 2*10)/(2+5)");
    let mut parser = Parser::new(&input);
    if let Some(item) = parser.result(){
        println!("{} = {}", input, item);
    }
    
}
