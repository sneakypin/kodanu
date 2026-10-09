# kodanu_math

A lightweight mathematics library for the **Kodanu** game engine, written in Rust.

`kodanu_math` provides foundational mathematical types and operations used in 2D and 3D graphics, transformations, and game development.

## Features

* **Vectors:** `Vec2`, `Vec3`, and `Vec4`
* **Quaternions:** `Quat` for representing 3D rotations
* **Matrices:** `Mat4` for transformations and projection
* **Mouse input types:** `MousePos` and `MouseDelta`
* Vector arithmetic and common geometric operations
* Rotation, scaling, translation, and perspective projection utilities

## Usage

```rust
use kodanu_math::{Vec3, Quat};

fn main() {
    let position = Vec3::new(1.0, 2.0, 3.0);
    let offset = Vec3::new(2.0, 0.0, -1.0);

    let new_position = position + offset;

    let rotation = Quat::from_rotation_y(90.0);
    let direction = rotation * Vec3::FORWARD;

    println!("Position: {new_position:?}");
    println!("Direction: {direction:?}");
}
```

## Documentation

Generate the API documentation locally:

```bash
cargo doc -p kodanu_math --no-deps --open
```

Run the crate's tests:

```bash
cargo test -p kodanu_math
```

## License

Licensed under the license specified by the Kodanu workspace.
