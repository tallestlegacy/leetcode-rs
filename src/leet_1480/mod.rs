/**
### Memoizing running sum in an array/vector
- time complexity  = O(N)
- space complexity = O(1)
*/

pub fn sol_1(array: &[i32]) -> Vec<i32> {
    println!();
    println!("Input : [i32;{}] = {:?}", array.len(), array);

    let mut memo = vec![0; array.len()];

    let mut x = 1;
    memo[0] = array[0];

    while x < array.len() {
        memo[x] = array[x] + memo[x - 1];
        x += 1;
    }

    println!("Output : Vec<i32> = {:?}", memo);

    return memo;
}

/**
### Modifying the input array
- time complexity  = O(N)
- space complexity = O(1)
*/

pub fn sol_2(array: &mut [i32]) -> &[i32] {
    println!();
    println!("Input : [i32;{}] = {:?}", array.len(), array);

    let mut x = 1;

    while x < array.len() {
        array[x] = array[x] + array[x - 1];
        x += 1;
    }

    println!("Input : [i32;{}] = {:?}", array.len(), array);

    return array;
}
