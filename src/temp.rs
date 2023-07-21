fn main() {
    
    let matches = App::new("temperature")
            .arg(
                Arg::with_name("f")
                    .value_name("Fahrenheit")
                    .help("Enter temperature in Fahrenheit")
                    .required(false)
                    .validator(validate_number)
            ).arg(
                Arg::with_name("c")
                    .value_name("Celsius")
                    .help("Enter temperature in Celsius")
                    .required(false)
                    .validator(validate_number)
            ).get_matches();
    
    let f: f32 = matches.value_of("f")
                    .unwrap_or("0.0").to_string().parse().unwrap();
    let c: f32 = matches.value_of("c")
                    .unwrap_or("0.0").to_string().parse().unwrap();
    
    if matches.is_present("fah"){
        let c_temp = (5.0/9.0)*(fah-32.0);
        println!("{f} Fahrenheit  is equal to {c_temp} in Celsius");
    }else if matches.is_present("cel") {
        let f_temp = ((9.0 * cel)/5.0) + 32.0;
        println!("{c} Celsius is equal to {f_temp} in Fahrenheit");
    }
}

// Function for validating the arguments passed in the command line
fn validate_number(value: String) -> Result<(), String> {
    match value.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(" Must be only numbers!")),
    }
}