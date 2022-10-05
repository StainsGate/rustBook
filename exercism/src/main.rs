mod anagram;
mod clock;
mod gigasecond;
mod luhn;
mod minesweeper;
mod plf;
mod reverse;
mod sublist;
use anagram::anagrams_for;
use clock::*;
use luhn::is_valid;
use minesweeper::annotate;
use plf::frequency;
use reverse::reverse;
use std::collections::HashMap;
use sublist::sublist;
fn main() {
    // println!("{}", reverse("dynamic"))
    // let years = 1000000000 / (365 * 24 * 3600);
    // let days = (1000000000 - (365 * 24 * 3600 * years)) / (24 * 3600);

    // println!("years : {}", years);
    // println!("days : {}", days);

    // let c1 = Clock::new(2, 20).add_minutes(-3000);
    // let c2 = Clock::new(7, 32);
    // println!("clock1: {}", c1);
    // println!("clock2: {}", c2);

    // assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));

    // let str1 = "clock";
    // let str_array = ["world", "letsin", "inlets", "lliitt", "ListeN"];
    // let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    // for i in str1.chars() {
    //     println!("{}", i);
    // }

    // let result = anagrams_for("listen", &str_array);
    // let result = anagrams_for("ΑΒΓ", &inputs);

    // println!("result : {}", result.len());
    // for word in result.iter() {
    //     println!("{}", word);
    // }

    // let list1 = [1, 2, 3, 4, 5];
    // let list2 = [1, 2, 3, 4, 5, 7];
    // let v: &[u32] = &[];

    // use std::iter::repeat;
    // let v: Vec<char> = repeat('x').take(1000).collect();
    // let v: &[u32] = &[];

    // let output = sublist(&v, &v);

    // for i in 0..list2.len() {
    //     println!("{}", i);
    // }
    // let v1 = [1, 2, 3, 4, 5];
    // let v2 = [1, 2, 3, 4, 5];

    // let str2 = "..*..";
    // let str1 = ".*.*.";
    // let str3 = "..*..";
    // let str4 = ".....";

    // let matrix = &[str1, str2, str3, str4];

    // let bytes = str.as_bytes();

    // println!(" char :{}", bytes[2])

    // for i in bytes.into_iter() {
    //     println!(" char : {}", i == &42);
    // }

    // println!("result : {:?}", output);

    // let result = annotate(&[]);

    // for str in result.into_iter() {
    //     println!("{}", str);
    // }

    // println!("result:{}", is_valid("095 245 88"));
    // let mut str = String::new();
    // let num: u32 = 8;
    // let ch = char::from_digit(num, 10).unwrap();
    // str.push(ch);
    // println!("num: {}", str);

    // let array = [1, 2, 3, 4, 5, 6, 7];

    // let a1 = &array[0..2];
    // let a2 = &array[2..4];
    // let a3 = &array[4..7];

    // for i in a1 {
    //     println!("num: {}", i);
    // }

    // for j in a2 {
    //     println!("num2: {}", j);
    // }

    // for k in a3 {
    //     println!("num3: {}", k);
    // }

    // test(&array, 3);

    // let text: [&str; 2] = ["Freude schöner Götterfunken", "Tochter aus Elysium,"];

    // let result = frequency(&text, 2);

    // println!("result: {:?}", result)

    // let mut map: HashMap<char, usize> = HashMap::new();

    // counter(&text, map);

    let vec = myvec!('a' => 3, 'b' => 11, 'z' => 32);

    for (k, v) in vec {
        println!("key {}", k);
        println!("value {}", v);
    }
}

#[macro_export]
macro_rules! myvec {
    ( $( $x:literal => $y:expr ),* ) => {
        {
            let mut temp_map: HashMap<char, usize>= HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}
