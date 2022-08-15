mod leet_0001;
mod leet_1480;

fn main() {
    // Variables
    let mut my_array: [i32; 5] = [3, 1, 2, 10, 1];

    //* 0001. Two Sum
    println!("Two Sum = {:?}", leet_0001::two_sum([3, 2, 4].to_vec(), 6));

    //* 1480. Running Sum of 1D Array
    // println!("Running Sum  = {:?}", leet_1480::sol_1(&my_array));
    // println!("Running Sum  = {:?}", leet_1480::sol_2(&mut my_array));

    //* 1672. Richest Customer
}
