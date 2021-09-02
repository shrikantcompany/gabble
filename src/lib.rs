/*!
Generate random pronounceable words.

## Quickstart
Gabble provides type [`Gab`] and [`GabLength`] that represents a random pseudo-word.
These types implement [`Distribution`](rand::distributions) trait which means they can be generated in an idomatic way by using method from [`rand::Rng`] such as [`gen()`](rand::Rng::gen()).
### Example
``` rust
use rand::{thread_rng, Rng};
let mut rng = thread_rng();

// `Gab` gives gibberish of some moderate length
use gabble::Gab;
let word: Gab = rng.gen();
println!("{} might be answer to life", word);

// `GabLength` gives gibberish of minimum N length
use gabble::GabLength;
let word: GabLength<14> = rng.gen();
println!("{} is a long word", word);
```
[`Gab`] and [`GabLength`] can be treated as string as they simply deref to [`String`].

## Custom Generation
Additionally there is [`Gabble`] type which can be used to generate pseudo-words with a bit more customization. You can tweak length, starting and ending syllable for now.
### Example
``` rust
use gabble::Gabble;
use gabble::Syllable::{Alphabet, Consonant};
use rand::thread_rng;
let mut rng = thread_rng();
// Generator configured to generate words
// that starts with consonant syllable and ends with a number
let gabble = Gabble::new()
    .with_length(10)
    .starts_with(Alphabet)
    .ends_with(Consonant);
println!("customized answer to life is {}", gabble.generate(rng));
```

These pseudo-words are generated by combining vowel and consonant syllables together. This crate is mostly inspired from python package called [gibberish](https://github.com/greghaskins/gibberish)

*/

mod generator;
mod gabble;
mod default;
mod syllables;
mod length;

use crate::syllables::{FINALS, INITIALS, VOWELS};
pub use crate::syllables::Syllable;
pub use crate::gabble::Gabble;
pub use crate::default::Gab;
pub use crate::length::GabLength;