fn main() {
    let input = 10;
    let result = fib(input);
    println!("The {}th fib number is {}", input, result);
}

fn fib(i: u32) -> u32 {
    if i == 0 {
        return 0;
    }
    if i == 1 {
        return 1;
    }
    fib(i - 1) + fib(i - 2)
}
