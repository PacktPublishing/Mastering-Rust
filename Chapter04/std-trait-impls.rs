use std::ops::Add;

#[derive(Debug)]
struct Money<T> {
	  amount: T,
    currency: String
}

impl<T: Add<T, Output=T>> Add for Money<T> {
    type Output = Money<T>;
    fn add(self, rhs: Money<T>) -> Self::Output {
        assert!(self.currency == rhs.currency);
        Money { currency: rhs.currency, amount: self.amount + rhs.amount }
    }
}

fn main() {
    let whole_euros_1: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let whole_euros_2: Money<u8> = Money { amount: 42, currency: "EUR".to_string() };
    let summed_euros = whole_euros_1 + whole_euros_2;

    println!("Summed euros: {:?}", summed_euros);
}
