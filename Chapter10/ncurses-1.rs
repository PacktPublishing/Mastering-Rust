use std::os::raw::c_char;
use std::ffi::CString;

#[link(name="ncurses")]
extern {
    fn initscr();
    fn printw(fmt: *const c_char, ...);
    fn refresh();
    fn getch();
    fn endwin();
}

fn main() {
    let the_message = CString::new("Rust and ncurses working together, and here is a number: %d").unwrap();
    unsafe {
        initscr();
        printw(the_message.as_ptr(), 1);
        refresh();
        getch();
        endwin();
    }
} 
