use std::io;

fn main() {
    loop {
        print_menu();
        let mut conversion_direction = String::new();
        console_read(&mut conversion_direction);

        let conversion_direction = string_to_unsigned_integer(&conversion_direction);
        if conversion_direction == 0 {
            println!("--> You don't need anything more? Bye for now!");
            break;
        }

        let direction_description = match conversion_direction {
            1 => "celcius --> fahrenheit",
            2 => "fahrenheit --> celcius",
            _ => {
                println!("unknown conversion direction");
                continue;
            }
        };

        println!("Enter the value to convert: {direction_description}");
        let mut value_to_convert = String::new();
        console_read(&mut value_to_convert);
        let value_to_convert = string_to_float(&value_to_convert);
        if conversion_direction == 1 {
            let converted_temperature = convert_from_celcius_to_fahrenheit(value_to_convert);
            println!("conversion of {value_to_convert} {direction_description} = {converted_temperature}");
        } else {
            let converted_temperature = fahrenheit_to_celcius_converter(value_to_convert);
            println!("conversion of {value_to_convert} {direction_description} = {converted_temperature}");
        }
    }
}

fn print_menu() {
    println!("Choisir le sens de conversion:");
    println!("  * 1: celcius --> fahrenheit");
    println!("  * 2: fahrenheit --> celcius");
    println!("  * 0: to exit the program");
}

fn convert_from_celcius_to_fahrenheit(temperature_celcius: f32) -> f32 {
    (temperature_celcius * 1.8) + 32.0
}

fn fahrenheit_to_celcius_converter(temperature_fahrenheit: f32) -> f32 {
    (temperature_fahrenheit - 32.0) / 1.8
}

fn console_read(console_read_buffer: &mut String) {
    io::stdin()
        .read_line(console_read_buffer)
        .expect("Reading the conversion direction failed!");
}

fn string_to_unsigned_integer(string_value: &String) -> u32 {
    string_value
        .trim()
        .parse()
        .expect("Unable to parse the given conversion direction")
}

fn string_to_float(string_value: &String) -> f32 {
    string_value
        .trim()
        .parse()
        .expect("Unable to parse the given conversion direction")
}
