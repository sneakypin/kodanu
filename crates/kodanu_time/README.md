# kodanu_time

Time management for the Kodanu game engine.

This crate provides:

* [`Time`](https://docs.rs/kodanu_time/latest/kodanu_time/struct.Time.html), the engine's time resource
* [`TimePlugin`](https://docs.rs/kodanu_time/latest/kodanu_time/struct.TimePlugin.html), which integrates time updates with the ECS
* frame delta time
* elapsed engine time
* delta-time clamping

## Usage

```rust
use kodanu_time::TimePlugin;

app.add_plugin(TimePlugin);
```

Systems can access the [`Time`](https://docs.rs/kodanu_time/latest/kodanu_time/struct.Time.html) resource through the ECS:

```rust
fn update(time: Res<Time>) {
    let delta = time.delta();

    // Update game state.
}
```

`delta()` and `elapsed()` return time in seconds as `f32`.

## License

MIT OR Apache-2.0
