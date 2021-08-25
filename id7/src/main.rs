// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

//use integer_sqrt::IntegerSquareRoot;


fn main() {

    let mut index = 1;
    let mut number = 3;

    loop {
        if is_prime(number) {
            index += 1
        }


        if index == 10001 {
            println!("{} is the 10001st prime number", number);
            break;
        }

        number += 1

    }



    //println!("{}", is_prime(6));
}

fn is_prime(n: i32) -> bool {
    for number in 2..((n as f32).sqrt() as i32 + 1) {
        if n % number == 0 {
            return false;
        }
    }
    return true;
}