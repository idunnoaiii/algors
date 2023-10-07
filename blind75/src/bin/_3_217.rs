fn _valid_parentheses(s: String) -> bool {
    let mut stack = "".to_string();

    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if let Some(m) = stack.pop() {
                    if m != c {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    return stack.len() == 0;
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(_valid_parentheses("()".to_string()), true);
    }
    
    #[test]
    fn ex2() {
        assert_eq!(_valid_parentheses("[]()".to_string()), true);
    }
    
    #[test]
    fn ex3() {
        assert_eq!(_valid_parentheses("[]({})".to_string()), true);
    }
    
    #[test]
    fn ex4() {
        assert_eq!(_valid_parentheses("[(]({})".to_string()), false);
    }
}

