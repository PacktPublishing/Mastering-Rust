static mut meep: u32 = 4;
static FUUP: u8 = 9;

fn main() {
    unsafe {
        println!("Meep is {}", meep);
        meep = 42;
        println!("Meep is now {}", meep);
    }

    println!("Fuup is {}", FUUP);
}
