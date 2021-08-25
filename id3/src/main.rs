// What is the largest prime factor of the number 600851475143 ?

use integer_sqrt::IntegerSquareRoot;


fn main() {
    let value: i64 = 600851475143;
    let mut largest_factor = 1;
    let mut num = 1;
    while num * num < value {
        if value % num == 0 && num > largest_factor {
            if is_prime(num) {
                largest_factor = num;
            }
        } 
        num += 1
    }

    println!("{}", largest_factor);
}

fn is_prime(n: i64) -> bool {
    for number in 2..(n.integer_sqrt()) {
        if n % number == 0 {
            //println!("{}", number);
 
 
            return false;
        }
    }
    return true;
}

