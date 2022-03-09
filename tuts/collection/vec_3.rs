/* 
    Check if a item is unique when compared to given number
*/

fn check_dup(v: Vec<i32>, num: i32) -> Result<bool, String> {
    let mut is_unique: bool = false;

    if !v.contains(&num) {
        is_unique = true;
    }

    Ok(is_unique)
}

fn main() {
    let v: Vec<i32> = vec![1, 4, 7, 34, 57];

    let num: i32 = 567;
    
    println!("Is item unique? -> {}", check_dup(v, num).expect("not found"));

}
