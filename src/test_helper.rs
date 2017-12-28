use std::io::Read;
use std::str;
use std::fmt;
use std::collections::vec_deque::VecDeque;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn load_test_cases<T>(filepath: &str) -> VecDeque<T>
    where T: str::FromStr, T::Err: fmt::Debug {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    let mut deque = VecDeque::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {
            let numbers = s.trim()
                .split(|c| c == '\n' || c == ' ')
                .collect::<Vec<&str>>();
            for num in &numbers {
                deque.push_back(num.parse().unwrap());
            }
        }
    }
    deque
}