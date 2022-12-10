use std::{env, process::exit};

enum Unit {
    Celsius,
    Fahrenheit,
}

fn main() {
    let (choice, temperature) = parse_args();

    match choice {
        Unit::Fahrenheit => {
            println!(
                "{}° Fahrenheit = {:.2}° Celsius",
                temperature,
                convert(temperature, choice)
            );
        }

        Unit::Celsius => {
            println!(
                "{}° Celsius = {:.2}° Fahrenheit",
                temperature,
                convert(temperature, choice)
            );
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_MENU: &str = "
Celsius <=> Fahrenheit

Usage:
    cf <flag> <temperature>

    flags:
        -f | --fahrenheit      To convert Fahrenheit to Celsius
        -c | --celsius         To convert Celsius to Fahrenheit
        -v | --version         Version
        -h | --help            Help Menu

Example:
    cf -f 100
    cf -c 37.78";

fn parse_args() -> (Unit, f64) {
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();

    if args_len > 3 || args_len == 1 {
        println!("{}", HELP_MENU);
        exit(1);
    }

    match args[1].as_str() {
        "-f" | "--fahrenheit" => {
            if args_len != 3 {
                println!("{}", HELP_MENU);
                exit(1)
            }
            (Unit::Fahrenheit, parse_float(&args[2]))
        }
        "-c" | "--celsius" => {
            if args_len != 3 {
                println!("{}", HELP_MENU);
                exit(1)
            }
            (Unit::Celsius, parse_float(&args[2]))
        }
        "-h" | "--help" => {
            println!("{}", HELP_MENU);
            exit(0)
        }
        "-v" | "-V" | "--version" => {
            println!("cf v{}", VERSION);
            exit(0)
        }
        _ => {
            println!("{}", HELP_MENU);
            exit(1);
        }
    }
}

fn parse_float(arg: &String) -> f64 {
    arg.parse().unwrap_or_else(|_| {
        println!("Invalid argument: {}\nPass in a number", arg);
        exit(1)
    })
}

fn convert(temp: f64, unit: Unit) -> f64 {
    match unit {
        Unit::Celsius => temp * (9f64 / 5f64) + 32f64,
        Unit::Fahrenheit => (temp - 32f64) * (5f64 / 9f64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float(&String::from("100")), 100.0);
        assert_eq!(parse_float(&String::from("37.78")), 37.78);
        assert_eq!(parse_float(&String::from("0")), 0.0);
        assert_eq!(parse_float(&String::from("-273.15")), -273.15);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert(212.0, Unit::Fahrenheit), 100.0);
        assert_eq!(convert(100.0, Unit::Celsius), 212.0);
        assert_eq!(convert(0.0, Unit::Celsius), 32.0);
        assert_eq!(convert(32.0, Unit::Fahrenheit), 0.0);
        assert_eq!(convert(-40.0, Unit::Celsius), -40.0);
        assert_eq!(convert(-40.0, Unit::Fahrenheit), -40.0);
    }

}
