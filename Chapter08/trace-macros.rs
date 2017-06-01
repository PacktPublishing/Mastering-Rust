#![feature(trace_macros, log_syntax)]

trace_macros!(true);

macro_rules! meep {
    () => ();
    ($block:block) => (
        log_syntax!("Inside 2nd branch, block is" $block); 
        ($block);
        log_syntax!("Leaving 2nd branch!");
    );
}

fn main() {
    meep!();
    meep!({silly; things});
    println!("Just to show how fun println! really gets");
}
