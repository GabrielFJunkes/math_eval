//! # Math_Eval
//!
//! A library for evaluating mathematical expressions.
//! This library uses Shunting Yard Algorithm for parsing math expressions in infix notations.

use std::str::Chars;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, PartialOrd)]
enum POp {
    Add,
    Sub,
    Mul,
    Div
}

impl POp {
    fn order(&self) -> i8 {
        match self {
            POp::Add => {1}
            POp::Sub => {1}
            POp::Mul => {2}
            POp::Div => {2}
        } 
    }
}

#[derive(Debug, PartialEq)]
enum BType{
    Left,
    Right
}

#[derive(Debug, PartialEq)]
enum PExpr {
    Op(POp),
    Cons(String), 
    Bracket(BType)
}

pub struct Parser<'a> {
    input: Chars<'a>,
    list: VecDeque<PExpr>,
    operation_stack: Vec<PExpr>,
    output_queue: VecDeque<PExpr>,
    brackets_num: usize
}

impl Parser<'_> {
    pub fn new(input_string: &String) -> Parser {
        let input_string = input_string.chars();
        Parser { 
            input: input_string, 
            list: VecDeque::new(), 
            operation_stack: vec![], 
            output_queue: VecDeque::new(), 
            brackets_num: 0
        }
    }
    fn next(&mut self) -> Option<char> {
        self.input.next()
    }
    fn clear_buffer(&mut self, expr: &mut String){
        if !expr.is_empty() {
            self.list.push_back(PExpr::Cons(expr.clone()));
            expr.clear();
        }
    }
    fn should_no_be(&mut self, expr: PExpr) {
        if let Some(last) = self.list.back() {
            match (last, expr) {
                (PExpr::Cons(_), PExpr::Cons(_)) => {
                    panic!("Error: two numbers without operator in between")
                }
                (PExpr::Op(_), PExpr::Op(_)) => {
                    panic!("Error: operator followed by another operator")
                }
                (PExpr::Bracket(BType::Right), PExpr::Bracket(BType::Right)) => {
                    panic!("Error: missing operator after closing bracket")
                }
                _ => {}
            }
        }
    }
    fn parse(&mut self) {
        let mut expr: String = Default::default();
        loop {
            if let Some(token) = self.next() {
                match token {
                    '0'..='9' => {
                        self.should_no_be(PExpr::Cons("".to_string()));
                        self.should_no_be(PExpr::Bracket(BType::Right));
                        expr.push(token);
                    }
                    '.' => {
                        if expr.is_empty() {
                            panic!("Error: could not parse alone dot")
                        }
                        if expr.contains('.') {
                            panic!("Error: could not parse alone dot")
                        }
                        expr.push(token);
                    }
                    ' ' => {
                        self.clear_buffer(&mut expr);
                    }
                    '+' => {
                        self.clear_buffer(&mut expr);
                        self.should_no_be(PExpr::Op(POp::Add));
                        self.list.push_back(PExpr::Op(POp::Add));
                    }
                    '-' => {
                        self.clear_buffer(&mut expr);
                        if let Some(last) = self.list.back() {
                            match last {
                                PExpr::Bracket(BType::Left) => {
                                    self.list.push_back(PExpr::Cons("0".to_string()));
                                }
                                _ => {}
                            }
                        }else{
                            self.list.push_back(PExpr::Cons("0".to_string()));
                        }
                        self.list.push_back(PExpr::Op(POp::Sub));
                    }
                    '*' => {
                        self.clear_buffer(&mut expr);
                        self.should_no_be(PExpr::Op(POp::Add));
                        self.list.push_back(PExpr::Op(POp::Mul));
                    }
                    '/' => {
                        self.clear_buffer(&mut expr);
                        self.should_no_be(PExpr::Op(POp::Add));
                        self.list.push_back(PExpr::Op(POp::Div));
                    }
                    '(' => {
                        self.brackets_num+=1;
                        self.clear_buffer(&mut expr);
                        if let Some(last) = self.list.back() {
                            match last {
                                PExpr::Cons(_) => {
                                    self.list.push_back(PExpr::Op(POp::Mul));
                                }
                                PExpr::Bracket(BType::Right) => {
                                    self.list.push_back(PExpr::Op(POp::Mul));
                                }
                                _ => {}
                            }
                        }
                        self.list.push_back(PExpr::Bracket(BType::Left));
                    }
                    ')' => {
                        self.brackets_num-=1;
                        self.clear_buffer(&mut expr);
                        self.list.push_back(PExpr::Bracket(BType::Right));
                    }
                    c => {panic!("Error: Char '{}' not recognized!", c)}
                }
            }else{
                break;
            }
        }
        self.clear_buffer(&mut expr);
        if self.brackets_num != 0 {
            panic!("Error: brackets number not matching")
        }
    }

    fn shunting(&mut self) {
        loop {
            if let Some(token) = self.list.pop_front(){
                match token {
                    PExpr::Cons(_) => {
                        self.output_queue.push_back(token);
                    }
                    PExpr::Op(ref operation) => {
                        if !self.operation_stack.is_empty(){
                            loop {
                                if let Some(PExpr::Op(last_op)) = self.operation_stack.last() {
                                    if last_op.order()>operation.order() {
                                        let Some(last_op) = self.operation_stack.pop() else {
                                            panic!("Error poping op from op stack")
                                        };
                                        self.output_queue.push_back(last_op);
                                    }else{
                                        break;
                                    }
                                }else{
                                    break;
                                }
                            }
                        }
                        self.operation_stack.push(token)
                    }
                    PExpr::Bracket(BType::Left) => {
                        self.operation_stack.push(token)
                    }
                    PExpr::Bracket(BType::Right) => {
                        loop {
                            if let Some(op) = self.operation_stack.pop() {    
                                if op != PExpr::Bracket(BType::Left) {
                                    self.output_queue.push_back(op);
                                }else{
                                    break;
                                }
                            }else{
                                break;
                            }
                        }
                    }
                }
            }else{
                break;
            }
        }
        while !self.operation_stack.is_empty() {
            if let Some(op) = self.operation_stack.pop(){
                self.output_queue.push_back(op);
            }
        }
    }

    fn solve_by_index(&mut self, i: usize) {
        let Some(current) = self.output_queue.get(i) else {panic!("Error: could not get third elem of queue")};
        match current {
            PExpr::Op(operation) => {
                let Some(left) = self.output_queue.get(i-2) else {panic!("Error: expected number")};
                let Some(right) = self.output_queue.get(i-1) else {panic!("Error: expected number")};

                let PExpr::Cons(num_string) = left else { panic!("Error: expected number") };
                let num_left: f32 = num_string.parse::<f32>().unwrap();
                let PExpr::Cons(num_string) = right else { panic!("Error: expected number") };
                let num_right: f32 = num_string.parse::<f32>().unwrap();

                let result: f32;
                match operation {
                    POp::Add => {result = num_left + num_right}
                    POp::Sub => {result = num_left - num_right}
                    POp::Mul => {result = num_left * num_right}
                    POp::Div => {result = num_left / num_right}
                }
                
                self.output_queue.insert(i-2, PExpr::Cons(result.to_string()));
                self.output_queue.drain(i-1..=i+1);
            }
            _ => {}
        }
    }

    fn resolve(&mut self) {
        self.parse();
        self.shunting();
        while self.output_queue.len()>2 {
            let mut i = 2;
            loop {
                let Some(current) = self.output_queue.get(i) else { panic!("Error: could not resolve") };
                match current {
                    PExpr::Op(_) => { self.solve_by_index(i); i = i-2;}
                    _ => {i+=1;}
                }
                if i == self.output_queue.len() { break }
            }
        }
    }

    pub fn result(&mut self) -> Option<String> {
        self.resolve();
        if let Some(item) = self.output_queue.get(0) {
            match item {
                PExpr::Cons(number) => {
                    return Some(number.to_string());
                }
                _ => {return None; }
            }
        }
        None
    }
}