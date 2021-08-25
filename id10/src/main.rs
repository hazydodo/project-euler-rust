// Find the sum of all the primes below two million.
// Sieve of Eratosthenes could be a better method...

fn main() {
    let mut total = 0;
    let mut value = 2;

    loop {
        if is_prime(value) {
            total += value;
        }

        if value > 2000000 {
            break;
        }

        value += 1
    }

    println!("{}", total)
    
}

fn is_prime(n: u64) -> bool {
    
    for number in 2..(n as f32).sqrt() as u64 + 1 {
        if n % number == 0 {
            return false
        }
    }
    
    
    true
}