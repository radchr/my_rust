fn main() {
    // println!("Hello, world!");
    // println!("Result of multy 2.5 x 4.3 = {}", multiply_both(2.5, 4.3));
    let answer = multiply_both(2.5, 4.3);
    println!("Result of multy 2.5 x 4.3 = {}", answer);

    let mut a = 255u8;
    a = a + 0;
    println!("{}", a);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}
