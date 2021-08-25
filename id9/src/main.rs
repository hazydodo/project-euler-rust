// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2


// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    //a + b + c = 1000
    // a^2 + b^2 = c^2

    'outer: for x in 1..1000 {
        for y in 1..1000 {
            for z in 1..1000 {
                if x < y && y < z {
                    if x + y + z == 1000 {
                        if x*x + y*y == z*z {
                            println!("x = {}, y = {}, z = {}", x, y, z);
                            println!("The product of xyz is {}", x*y*z);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    
}
