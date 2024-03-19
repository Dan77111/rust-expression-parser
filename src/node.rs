use std::error::Error;

const OPERATORS: &str = "+-*/^";

pub struct Node {
    pub value: String,
    pub l_child: Option<Box<Node>>,
    pub r_child: Option<Box<Node>>,
}

#[derive(Debug)]
pub enum NodeError {
    InvalidExpression(String),
    DivideByZero,
}

impl Error for NodeError {}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeError::InvalidExpression(msg) =>
                    format!("The entered expression is invalid: {}", msg),
                NodeError::DivideByZero => "Cannot divide by zero".to_string(),
            }
        )
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Node {
    pub fn from_expression(expression: String) -> Self {
        let (operator, l_expression, r_expression) = split_on_lowest_priority_operator(expression);

        let l_child: Option<Box<Node>>;
        let r_child: Option<Box<Node>>;
        if l_expression == "".to_string() {
            l_child = None;
        } else {
            l_child = Some(Box::new(Self::from_expression(l_expression)));
        }
        if r_expression == "".to_string() {
            r_child = None;
        } else {
            r_child = Some(Box::new(Self::from_expression(r_expression)));
        }

        Node {
            value: operator,
            l_child: l_child,
            r_child: r_child,
        }
    }

    pub fn evaluate(&self) -> Result<f64, NodeError> {
        if !self.has_children() {
            let value = self
                .value
                .parse::<f64>()
                .map_err(|_| NodeError::InvalidExpression(self.value.clone()));

            return value;
        }

        let l_operand = match &self.l_child {
            None => 0.0,
            Some(l_child) => {
                let res = l_child.evaluate();

                match res {
                    Ok(op) => op,
                    Err(err) => return Err(err),
                }
            }
        };

        let r_operand = match &self.r_child {
            None => 0.0,
            Some(r_child) => {
                let res = r_child.evaluate();

                match res {
                    Ok(op) => op,
                    Err(err) => return Err(err),
                }
            }
        };

        Self::execute_operation(&self.value, l_operand, r_operand)
    }

    fn execute_operation(
        operator: &String,
        l_operand: f64,
        r_operand: f64,
    ) -> Result<f64, NodeError> {
        match operator.as_str() {
            "+" => Ok(l_operand + r_operand),
            "-" => Ok(l_operand - r_operand),
            "*" => Ok(l_operand * r_operand),
            "/" => {
                if r_operand == 0.0 {
                    Err(NodeError::DivideByZero)
                } else {
                    Ok(l_operand / r_operand)
                }
            }
            "^" => Ok(l_operand.powf(r_operand)),
            _ => Err(NodeError::InvalidExpression(format!(
                "{} {} {}",
                l_operand, operator, r_operand
            ))),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = self.value.clone() + if self.has_children() { "\n" } else { "" };

        if self.l_child.is_some() {
            let l_child_string = self.l_child.as_ref().unwrap().to_string();
            let l_child_rows = l_child_string.trim_end().split("\n").collect::<Vec<&str>>();

            if self.r_child.is_some() {
                let r_child_string = self.r_child.as_ref().unwrap().to_string();
                let r_child_rows: Vec<&str> = r_child_string.trim_end().split("\n").collect();

                for i in 0..l_child_rows.len() {
                    if i == 0 {
                        result.push_str("|-- ");
                    } else {
                        result.push_str("|   ");
                    }
                    result.push_str(l_child_rows[i]);
                    result.push_str("\n");
                }

                for i in 0..r_child_rows.len() {
                    if i == 0 {
                        result.push_str("`-- ");
                    } else {
                        result.push_str("    ");
                    }
                    result.push_str(r_child_rows[i]);
                    result.push_str("\n");
                }
            } else {
                for i in 0..l_child_rows.len() {
                    if i == 0 {
                        result.push_str("`-- ");
                    } else {
                        result.push_str("    ");
                    }
                    result.push_str(l_child_rows[i]);
                    result.push_str("\n");
                }
            }
        }

        result
    }

    pub fn has_children(&self) -> bool {
        match (&self.l_child, &self.r_child) {
            (None, None) => false,
            (_, _) => true,
        }
    }
}

fn has_no_operators(expression: &String) -> bool {
    for operator in OPERATORS.split("").collect::<Vec<&str>>() {
        if expression.contains(operator) {
            return false;
        }
    }
    true
}

fn split_on_lowest_priority_operator(expression: String) -> (String, String, String) {
    if has_no_operators(&expression) {
        return (expression, "".to_string(), "".to_string());
    };

    let expression_copy = expression.clone();
    let tokens = expression_copy.split(" ").collect::<Vec<&str>>();
    let mut lowest_priority_operator_index: usize = 0;
    let mut current_priority: u8 = 4;
    for (index, &token) in tokens.iter().enumerate() {
        match token {
            "+" | "-" => {
                lowest_priority_operator_index = index;
                break;
            }
            "*" | "/" => {
                if current_priority > 2 {
                    lowest_priority_operator_index = index;
                    current_priority = 2;
                } else {
                    continue;
                }
            }
            "^" => {
                if current_priority > 3 {
                    lowest_priority_operator_index = index;
                    current_priority = 3
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }
    if lowest_priority_operator_index == 0 {
        (
            tokens[lowest_priority_operator_index].to_string(),
            "".to_string(),
            tokens.split_first().unwrap().1.join(" "),
        )
    } else if lowest_priority_operator_index == tokens.len() {
        (
            tokens[lowest_priority_operator_index].to_string(),
            tokens.split_last().unwrap().1.join(" "),
            "".to_string(),
        )
    } else {
        (
            tokens[lowest_priority_operator_index].to_string(),
            tokens.split_at(lowest_priority_operator_index).0.join(" "),
            tokens
                .split_at(lowest_priority_operator_index + 1)
                .1
                .join(" "),
        )
    }
}

#[test]
fn test_addition() {
    let root = Node::from_expression("1 + 2".to_string());
    assert!(root.evaluate().is_ok_and(|x| x == 3.0));
}

#[test]
fn test_subtraction() {
    let root = Node::from_expression("1 - 2".to_string());
    assert!(root.evaluate().is_ok_and(|x| x == -1.0));
}

#[test]
fn test_multiplication() {
    let root = Node::from_expression("2 * 10".to_string());
    assert!(root.evaluate().is_ok_and(|x| x == 20.0));
}

#[test]
fn test_division() {
    let root = Node::from_expression("1 / 10".to_string());
    assert!(root.evaluate().is_ok_and(|x| x == 0.1));
}

#[test]
fn test_divide_by_zero() {
    let root = Node::from_expression("1 / 0".to_string());
    assert!(root.evaluate().is_err());
}

#[test]
fn test_invalid_expression() {
    let root = Node::from_expression("expression".to_string());
    assert!(root.evaluate().is_err());
}
