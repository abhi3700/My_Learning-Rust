pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    println!("Original array: {:?}", &nums);
    let mut res: Vec<(i32, i32)> = Vec::new();

    // check for 2 elements if they sum up to the target
    for (i, ei) in nums.iter().enumerate() {
        for (j, ej) in nums.iter().enumerate() {
            // left index < right index
            if (i < j) && (ei + ej == target) {
                res.push((i as i32, j as i32));
            }
        }
    }
    println!("Final indices: {:?}", res);

    return res;
}
