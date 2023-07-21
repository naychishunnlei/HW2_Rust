use clap::{Arg, App};

fn main() {
    
    let matches = App::new("circle")
    .version("0.1.0")
            .author("66011090@kmitl.ac.th")
            .about("calculate the area of a circle")
            .arg(
                Arg::with_name("radius")
                    .value_name("RADIUS")
                    .help("Enter a value for radius")
                    .required(true)
                    .index(1)
            ).get_matches();
    
    let r = matches.value_of("r").unwrap().to_string();
    let r = r.parse().unwrap_or(0.0);
    let pi = 3.1416;
    let area = pi * r * r;

    println!("The area of the circle with radius = {r} is : {area}");
    
    //use :  cargo run --bin circle <radius> to run the program
}