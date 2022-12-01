# auto_into

this macro was born out of this [internals.rust discusssion](https://internals.rust-lang.org/t/into-function-parameters/17870/3).

i've come around to thinking that this is an anti-pattern, i don't recommend you actually use this create. but developing it was cool and it works as a proof-of-concept if this discussion is ever brought up again.

## Usage

here's the code example
```rust
use auto_into::auto_into;

#[auto_into]
fn takes_whatever(#[into] _: String) { }

fn main() {
	takes_whatever("lorem"); // &str
	takes_whatever(std::borrow::Cow::Borrowed("ipsum")); // moo
	takes_whatever(format!("{} sit", "dolor")); // String
	takes_whatever('a'); // char
}
```

oh, and in `#[into] _: String`, the `_` is a pattern, not an identifier, we support destructuring too.
