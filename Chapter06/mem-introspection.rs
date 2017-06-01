use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("type u8: {}", std::mem::size_of::<u8>());
    println!("type f64: {}", std::mem::size_of::<f64>());
    println!("value 4u8:  {}", std::mem::size_of_val(&4u8));
    println!("value 4:  {}", std::mem::size_of_val(&4));
    println!("value 'a': {}", std::mem::size_of_val(&'a'));

    println!("value \"Hello World\" as a static str slice: {}", std::mem::size_of_val("Hello World"));
    println!("value \"Hello World\" as a String: {}", std::mem::size_of_val("Hello World").to_string());

    println!("Cell(4)): {}", std::mem::size_of_val(&Cell::new(84)));
    println!("RefCell(4)): {}", std::mem::size_of_val(&RefCell::new(4)));

    println!("Rc(4): {}", std::mem::size_of_val(&Rc::new(4)));
    println!("Rc<RefCell(8)>): {}", std::mem::size_of_val(&Rc::new(RefCell::new(4))));
}
