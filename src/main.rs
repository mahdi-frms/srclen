fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn count(text:&str)->usize{
    let mut counter = 0;
    for c in text.chars() {
        if c == '\n'{
            counter += 1;
        }
    }
    counter + 1
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
}
