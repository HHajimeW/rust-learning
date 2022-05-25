// mod authentication {
//     use std::collections::hash_map::DefaultHasher;
//     use std::hash::{Hash, Hasher};
//     pub struct User {
//         username: String,
//         password_hash: u64,
//     }

//     impl User {
//         pub fn new(username: &str, password: &str) -> User {
//             User {
//                 username: username.to_string(),
//                 password_hash: hash_password(&password.to_owned()),
//             }
//         }

//         pub fn get_username(&self) -> &String {
//             &self.username
//         }

//         pub fn set_password(&mut self, new_password: &str) {
//             self.password_hash = hash_password(&new_password.to_owned())
//         }
//     }
//     fn hash_password<T: Hash>(t: &T) -> u64 {
//         let mut s = DefaultHasher::new();
//         t.hash(&mut s);
//         s.finish()
//     }
// }

// fn main() {
//     let mut user = authentication::User::new("jeremy", "super-secret");

//     let s = "hello world"; // スタック
//     let str = String::from("hello world"); // ヒープ

//     println!("The username is: {}", user.get_username());
//     user.set_password("even-more-secret");
// }

// use regex::Regex;

// fn main() {
//     let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//     println!("Did our date match? {}", re.is_match("2014-01-01"));
// }

// mod car_factory {
//     pub fn build_car() {
//         println!("Honk honk!");
//     }
// }

// fn main() {
//     car_factory::build_car();
// }

mod text_processing {
    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod number {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::number::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

fn main() {
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}