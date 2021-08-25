// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn main() {

    let mut result = 0;
    let mut index = 0;

    while fibonacci(index) < 4000000 {
        // current fibonacci value is used many times so its more efficient to save it.
        let current_num = fibonacci(index);

        if current_num % 2 == 0 {
            result += current_num
        }

        index += 1
    }

    println!("{}", result)
}


fn fibonacci(n: i64) -> i64 {
    if n < 2 {
        n
    }
    else {
        fibonacci(n - 1) + fibonacci(n -2)
    }
}