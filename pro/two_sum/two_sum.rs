pub fn run(nums: Vec<u32>, target: u32) {
    println!("Original array: {:?}", &nums);
    let mut res1 = Vec::new();
    let mut res2 = Vec::new();

    // 1. shortlist the elements smaller than the target
    for n in nums.iter() {
        if n <= &target {
            res1.push(n);
        }
    }
    println!("Shortened array: {:?}", res1);

    // 2. check for 2 elements if they sum up to the target
    for i in 0..res1.len() {
        for j in 1..res1.len() {
            if (i < j) && (res1[i] + res1[j] == target) {
                res2.push(i);
                res2.push(j);
            }
        }
    }
    println!("Final indices: {:?}", res2);
}
