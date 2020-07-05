# Finite Repr

This create exists to encode & decode data types with a finite
number of representations. _English please!_ Okay, okay, this
crates allows you to convert your data types into numbers & back
(or any other data type that implements `FiniteValue`), assuming
the number of possible representations for that data type can be
counted with the type you are encoding too.

## Should You use this?

Probably not, probably just stick with serde.

## Before you use this

This crate has alpha levels of reliablity & isn't likley to be
the most efficent approach to encode data in your appplication.

It will also panic if you encode a large type into a small data
type in some cases. While I do intend to address these issues I
would encourage it's use at this point.

## Okay but how do I use this...

I would recommend deriving the implementations, instead
writing them by hand to ensure they're isomorphic & correct.

```rust
// If you choose to import it, this is how you would do so.
use finite_repr::{FiniteRepr, FiniteDecoding, FiniteEncoding};

#[derive(FiniteRepr, FiniteDecoding, FiniteEncoding)]
struct Character(pub RpgClass, pub Faction);

#[derive(FiniteRepr, FiniteDecoding, FiniteEncoding)]
enum RpgClass { Mage, Knight }

#[derive(FiniteRepr, FiniteDecoding, FiniteEncoding)]
enum Faction {
  GoodGuys,
  ComicallyEvilBadGuys,
  AntagonistWhoMakesYouQuestionYourOwnSenseOfMoralityByTheEndOfThePlot,
}

impl PartialEq for Character { /* ... */ }

fn main() {
  let my_character = Character(RpgClass::Mage, Faction::GoodGuys);
  let encoded = my_character.into_finite::<u16>();
  let decoded = encoded.and_then(Character::from_finite);

  // This assertion will be true.
  assert_eq!(Some(my_character), decoded);
}
```

## Why does this exist?

This crate is the byproduct of bike shedding on a side project
where finishing the project was optional. The main reason I
deployed it on crates.io was to simplify reuse between my own
projects.

Also I thought it would be fun to work on, but also I wanted a way
to encode some data types with very limited possible representations
in my game as numbers, as I was passing them through a trait object
& couldn't make it generic.


## Shortcomings of this crate

- The code from the macros could be better.

- The crate is likely misusing `usize` in `FiniteRepr`, &
  should probalby be replaced with a type whose size does
  not vary. I'll replace this eventually but atm I just
  want to start using this crate in my code.

- No support for union types, it coudld probably be added
  but it just doesn't exist at this point in time.

- It doesn't always gracefully handle failure in the event of
  an integer overflow.

- This crate likely isn't suitable for encoding data that has
  a large amount of representations, such as a `u128` or even
  `(u32, u32)`.
