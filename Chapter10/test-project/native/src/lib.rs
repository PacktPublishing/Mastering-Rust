#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsNumber;

fn square_root(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let s = try!(try!(call.arguments.require(scope, 0)).check::<JsNumber>()).value();
    let e = match call.arguments.get(scope, 1) {
        Some(js_number) => try!(js_number.check::<JsNumber>()).value(),
        None => 0.00000001
    };
    let mut x=0.0;

    while x*x < s-e || x*x > s+e {
        x += e;
    };
    Ok(JsNumber::new(scope, x))
}

register_module!(m, {
    m.export("squareRoot", square_root)
});
