use std::io;
use colored::*;
fn main() {
    let f = loop {
        println!("{}", "Please input a temperature in Fahrenheit:".bright_yellow().bold());
        let mut f = String::new();
        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read input!");

        let f: f64 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break f;
    };
    let c = f_to_c(f);
    let f = format!(" {:.1} ", &f);
    let c = format!(" {:.1} ", &c);
    println!("{} {},\n{} {}",
        "Input Fahrenheit:".bright_yellow().bold(),
        f.bright_white().bold().on_bright_red(),
        "in Celsius:".bright_yellow().bold(), 
        c.black().bold().on_bright_green());
}

fn f_to_c(base: f64) -> f64 {
    let f = base - 32f64;
    let i = 5f64 / 9f64;
    let c = f * i;
    c
}
