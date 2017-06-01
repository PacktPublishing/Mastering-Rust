fn main() {
    let mut outer_scope_x = 42;

    {
        let mut closure = move || {
            outer_scope_x += 42;
            println!("Outer scope variable is {}", outer_scope_x);
        };

        closure();
    }
    println!("Outer_scope_x {}", outer_scope_x);
}
