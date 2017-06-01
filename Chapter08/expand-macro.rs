macro_rules! meep {
    () => (nothing);
    ($block:block) => ( make($block); );
}

fn main() {
    meep!();
    meep!({silly; things});
    println!("Just to show how fun println! really gets");
}
