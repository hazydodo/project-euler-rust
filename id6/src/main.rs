// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    println!("The difference between the sum of the square of the first oen hundred natural numbers and the square of the sum is: ");
    println!("{}", (sum_of_squares(100) - square_of_sum(100)).abs());
}

// 1^2 + 2^2 + 3^2 ...
fn sum_of_squares(n: i32) -> i32{

    let mut result = 0;

    for number in 0..n + 1 {
        result += number * number;
    }
    result
}

// (1 + 2 + 3 + ...)^2
fn square_of_sum(n: i32) -> i32{

    let mut result = 0;

    for number in 0..n + 1 {
        result += number;
    }
    result * result
}