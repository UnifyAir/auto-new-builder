# auto-new-builder

![README AI Generated](https://img.shields.io/badge/README-AI%20Generated-blue)

A lightweight, zero-dependency Rust derive macro for automatically generating builder patterns with a clean and intuitive API.

## Features

- 🚀 Automatically generates `new()` and builder methods
- 🎯 Support for optional fields with `Option<T>`
- 💫 Clean and intuitive API
- 🛠 Zero runtime overhead
- 📦 No external dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto-new-builder = { git = "https://github.com/UnifyAir/auto-new-builder.git", package = "auto-new-builder", branch = "master" }
```

## Usage

The `AutoNewBuilder` derive macro automatically generates a constructor and builder methods for your structs. Here's a simple example:

```rust
use auto_new_builder::auto_new_builder_derive::AutoNewBuilder;

#[derive(AutoNewBuilder, Debug)]
pub struct Hello {
    world: u32,
    next_world: Option<u32>
}

fn main() {
    // Basic usage with just required fields
    let hello = Hello::new(21);
    println!("{:?}", hello.world);        // Prints: 21
    println!("{:?}", hello.next_world);   // Prints: None

    // Using the builder pattern for optional fields
    let hello_with_next = Hello::new(42).with_next_world(32);
    println!("{:?}", hello_with_next.world);      // Prints: 42
    println!("{:?}", hello_with_next.next_world); // Prints: Some(32)
}
```

## How It Works

The `AutoNewBuilder` derive macro generates:

1. A `new()` constructor that takes all required fields (non-Option fields)
2. Builder methods (`with_*`) for each optional field
3. Maintains clean method chaining for the builder pattern

## Generated Code

For the example above, the macro generates something equivalent to:

```rust
impl Hello {
    pub fn new(world: u32) -> Self {
        Self {
            world,
            next_world: None,
        }
    }

    #[inline]
    pub fn with_next_world(mut self, next_world: u32) -> Self {
        self.next_world = Some(next_world);
        self
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

Created with ❤️ by UnifyAir
