#[path = "./v1.rs"]
mod v1;

#[path = "./v2.rs"]
mod v2;

fn main() {
    println!("=======v1: capture only single/1st combination=========");
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    v1::two_sum(nums1, target1);

    // v2
    println!("=======v2: capture all the combination=========");
    let nums2 = vec![2, 7, 2, 15];
    let target2 = 9;
    v2::two_sum(nums2, target2);
}
