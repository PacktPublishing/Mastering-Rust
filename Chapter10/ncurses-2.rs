use std::os::raw::{c_char, c_short};
use std::ffi::CString;

#[link(name="ncurses")]
extern {
    fn initscr();
    fn printw(fmt: *const c_char, ...);
    fn refresh();
    fn getch() -> usize;
    fn mvgetch(y: usize, x: usize) -> usize;
    fn endwin();
    fn start_color();
    fn init_pair(pair: c_short, f: c_short, b: c_short);
    fn attron(pair: c_short);
    fn COLOR_PAIR(pair: c_short) -> c_short;
}

struct Ncurses;
impl Ncurses {
    fn new() -> Self {
        let ncurses = Ncurses {};
        ncurses.init_screen();
        ncurses
    }
    fn init_screen(&self) {
        unsafe { initscr(); start_color(); }
    }
    fn refresh(&self) {
        unsafe { refresh(); }
    }
    fn get_char(&self) -> usize {
        unsafe { getch() }
    }
    fn move_and_get_char(&self, y: usize, x: usize) -> usize {
        unsafe {
            mvgetch(y, x)
        }
    }
    fn deinit_screen(&self) {
        unsafe { endwin(); }
    }
    fn color_red(&self) {
        unsafe { init_pair(1, 1, 0); attron(COLOR_PAIR(1)); }
    }
}

impl Drop for Ncurses {
    fn drop(&mut self) {
        self.deinit_screen();
    }
}

fn main() {
    let the_message = CString::new("Rust and ncurses working together, and here is a number: %d").unwrap();

    let ncurses = Ncurses::new();
    ncurses.color_red();
    unsafe {
        printw(the_message.as_ptr(), 1);
    }

    ncurses.refresh();
    ncurses.get_char();
    ncurses.move_and_get_char(50, 20);
}
