#![feature(log_syntax)]

macro_rules! m1 {
    ($($x:tt),*) => { 
        $(            
            log_syntax!(Got $x);            
        )*
    };
}

fn main() {
    m1!(Meep, Moop, { 1 2 3 });
} 