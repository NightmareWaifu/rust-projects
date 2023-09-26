use std::io;

fn io() -> io::Stdin{
    let stdin = io::stdin();

    return stdin;
}
fn main() {
    mining();
}


fn mining(){
    let mut extractor: String = String::new();
    let mut int_extractor: i32 = 0;
    println!("How many extractors: ");
    io().read_line(&mut extractor);
    int_extractor = extractor.trim().parse().expect("Invalid Input");
    //1 extractor = 50/s
    //1 solar panel = 50/s
    //1 battery holds 45k
    //day-night cycle 30mins (15mins each)

    //take note -> excess power to charge batteries
    //batteries can charge extractors for 15mins
    const night_duration: i32 = 15*60;
    const day_duration: i32 = 12*60;
    const extractor_power: i32 = 50;
    const battery_power: i32 = 45000;
    let power_needed = int_extractor * night_duration * extractor_power;
    let batteries_needed: i32 = (power_needed / battery_power) + 1; 
    let solar_panels: i32 = power_needed*2 / 50 / day_duration;
    let solar_generation: i32 = solar_panels * 50;
    println!("Power needed: {}\nBatteries needed: {}\nSolar Panels needed: {}\nPower Generated: {}",power_needed, batteries_needed, solar_panels,solar_generation);
}