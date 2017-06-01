extern crate num;
use num::Num;

fn multiply_numbers_by<N: Num + Copy>(nums: Vec<N>, multiplier: N) -> Vec<N> {
    let mut ret = vec!();
    for num in nums {
        ret.push(num * multiplier);
    }
    ret
}

fn main() {
    let nums: Vec<f64> = vec!(1.5, 2.192, 3.0, 4.898779, 5.5, -5.0);
    println!("Multiplied numbers: {:?}", multiply_numbers_by(nums, 3.141));
}

