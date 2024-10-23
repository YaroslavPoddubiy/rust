#[derive(Default)]
pub(crate) struct Calculator;
impl Calculator {
    pub(crate) fn calculate(&self, expression: String) -> Result<f32, String> {

        let mut tokens: Vec<String> = Vec::new();
        match self.parse(expression.clone()) {
            Ok(tok) => {tokens = tok},
            Err(error) => return Err(error),
        }
        let postfix_tokens = self.postfix_gen(tokens.clone());
        match self.calculate_postfix(postfix_tokens) {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        }

    }

    pub(crate) fn parse(&self, expression: String) -> Result<Vec<String>, String> {
        let mut tokens_array: Vec<String> = Vec::new();
        let mut token: String = String::new();
        for char in expression.chars() {
            if char.is_digit(10) {
                if !token.is_empty()  && (token.ends_with('+') || token.ends_with('-') ||
                    token.ends_with('*') || token.ends_with('/') || token.ends_with('^')) {
                    tokens_array.push(token.clone());
                    token = String::new();
                }
                token.push(char);
            }
            else if char == '.' {
                if !token.is_empty() && token.chars().last().unwrap().is_digit(10) {
                    token.push(char);
                }
                else {
                    return Err(format!("Unexpected token {} at {}", char, token.clone() + char.to_string().as_str()));
                }
            }
            else if char == '+' || char == '-'  || char == '*' || char == '/'  || char == '^' {
                if !token.is_empty() && token.chars().last().unwrap().is_digit(10) {
                    tokens_array.push(token.clone());
                    token = String::from(char);
                }
                else {
                    return Err(format!("Unexpected token {} at {}", char, token.clone() + char.to_string().as_str()));
                }
            }
            else if char == ' '  || char == '\n' || char == '\r' || char == '\t' {
                continue;
            }
            else {
                return Err(format!("Unexpected token {} at {}", char, token.clone() + char.to_string().as_str()));
            }
        }
        tokens_array.push(token);
        Ok(tokens_array)
    }

    fn get_operation_result(&self, a: f32, b: f32, operator: String) -> Result<f32, String> {
        if operator == "*" {
            Ok(a * b)
        } else if operator == "/" {
            if b == 0.0 {
                Err(String::from("Cannot divide by zero"))
            }
            else {
                Ok(a / b)
            }
        } else if operator == "^" {
            Ok(a.powf(b))
        } else if operator == "+" {
            Ok(a + b)
        } else if operator == "-" {
            Ok(a - b)
        } else {
            Err(format!("Unknown operator {}", operator))
        }
    }

    fn get_token_type(&self, item: String) -> String {
        if item.parse::<f32>().is_ok() {
            String::from("num")
        }
        else if item == "*" || item == "/" || item == "^" || item == "+"  || item == "-" {
            String::from("operator")
        } else {
            String::from("unknown_item")
        }
    }

    fn priority(&self, operator: String) -> u8 {
        if operator == "^"{
            3
        }
        else if operator == "*" || operator == "/" {
            2
        }
        else if operator == "+" || operator == "-" {
            1
        }
        else {
            0
        }
    }

    fn postfix_gen(&self, tokens: Vec<String>) -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        let mut postfix_tokens: Vec<String> = Vec::new();
        for token in tokens {
            if self.get_token_type(token.clone()) == "num"{
                postfix_tokens.push(token.clone());
            }
            else if self.get_token_type(token.clone()) == "operator"{
                if stack.is_empty() || self.priority(token.clone()) >= self.priority(stack.last().unwrap().to_string()) {
                    stack.push(token.clone());
                }
                else {
                    postfix_tokens.push(stack.pop().unwrap());
                    for i in (0..stack.len()).rev()  {
                        if self.priority(stack[i].clone()) >= self.priority(token.clone()) {
                            postfix_tokens.push(stack.pop().unwrap());
                        }
                        else { break; }
                    }
                    stack.push(token.clone());
                }
            }
        }
        for _i in 0..stack.len() {
            postfix_tokens.push(stack.pop().unwrap());
        }
        postfix_tokens
    }

    fn calculate_postfix(&self, tokens: Vec<String>) -> Result<f32, String> {
        let mut operands: Vec<f32> = Vec::new();
        for token in tokens {
            if self.get_token_type(token.clone()) == "num"{
                operands.push(token.parse::<f32>().unwrap());
            }
            else{
                let operand2: f32 = operands.pop().unwrap();
                let operand1: f32 = operands.pop().unwrap();
                match self.get_operation_result(operand1, operand2, token.clone()) {
                    Ok(result) => operands.push(result),
                    Err(error) => return Err(error),
                }
            }
        }
        Ok(operands[0])
    }
}
