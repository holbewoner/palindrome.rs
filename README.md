[![ci-badge][]][ci] [![docs-badge][]][docs]

# Palindrome

Do you want to check if something is or contains a palindrome? Use this library!

## Examples

If you want to check whether something is a palindrome:

```rust
extern crate palindrome;

assert!(palindrome::is_palindrome("Wow!"));

assert!(palindrome::is_palindrome("Are we not pure? “No sir!” Panama’s moody Noriega brags. “It is garbage!” Irony dooms a man; a prisoner up to new era."));
```
Alternatively, if you want to check whether something contains a palindrome:

```rust
extern crate palindrome;

assert!(palindrome::contains_palindrome("This contains a palindrome, wow!"));
assert!(!palindrome::contains_palindrome("Single characters like a are not palindromes for convenience"));
```

## License

Licensed under MIT

[ci]: https://travis-ci.org/TheUnitedStatesOfAmerica/yn.rs
[ci-badge]: https://travis-ci.org/TheUnitedStatesOfAmerica/yn.rs.svg?branch=master
[docs]: https://docs.rs/yn
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg