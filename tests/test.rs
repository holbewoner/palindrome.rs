extern crate palindrome;

#[test]
fn test_full_string() {
  assert!(palindrome::is_palindrome("Ana."));
  assert!(palindrome::is_palindrome("A car, a man, a maraca."));
  assert!(palindrome::is_palindrome("Are we not pure? “No sir!” Panama’s moody Noriega brags. “It is garbage!” Irony dooms a man; a prisoner up to new era."));
  assert!(!palindrome::is_palindrome("This is not a palindrome."));
}

#[test]
fn test_single_words() {
  assert!(palindrome::contains_palindrome("This contains a palindrome: Ana."));
  assert!(palindrome::contains_palindrome("Wow much palindrome."));
  assert!(!palindrome::contains_palindrome("While a is a palindrome, we exclude it for convenience."));
}
