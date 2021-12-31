// Instructions
// Reverse a string

// For example: input: "cool" output: "looc"

// Test your function on this string: uüu and see what happens.
// Try to write a function that properly reverses this string. Hint: grapheme clusters

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
    // input.chars().rev().collect()
}
