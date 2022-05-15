// fn main() {
//     let mut greeting = String::from("hello");
//     change(&mut greeting);
//     println!("{}", greeting)
// }

// fn change(text: &mut String) {
//     text.push_str(", world");
// }

// fn main() {
//     let mut value = String::from("hello");

//     let ref1 = &value;
//     let ref2 = &mut value;

//     println!("{}, {}", ref1, ref2);
// }

// fn main() {
//     let x;

//     {
//         let y = 42;
//         x = &y;
//     }

//     println!("x: {}", x);
// }

// fn main() {
//     let magic1 = String::from("abracadabra!");
//     let magic2 = String::from("shazam!");

//     let result = longest_word(&magic1, &magic2);
//     println!("The longest magic word is {}", result);
// }

// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// #[derive(Debug)]
// struct Highlight<'document>(&'document str);

// fn erase(_: &String) { }

// fn main() {
//     let text = String::from("The quick brown fox jumps over the lazy dog.");
//     let fox = Highlight(&text[4..19]);
//     let dog = Highlight(&text[35..43]);

//     erase(&text);

//     println!("{:?}", fox);
//     println!("{:?}", dog);
// }

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String{
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

// Vector型の復習
// fn main() {
//     let mut xs = vec![1i32, 2, 3];
//     println!("Initial vector: {:?}", xs);

//     println!("Push 4 into the vector");
//     xs.push(4);
//     println!("Vector: {:?}", xs);

//     println!("Pop last element: {:?}", xs.get(1));
//     println!("Pop last element: {:?}", xs.get(1).unwrap());
// }

// fn give_adult(dringk: Option<&str>) {
//     match drink {
//         Some("lemonade") => println!("Yuck! Too sugary."),
//         Some(inner) => println!("{}? How nice."),
//         None => println!("No drink? Oh well."),
//     }
// }