fn is_correct(evidence: &str) -> bool {
    /*
     The basic idea here is to use a stack.
     iterate through the elements of the str, pushing opening
     brackets on to the stack and popping when encountering
     a closing bracket.
    */
    let mut bracket_stack: Vec<char> = vec!();
    for c in evidence.chars() {
        if c == '{' {
            bracket_stack.push(c);
        } else if c == '}' {
            let bracket =
                bracket_stack
                    .pop();
            if bracket.is_none() {
                return false;
            }
        }
    }
    bracket_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let actual = is_correct("{}");
        assert!(actual);
    }

    #[test]
    fn complex_test() {
        assert!(is_correct("{{{}{}}}"));
    }

    #[test]
    fn simple_incorrect() {
        assert!(!is_correct("}"));
    }

    #[test]
    fn complex_incorrect() {
        assert!(!is_correct("{{{}{}}}}"))
    }
}

fn main() {
    println!("brackets are awesome");
}