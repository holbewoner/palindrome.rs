//! # Palindrome
//! 
//! Do you want to check if something is or contains a palindrome? Use this library!
//! 
//! ## Examples
//! 
//! If you want to check whether something is a palindrome:
//! 
//! ```rust
//! extern crate palindrome;
//! 
//! assert!(palindrome::is_palindrome("Wow!"));
//! 
//! assert!(palindrome::is_palindrome("Are we not pure? “No sir!” Panama’s moody Noriega brags. “It is garbage!” Irony dooms a man; a prisoner up to new era."));
//! ```
//! Alternatively, if you want to check whether something contains a palindrome:
//! 
//! ```rust
//! extern crate palindrome;
//! 
//! assert!(palindrome::contains_palindrome("This contains a palindrome, wow!"));
//! assert!(!palindrome::contains_palindrome("Single characters like a are not palindromes for convenience"));
//! ```
//! 
//! ## License
//! 
//! Licensed under MIT

use std::fmt::Display;

/// Determines if a value is a palindrome
/// 
/// Returns true when the value is a palindrome, this can be a sentence or a single word.
/// 
/// # Examples
/// 
/// Is a palindrome:
/// ```rust
/// assert!(palindrome::is_palindrome("Ana."));
/// assert!(palindrome::is_palindrome("A car, a man, a maraca."));
/// ```
/// 
/// Is not a palindrome:
/// ```rust
/// assert!(!palindrome::is_palindrome("Driving. Toot toot."));
/// ```
/// While it contains a palindrome, it's not a palindrome by itself.
/// Use `contains_palindrome` to check if it contains a palindrome.
pub fn is_palindrome(string: impl Display) -> bool {
    check(string)
}

/// Determines if a value contains a palindrome.
/// 
/// Returns true when the value contains a palindrome, like a sentence containing a palindrome.
/// 
/// # Examples
/// 
/// Contains a palindrome:
/// ```rust
/// assert!(palindrome::contains_palindrome("Ana."));
/// assert!(palindrome::contains_palindrome("This contains a palindrome: Ana."));
/// ```
/// 
/// Does not contain a palindrome:
/// ```rust
/// assert!(!palindrome::contains_palindrome("While a is a palindrome, we exclude it for convenience."));
/// ```
pub fn contains_palindrome(string: impl Display) -> bool {
    check_sentence(string)
}

fn check(string: impl Display) -> bool {
    let s = string.to_string().to_lowercase().chars().filter(|&c| c.is_alphanumeric()).collect::<String>();
    s == s.chars().rev().collect::<String>()
}

fn check_sentence(string: impl Display) -> bool {
    string.to_string().to_lowercase().split_whitespace().any(|w| {
        let word = w.chars().filter(|&c| c.is_alphanumeric()).collect::<String>();
        word.len() > 1 && word == word.chars().rev().collect::<String>()
    })
}
