fn main() {
    let mut x = 1;
    {
        let immut_x_1 = &x;
    }

    {
        let mut_x_1 = &mut x;
    }

    let mut_x_2 = &mut x;
    let immut_x_3 = &x;
}
