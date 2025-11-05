dry run publish

# bitstates

Reactive bit state management with event callbacks for Rust.

## Installation

```toml
[dependencies]
bitstates = "0.1.0"
```

## Example

```rust
use bitstates::BitStates;

#[derive(BitStates)]
#[repr(u8)]
enum Status {
    Ready = 0,
    Active = 1,
}

fn main() {
    let mut status = StatusStates::new(
        |flag| println!("SET: {:?}", flag),
        |flag| println!("CLEARED: {:?}", flag),
    );

    status.set_with_state(0b01);
}
```

## License

MIT OR Apache-2.0
