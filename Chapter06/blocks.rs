fn main() {
    let level_0_str = String::from("foo");
    {
        let level_1_number = 9;
        {
            let level_2_vector = vec![1, 2, 3];
        } // level_2_vector goes out of scope here

        {
            let level_2_number = 9;
        } // level_2_number goes out of scope here
    } // level_1_number goes out of scope here
} // level_0_str goes out of scope here
