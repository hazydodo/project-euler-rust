// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    //println!("Hello, world!");

    let mut result: [i32; 3] = [0, 0, 0];

    for factor_a in 100..999 {
        for factor_b in 100..999 {
            if is_palindrome(factor_a * factor_b) {
                println!("{} * {} form the palindrome {}", factor_a, factor_b, factor_a * factor_b);

                if factor_a * factor_b > result[2] {
                    result[0] = factor_a;
                    result[1] = factor_b;
                    result[2] = factor_a * factor_b;
                }

            }
        }
    }

    println!("The largest palindrome is {} formed by {} * {}", result[2], result[0], result[1]);

}

fn is_palindrome(n: i32) -> bool {
    // change n to a string so we can slice it
    let number = n.to_string();

    for index in 0..((number.len() / 2)){
        if number[index..index + 1] == number[number.len() - (1 + index)..number.len() - index] {
            continue;
        }
        else {
            return false
        }
    }
    true 
}