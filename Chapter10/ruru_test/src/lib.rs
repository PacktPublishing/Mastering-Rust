#[macro_use]
extern crate ruru;

use ruru::{Float, Class, Object};

methods!(
    Float,
    itself,

    fn sum_floats(f2: Float) -> Float {
        let f1 = itself.to_f64();
        let f2 = f2.expect("f2 contained an error").to_f64();

        Float::new(f1 + f2)
    }


    fn square_root(e: Float) -> Float {
        let e = match e {
            Ok(ruby_float) => ruby_float.to_f64(),
            Err(_) => 0.00000001
        };
        let mut x=0.0;
        let s = itself.to_f64();
        while x*x < s-e || x*x > s+e {
            x += e;
        };
        Float::new(x)
    }
);

#[no_mangle]
pub extern fn initialize_sum_floats() {
    Class::from_existing("Float").define(|itself| {
        itself.def("+", sum_floats);
        itself.def("square_root", square_root);
    });
}
