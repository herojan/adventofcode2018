//use std::error::Error;
//use std::fs::File;
//use std::io::BufRead;
//use std::io::BufReader;
//
//
//fn main() -> Result<(), Box<Error>> {
//    let f = File::open("input.txt")?;
//    let f = BufReader::new(f);
//    let sum = f.lines().try_fold(
//        0,
//        |acc, s: Result<String, std::io::Error>| -> Result<i32, Box<Error>> {
//            let s: String = s?;
//            let n = s.parse::<i32>()?;
//
//            Ok(acc + n)
//        },
//    );
//
//    Ok(println!("{}", sum?))
//}
