use std::collections::HashMap;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug, Clone)]
enum Expression {
    CONST(i64),
    COMPOUND(CompoundExpression),
    VAR,
    // ID(String),
}

#[derive(Debug, Clone)]
struct CompoundExpression {
    operation: Operation,
    // lhs: Box<Expression>,
    // rhs: Box<Expression>,
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

fn parse_input(input: &str) -> HashMap<String, Expression> {
    let mut res = HashMap::<String, Expression>::new();
    for line in input.lines() {
        if line.len() > 0 {
            let (name, tail) = line.split_once(": ").unwrap();
            let expr: Expression;
            if !tail.contains(' ') {
                expr = Expression::CONST(tail.trim().parse::<i64>().unwrap());
            } else {
                let (left, tail) = tail.split_once(" ").unwrap();
                let (op, right) = tail.split_once(" ").unwrap();
                let op: Operation = match op {
                    "+" => Operation::ADD,
                    "-" => Operation::SUB,
                    "*" => Operation::MUL,
                    "/" => Operation::DIV,
                    _ => {
                        panic!("unknown op!");
                    }
                };
                expr = Expression::COMPOUND(CompoundExpression {
                    operation: op,
                    lhs: String::from(left),
                    rhs: String::from(right),
                });
            }

            res.insert(String::from(name), expr);
        }
    }
    res
}

fn evaluate(expressions: &mut HashMap<String, Expression>, target: &str) -> i64 {
    maybe_evaluate(expressions, target).unwrap()
}

fn maybe_evaluate(expressions: &mut HashMap<String, Expression>, target: &str) -> Option<i64> {
    let expr = expressions.get(target).unwrap().clone();
    match expr {
        Expression::CONST(val) => Some(val),
        Expression::COMPOUND(CompoundExpression {
            operation,
            lhs,
            rhs,
        }) => {
            let lhs_val = maybe_evaluate(expressions, &lhs)?;
            let rhs_val = maybe_evaluate(expressions, &rhs)?;
            let res = match operation {
                Operation::ADD => lhs_val + rhs_val,
                Operation::SUB => lhs_val - rhs_val,
                Operation::MUL => lhs_val * rhs_val,
                Operation::DIV => lhs_val / rhs_val,
            };
            expressions.insert(String::from(target), Expression::CONST(res));
            Some(res)
        }
        Expression::VAR => None,
    }
}

// This function assumes that only one branch of each expression has a dependence on the
// variable 'humn'. Also performs some sketchy math / assumptions w.r.t. div / mul.
fn solve(expressions: &mut HashMap<String, Expression>, target: &str, target_value: i64) -> i64 {
    let target_expr = expressions.get(target).unwrap().clone();
    if let Expression::VAR = target_expr {
        return target_value;
    }
    if let Expression::CONST(expr_val) = target_expr {
        if target_value != expr_val {
            panic!("No solution.");
        }
        return expr_val;
    }
    if let Expression::COMPOUND(CompoundExpression {
        operation,
        lhs,
        rhs,
    }) = target_expr
    {
        let lhs_maybe = maybe_evaluate(expressions, &lhs);
        if !lhs_maybe.is_none() {
            let lhs_val = lhs_maybe.unwrap();
            let new_target = match operation {
                Operation::ADD => target_value - lhs_val,
                Operation::SUB => lhs_val - target_value,
                Operation::MUL => {
                    if target_value % lhs_val != 0 {
                        panic!("Invalid mul result.");
                    }
                    target_value / lhs_val
                }
                Operation::DIV =>
                // Sketchy...
                {
                    lhs_val / target_value
                }
            };
            return solve(expressions, &rhs, new_target);
        }
        let rhs_maybe = maybe_evaluate(expressions, &rhs);
        if rhs_maybe.is_none() {
            panic!("Both rhs and lhs contain the variable!");
        }
        let rhs_val = rhs_maybe.unwrap();
        let new_target = match operation {
            Operation::ADD => target_value - rhs_val,
            Operation::SUB => target_value + rhs_val,
            Operation::MUL => {
                if target_value % rhs_val != 0 {
                    panic!("Invalid mul result.");
                }
                target_value / rhs_val
            }
            Operation::DIV =>
            // Sketchy...
            {
                rhs_val * target_value
            }
        };
        return solve(expressions, &lhs, new_target);
    }
    panic!("wtf");
}

fn main() {
    let input = get_input();
    let expressions = parse_input(&input);
    let mut expressions_p1 = expressions.clone();

    println!("P1: {}", evaluate(&mut expressions_p1, "root"));

    let mut expressions_p2 = expressions.clone();
    *expressions_p2.get_mut("humn").unwrap() = Expression::VAR {};
    if let Expression::COMPOUND(CompoundExpression {
        operation: _,
        lhs,
        rhs,
    }) = expressions_p2.get("root").unwrap().clone()
    {
        expressions_p2.insert(
            String::from("root"),
            Expression::COMPOUND(CompoundExpression {
                operation: Operation::SUB,
                lhs: lhs,
                rhs: rhs,
            }),
        );
    }
    let res = solve(&mut expressions_p2, "root", 0);
    dbg!(res);
}
