fn square(x: u32) -> u32 {
    x * x
}

fn function_without_vars() {
    println!("Entered function without variables");
}

fn main() {
    let square_c1 = |x: u32|          x*x;
    let square_c2 = |x: u32|        { x*x };
    let square_c3 = |x: u32| -> u32 { x*x };

    let closure_without_vars = || println!("Entered closure without variables");

    println!("square of 4 = {}", square(4));
    println!("square of 4 = {}", square_c1(4));
    println!("square of 4 = {}", square_c2(4));
    println!("square of 4 = {}", square_c3(4));

    function_without_vars();
    closure_without_vars();
}
