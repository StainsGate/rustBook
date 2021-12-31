// use core::fmt;s
use enum_iterator::IntoEnumIterator;
// use int_enum::IntEnum;
// use std::fmt::{self, Display};

// #[repr(u8)]
#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Blue => 6,
        ResistorColor::Brown => 1,
        ResistorColor::Green => 5,
        ResistorColor::Grey => 8,
        ResistorColor::Orange => 3,
        ResistorColor::Red => 2,
        ResistorColor::Violet => 7,
        ResistorColor::White => 9,
        ResistorColor::Yellow => 4,
    };

    // let result = ResistorColor::into_enum_iter()
    //     .enumerate()
    //     .filter(|(_, v)| v == &_color)
    //     .map(|(k, _)| k);

    // println!("{}", result)
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => String::from("Black"),
        6 => String::from("Blue"),
        1 => String::from("Brown"),
        5 => String::from("Green"),
        8 => String::from("Grey"),
        3 => String::from("Orange"),
        2 => String::from("Red"),
        7 => String::from("Violet"),
        9 => String::from("White"),
        4 => String::from("Yellow"),
        _ => String::from("Nothing"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    // let mut result = Vec::new();
    // let iter = ResistorColor::into_enum_iter();
    // for value in iter {
    //     result.push(value);
    // }
    // result.sort_by(|a,b| )
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}

pub fn create_empty(count: usize) -> Vec<u8> {
    // unimplemented!()
    return vec![0; count];
}

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        if self.health == 0 {
            if self.level >= 10 {
                Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                })
            } else {
                Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                })
            }
        } else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)

        if self.mana == None {
            if self.health > mana_cost {
                self.health -= mana_cost
            }
            return 0;
        } else if self.mana < Some(mana_cost) {
            return 0;
        } else {
            let mana = self.mana.unwrap();
            self.mana = Some(mana - mana_cost);
            return mana_cost * 2;
        }
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    // let result = iter.advance_by(2).into_iter();
    // result = iter.advance_by(2);
    // let mut result = std::iter::empty::<T>();

    return iter
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, e)| e);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_value() {
        // assert_eq!(9, color_to_value(ResistorColor::White));
    }

    #[test]
    fn test_to_string() {
        // let result = value_to_color_string(9);
        assert_eq!("White", value_to_color_string(9));
    }

    #[test]
    fn test_output_vec() {
        assert_eq!(
            vec![
                ResistorColor::Black,
                ResistorColor::Brown,
                ResistorColor::Red,
                ResistorColor::Orange,
                ResistorColor::Yellow,
                ResistorColor::Green,
                ResistorColor::Blue,
                ResistorColor::Violet,
                ResistorColor::Grey,
                ResistorColor::White,
            ],
            colors()
        )
    }

    #[test]
    fn test_vec() {
        assert_eq!(vec![0; 3], create_empty(3))
    }

    #[test]
    fn test_events() {
        // let mut iter = evens([0, 1, 2, 3, 4].iter());
        let mut iter = evens(1_i16..);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
    }
}
