#[derive(PartialEq)]
struct Fahrenheit(i64);

#[derive(PartialEq)]
struct Celsius(i64);

fn main() {
    let temperature1 = Fahrenheit(10);
    let temperature2 = Celsius(10);

    println!("Is temperature 1 the same as temperature 2? Answer: {}",
             temperature1 == temperature2);

    println!("Temperature 1 is {} fahrenheit", temperature1.0);
    println!("Temperature 2 is {} celsius", temperature2.0);
}
