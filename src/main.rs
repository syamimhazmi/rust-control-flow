fn main() {
    loop {
        println!("Please put the value of temperature that you want to convert.");

        let mut temperature = String::new();
        std::io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to temperature read line");

        let temperature: f64 = temperature.trim()
            .parse()
            .expect("Please type a float value. eg: 32.5 or 24.0");

        println!("Please type the value of temperature format that you want to.");

        let mut temperature_format = String::new();

        std::io::stdin()
            .read_line(&mut temperature_format)
            .expect("Failed to read temperature format line");

        let temperature_format = temperature_format.trim()
            .parse()
            .expect("Failed to parse format string");

        let result = convert(temperature, temperature_format);

        println!("{}", result)
    }

}

fn convert(temperature: f64, format: String) -> String {
    match format.to_lowercase().as_str() {
        "celsius" => {
            let value = (temperature * (9.0/5.0)) + 32.0;
            format!("The value from fahrenheit to celsius is: {:.2}°C", value).to_string()
        },
        "fahrenheit" => {
            let value = (temperature - 32.0) * (5.0/9.0);
            format!("The value from celsius to fahrenheit is: {:.2}°F", value).to_string()
        },
        _ => {
            "Invalid format value".to_string()
        }
    }
}
