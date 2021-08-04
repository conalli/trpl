use std::io;
use colored::*;
fn main() {
    loop {
        println!("{}", "Please input a temperature in Fahrenheit:".yellow());
        let mut f = String::new();
        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read input!");

        let f: f64 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let c = f_to_c(f);
        let f = format!("{:.1}", &f);
        let c = format!("{:.1}", &c);
        break println!("{} in Celsius: {}", f.red(), c.green());
    }
}

fn f_to_c(base: f64) -> f64 {
    let f = base - 32f64;
    let i = 5f64 / 9f64;
    let c = f * i;
    c
}
