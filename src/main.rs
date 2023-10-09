use math_eval::Parser;

fn main() {
    let input = String::from("-(3+2*2*2*9)/(75)");
    let mut parser = Parser::new(&input);
    if let Some(item) = parser.result(){
        println!("{} = {}", input, item);
    }
    
}
