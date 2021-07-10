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
