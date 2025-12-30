fn main() {
    for fahrenteit_value in (50..=300).step_by(50) {
        let celsius_value = convert(fahrenteit_value);
        println!(
            "Temperature {fahrenteit_value} of fahrenheit is equal to {celsius_value} in celsius."
        );
    }
}

fn convert(fahrenheit_value: i32) -> i32 {
    (fahrenheit_value - 32) * 5 / 9
}
