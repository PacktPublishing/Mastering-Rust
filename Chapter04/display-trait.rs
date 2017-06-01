use std::fmt::{Formatter, Display, Result};

struct Money<T> {
	  amount: T,
    currency: String
}

impl<T> Display for Money<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

fn main() {
    let money = Money { amount: 42, currency: "EUR".to_string() };
    println!("Displaying money: {}", money);
}



