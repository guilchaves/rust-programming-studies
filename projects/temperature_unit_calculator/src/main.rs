use std::io;

fn main() {
    let mut unit  = String::new();
    let mut temperature = String::new();

    println!("Select which unit you want to convert. Fahrenheit (F) / Celcius (C)");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read unit");

    let unit = unit.trim().to_lowercase();

    if unit == "f" {
        println!("Enter temperature in Fahrenheit: ");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let fahrenheit = temperature.trim().parse::<f32>().unwrap();
        let celcius= calculate_f_to_c(&fahrenheit);

        println!("{:.2} F is {:.2} C", fahrenheit, celcius);
    }

    if unit == "c" {
        println!("Enter temperature in Celcius: ");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let celcius = temperature.trim().parse::<f32>().unwrap();
        let fahrenheit = calculate_c_to_f(&celcius);

        println!("{:.2} C is {:.2} F", celcius, fahrenheit);
    }

}

fn calculate_f_to_c(temperature: &f32) -> f32 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn calculate_c_to_f(temperature: &f32) -> f32 {
    (temperature * 9.0 / 5.0) + 32.0
}
