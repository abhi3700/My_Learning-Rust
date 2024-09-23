/* 
    print the files in given directory
*/

pub fn run() {
    let paths = std::fs::read_dir("./tuts/error_handling/").unwrap();

    for path in paths {
        // println!("{:?}", path.unwrap().path().display());        // get the filename along with directory
        println!("{:?}", path.unwrap().path().file_name().unwrap());    // get the filename only
    }
}