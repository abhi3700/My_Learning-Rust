fn sum_list(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    println!("sum of vector is: {}", sum_list(vec![1, 2, 6, 8]));
}
