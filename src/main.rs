fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn count(text:&str)->usize{
    let mut counter = 0;
    let mut new_line = true;
    let mut slash = false;
    for c in text.chars() {
        if c == '\n' {
            new_line = true;
        }
        else if c == '/' {
            if new_line {
                if slash {
                    slash = false;
                    new_line = false;
                }
                else{
                    slash = true;
                }
            }
        }
        else{
            slash = false;
            if c != ' ' && new_line {
                counter += 1;
                new_line = false;
            }
        }
    }
    
    counter
}

#[cfg(test)]
mod test {
    use crate::count;

    #[test]
    fn counts_lines(){
        let text = 
"fn main() {
    println!(\"Hello, world!\");
}";
        assert_eq!(count(text),3);
    }

    #[test]
    fn ignores_empty_lines(){
        let text = 
"fn main() {
    println!(\"Hello, world!\");

}
";
        assert_eq!(count(text),3);
    }

    #[test]
    fn ignores_lines_with_only_white_spaces(){
        let text = 
"fn main() {
    println!(\"Hello, world!\");
    
}
";
        assert_eq!(count(text),3);
    }

    #[test]
    fn ignores_comments(){
        let text = 
"fn main() {
    //println!(\"Hello, world!\");
}";
        assert_eq!(count(text),2);
    }
}
