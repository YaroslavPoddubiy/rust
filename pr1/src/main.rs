fn main() {
    let mut num: usize = 0;
    num += 1;
    let str = "Hello, world!";
    while num <= str.len() {
        for j in 0..num{
            print!("{}", str.as_bytes()[j] as char);
        }
        println!();
        num += 1;
    }
}

