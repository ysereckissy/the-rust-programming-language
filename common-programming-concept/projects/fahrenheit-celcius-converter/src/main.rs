use std::io;

fn main() {
    loop {
        print_menu();
        let mut conversion_direction = String::new();
        io::stdin()
            .read_line(&mut conversion_direction)
            .expect("Reading the conversion direction failed!");

        let conversion_direction: u32 = conversion_direction
            .trim()
            .parse()
            .expect("Unable to parse the given conversion direction");

        if conversion_direction == 0 {
            println!("--> You don't need anything more? Bye for now!");
            break;
        }

        let direction_description = if conversion_direction == 1 {
            "celcius --> fahrenheit"
        } else if conversion_direction == 2 {
            "fahrenheit --> celcius"
        } else {
            println!("unknown conversion direction");
            continue;
        };

        println!("Enter the value to convert: {direction_description}");
        let mut value_to_convert = String::new();

        io::stdin()
            .read_line(&mut value_to_convert)
            .expect("Enable to read value to convert");
        let value_to_convert: f32 = value_to_convert
            .trim()
            .parse()
            .expect("Unable to deconde the value to convert");

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
