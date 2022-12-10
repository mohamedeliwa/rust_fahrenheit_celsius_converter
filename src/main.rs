use std::io;

fn main() {
    println!("\nFahrenheit and Celsius\n");

    'outer: loop {
        println!("\nChoose the temperature scale you're converting from");
        println!("1. Fahrenheit");
        println!("2. Celsius");
        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Can't read your input!");
        let scale: u8 = match scale.trim().parse() {
            Ok(num) => match num {
                1 => 1,
                2 => 2,
                _ => {
                    println!("Please choose 1 or 2!");
                    continue;
                }
            },
            Err(_) => {
                println!("Please choose 1 or 2!!");
                continue;
            }
        };

        println!("Enter temp: ");
        'inner: loop {
            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Can't read your input!");

            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a valid temp!");
                    continue 'inner;
                }
            };

            if scale == 1 {
                let celsius = (temp - 32.0) * (5.0 / 9.0);
                println!("\nFahrenheit: {},  Celsius: {}", temp, celsius);
                continue 'outer;
            }

            if scale == 2 {
                let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
                println!("\nFahrenheit: {},  Celsius: {}", fahrenheit, temp);
                continue 'outer;
            }
        }
    }
}
