use math_eval::Parser;

#[test]
fn one_number() {
    let input = String::from("1");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("1".to_string()), parser.result());
}

#[test]
fn one_negative_number() {
    let input = String::from("-1");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("-1".to_string()), parser.result());
}

#[test]
fn negate_brackets() {
    let input = String::from("-(2+3*1)");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("-5".to_string()), parser.result());
}

#[test]
fn double_negate_brackets() {
    let input = String::from("-(-3)");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("3".to_string()), parser.result());
}

#[test]
fn negative_multiplication() {
    let input = String::from("2 * (-1)");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("-2".to_string()), parser.result());
}

#[test]
fn simple_sum() {
    let input = String::from("1 + 2");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("3".to_string()), parser.result());
}

#[test]
fn multiple_with_brackets() {
    let input = String::from("2(1 + 2)");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("6".to_string()), parser.result());
}

#[test]
fn multiple_two_brackets() {
    let input = String::from("(1 + 2)(2)");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("6".to_string()), parser.result());
}

#[test]
fn checks_correct_operator_order() {
    let input = String::from("2(1 + 2)/2+2");
    let mut parser = Parser::new(&input);
    assert_eq!(Some("5".to_string()), parser.result());
}

#[test]
#[should_panic(expected = "Error: operator followed by another operator")]
fn two_operators_without_number_in_between() {
    let input = String::from("2++2");
    let mut parser = Parser::new(&input);
    parser.result();
}

#[test]
#[should_panic(expected = "Error: two numbers without operator in between")]
fn two_numbers_without_operator_in_between() {
    let input = String::from("2 2");
    let mut parser = Parser::new(&input);
    parser.result();
}

#[test] 
#[should_panic(expected= "Error: missing operator after closing bracket")]
fn missing_operator_after_closing_bracket() {
    let input = String::from("(2)2");
    let mut parser = Parser::new(&input);
    parser.result();
}

#[test] 
#[should_panic(expected= "Error: Char 'x' not recognized!")]
fn char_not_recognized() {
    let input = String::from("2 + x");
    let mut parser = Parser::new(&input);
    parser.result();
}

