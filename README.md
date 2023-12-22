# Try It
Add this to your Cargo.toml:

```
[dependencies.typenum-convert]
git = "https://github.com/fcard/typenum-convert"
branch = "1.0.0"
```

# Example

```rs
use typenum_convert::*;
use typenum::{U4, Unsigned};
use std::ops::Add;

fn main() {
  type Four = U4;
  type Eight = T32<{2 * Four::U32}>;
  type Twelve = T32<{Eight::U32 + Four::U32}>;
  println!("{} {} {}",
    Four::U32, Eight::U32, Twelve::U32);
}
```
