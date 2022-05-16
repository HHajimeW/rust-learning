// use std::fmt;

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let laura = Person {
//         name: String::from("Laura"),
//         age: 31,
//         favorite_fruit: String::from("apples"),
//     };

//     let fido = Dog {
//         name: String::from("Fido"),
//         color: String::from("Black"),
//         likes_petting: true,
//     };

//     send_data_as_json(&laura);
//     send_data_as_json(&fido);

//     let kitty = Cat {
//         name: String::from("Kitty"),
//         sharp_claws: false,
//     };

//     send_data_as_json(&kitty)

// }

// struct Cat {
//     name: String,
//     sharp_claws: bool,
// }

// trait AsJson {
//     fn as_json(&self) -> String;
// }

// fn send_data_as_json(value: &impl AsJson){
//     println!("Sending JSON data to server...");
//     println!("-> {}", value.as_json());
//     println!("Done!\n");
// }

// fn send_data_as_json<T: AsJson>(value: &T){
//     println!("Sending JSON data to server...");
//     println!("-> {}", value.as_json());
//     println!("Done!\n");
// }

// struct Person {
//     name: String,
//     age: u8,
//     favorite_fruit: String,
// }

// struct Dog {
//     name: String,
//     color: String,
//     likes_petting: bool,
// }

// impl AsJson for Person {
//     fn as_json(&self) -> String {
//         format!(
//             r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
//             self.name, self.age, self.favorite_fruit
//         )
//     }
// }

// impl AsJson for Dog {
//     fn as_json(&self) -> String {
//         format!(
//             r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
//             self.name, self.color, self.likes_petting
//         )
//     }
// }

// #[derive(Debug)]
// struct Counter {
//     length: usize,
//     count: usize,
// }

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Counter {
//     fn new(length: usize) -> Counter {
//         Counter { count: 0, length }
//     }
// }

// impl Iterator for Counter {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count <= self.length {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     for number in Counter::new(10) {
//         println!("{}", number);
//     }

//     let sum_until_10: usize = Counter::new(10).sum();
//     assert_eq!(sum_until_10, 55);

//     let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
//     assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
// }



struct Container<T> {
    value: T,
}

impl <T> Container<T> {
    pub fn new(value:T) -> Self {
        Container {value}
    }
}


fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}