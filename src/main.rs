fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn count(text:&str)->usize{
    let mut counter = 0;
    let mut new_line = true;
    for c in text.chars() {
        if c == '\n' {
            new_line = true;
        }
        else {
            if new_line {
                counter += 1;
            }
            new_line = false;
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
}
