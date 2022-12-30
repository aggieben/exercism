#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    let mut stack : Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            Add => {
                if stack.len() >= 2 {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op1 + op2);
                }
            },
            Subtract => {
                if stack.len() >= 2 {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 - op1);
                }
            },
            Multiply => {
                if stack.len() >= 2 {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 * op1);
                }
            },
            Divide => {
                if stack.len() >= 2 {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    stack.push(op2 / op1);
                }
            },
            Value(v) => stack.push(*v)
        }
    }
    stack.pop()
}
