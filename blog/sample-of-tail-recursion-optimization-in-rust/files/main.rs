use std::io;

fn main() {
    println!("Enter 2 positive integers to be added: ");
    let mut inp1: String = String::new();
    let mut inp2: String = String::new();

    io::stdin()
        .read_line(&mut inp1)
        .expect("Failed to read line!");

    let inp1: u32 = match inp1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a positive 32-bit integer");
            return;
        }
    };

    io::stdin()
        .read_line(&mut inp2)
        .expect("Failed to read line!");

    let inp2: u32 = match inp2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a positive 32-bit integer");
            return;
        }
    };

    println!("{}", add(inp1, inp2));
}

fn add(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else {
        add(a - 1, b + 1)
    }
}
