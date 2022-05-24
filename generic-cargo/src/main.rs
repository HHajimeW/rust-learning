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



// struct Container<T> {
//     value: T,
// }

// impl <T> Container<T> {
//     pub fn new(value:T) -> Self {
//         Container {value}
//     }
// }


// fn main() {
//     assert_eq!(Container::new(42).value, 42);
//     assert_eq!(Container::new(3.14).value, 3.14);
//     assert_eq!(Container::new("Foo").value, "Foo");
//     assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
//     assert_eq!(Container::new(true).value, true);
//     assert_eq!(Container::new(-12).value, -12);
//     assert_eq!(Container::new(Some("text")).value, Some("text"));
// }

struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq + std::fmt::Debug> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element ==first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    //groups:
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}