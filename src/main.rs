#[cfg(test)]
mod test;

fn main() {
    
    let mut args = std::env::args();
    args.next();
    let mut sum = 0;
    for arg in args {
        match std::fs::read_to_string(arg.as_str()){
            Ok(content)=> {
                let count = count(content.as_str());
                sum += count;
                println!("{}: {} lines",arg,count);
            },
            Err(_)=> {
                println!("{}: could not read file!",arg);
            }
        }
    }

    println!("\nsum: {} lines",sum);
}

struct CounterStatus {
    counter:usize,
    slash:bool,
    new_line:bool
}
impl CounterStatus {
    fn new()->CounterStatus {
        CounterStatus{
            counter:0,
            slash:false,
            new_line:true
        }
    }
}
fn count(text:&str)->usize{
    let mut status = CounterStatus::new();
    for c in text.chars() {
        if c == '\n' {
            status.new_line = true;
        }
        else if c == '/' {
            count_handle_slash(&mut status)
        }
        else{
            count_normal_char(&mut status ,c)
        }
    }
    status.counter
}

fn count_handle_slash(status:&mut CounterStatus) {
    if  status.new_line {
        if status.slash {
            status.slash = false;
            status.new_line = false;
        }
        else{
            status.slash = true;
        }
    }
}

fn count_normal_char(status:&mut CounterStatus,c:char) {
    status.slash = false;
    if c != ' ' && status.new_line {
        status.counter += 1;
        status.new_line = false;
    }
}