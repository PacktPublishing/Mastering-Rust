use std::convert::Into;

struct Money<T> {
	  amount: T,
    currency: String
}

#[derive(Debug)]
struct CurrencylessMoney<T> {
	  amount: T
}

impl<T> Into<CurrencylessMoney<T>> for Money<T> {
    fn into(self) -> CurrencylessMoney<T> {
        CurrencylessMoney { amount: self.amount }
    }
}

fn main() {
    let money = Money { amount: 42, currency: "EUR".to_string() };
    let currencyless_money: CurrencylessMoney<u32> = money.into();

    println!("Money without currency: {:?}", currencyless_money);
}
