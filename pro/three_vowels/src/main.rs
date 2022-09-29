/*
    To find out 3 consecutive vowels in a word.
*/

fn three_vowels(w: &str) -> bool {
    let mut count = 0;
    for c in w.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
                // println!("{c}: {count}");
                if count >= 3 {
                    return true;
                }
            }
            _ => count = 0,
        }
    }

    return false;
}

fn three_non_vowels(w: &str) -> bool {
    let mut count = 0;
    for c in w.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
                println!("{c}: {count}");
                if count >= 3 {
                    return true;
                }
            }
            _ => (),
        }
    }

    return false;
}

fn main() {
    // let sentence = "Curious is a good boy".to_string();

    // for word in sentence.split(" ") {
    //     if three_vowels(word) {
    //         println!("3 consecutive vowels found");
    //     }
    // }

    let word = "Abhijit Roy";

    println!("{}", three_non_vowels(word));
}
