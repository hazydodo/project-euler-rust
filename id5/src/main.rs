// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    println!("Hello, world!");

    let mut result = 1;

    loop {
        if divisible(result) {
            println!("{} is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20", result);
            break;
        }
        result += 1;
    }  
}

fn divisible(n: i32) -> bool{

    for number in 1..20 {
        if n % number != 0 {
            return false
        }
    }

    return true;
}

