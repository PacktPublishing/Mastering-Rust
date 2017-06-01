use std::fmt::Debug;

static mut x: u8 = 1;
const FOO: u8 = 4;
fn print_slice<T: Debug>(slice: &[T]) {
    println!("{:?}", slice);
}

fn main() {
    let array: [u8; 5] = [1, 2, 3, 4, 5];

    print!("Whole array just borrowed: ");
    print_slice(&array);

    print!("Whole array sliced: ");
    print_slice(&array[..]);

    print!("Without the first element: ");
    print_slice(&array[1..]);

    print!("One element from the middle: ");
    print_slice(&array[3..4]);

    print!("First three elements: ");
    print_slice(&array[..3]);

    //print!("Oops, going too far!: ");
    //print_slice(&array[..900]);

    unsafe {
    x = 4;
    }
    let z = &FOO;
    let g = &FOO;
    println!("{} {}", &z, g);
}
