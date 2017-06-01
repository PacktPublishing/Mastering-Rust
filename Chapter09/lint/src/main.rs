#![feature(plugin)]
#![plugin(lint_fn)]

fn return_the_answer() -> u8 {
    42
}

fn do_nothing() {
    
}

fn main() {
    return_the_answer();
    do_nothing();
}
