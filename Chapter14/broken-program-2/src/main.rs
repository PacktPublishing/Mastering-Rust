extern crate num;
use num::Num;
use std::thread;
use std::sync::{Arc, Mutex};

fn multiply_and_store<N: Num + Copy + Send>(nums: Arc<Mutex<Vec<N>>>, num: N, multiplier: N) {
    let mut data = nums.lock().unwrap();
    data.push(num * multiplier);
}

fn multiply_numbers_by<N: 'static + Num + Copy + Send>(nums: Vec<N>, multiplier: N, ret: Arc<Mutex<Vec<N>>>) {
    let mut threads = vec!();

    for num in nums {
        let ret = ret.clone();
        threads.push(thread::spawn(move || {
            multiply_and_store(ret, num, multiplier);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}

fn main() {
    let nums: Vec<f64> = vec!(1.5, 2.192, 3.0, 4.898779, 5.5, -5.0);
    let multiplied_nums = Arc::new(Mutex::new(vec!()));
    multiply_numbers_by(nums, 3.141, multiplied_nums.clone());

    println!("Multiplied numbers: {:?}", multiplied_nums);
}
