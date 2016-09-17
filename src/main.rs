fn main() {
    let x = 20;
    let y: u64 = 125;
    let y = y - 1;
    println!("x: {}, y: {}", x, y);
    println!("Hello!");
    print_i32(y as i32);
    ask_yesno();
    let z = factorial(x);
    println!("The factorial of {x} is {z}.", x = x, z = z);
}

fn print_i32(x: i32) {
    println!("x = {}", x);
}

fn ask_yesno() {
    
}

fn factorial(x: u32) -> u64 {
    let mut i = x;
    let mut result: u64 = 1;
    loop {
        println!("{}", result);
        result *= i as u64;

        i -= 1;
        if i == 0 { break; }
    }
    result
}
