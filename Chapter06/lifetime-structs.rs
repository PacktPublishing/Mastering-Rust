struct Number<'a> {
    num: &'a u8
}

impl<'a> Number<'a> {
  fn get_the_number(&self) -> &'a u8{
      self.num
  }
  fn set_the_number(&mut self, new_number: &'a u8) {
      self.num = new_number  
  }
}

fn main() {
    let inner_one = 1;
    let inner_two = 2;
    let mut num = Number { num: &inner_one };

    println!("num is now {}", num.get_the_number());

    num.set_the_number(&inner_two);

    println!("num is now {}", num.get_the_number());
}
