# Time API

Time API provides the time between frames and the elapsed time since the application started.

It is available from `UpdateContext`.

```rust
let delta_time = context.delta_seconds();
```

## `delta_seconds()`

```rust
let delta_time: f32 = context.delta_seconds();
```

Returns the time in seconds since the previous frame.

Use it for frame-rate independent movement, rotation, and animation.

```rust
let speed = 2.0;
let delta_time = context.delta_seconds();

transform.position.x += speed * delta_time;
```

## `elapsed_seconds()`

```rust
let elapsed: f32 = context.elapsed_seconds();
```

Returns the time in seconds since the application started.

Use it for timers and time-based animation.

```rust
let t = context.elapsed_seconds();
transform.position.y = t.sin();
```

## Example

```rust
impl UpdateSystem for MoveSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let delta_time = context.delta_seconds();

        for (_, transform, mover) in context.query2_mut::<Transform, MoveComponent>() {
            transform.position += mover.velocity * delta_time;
        }

        Ok(())
    }
}
```

`Time` is updated at the beginning of every `App::update()` call.

`TimeApi` is currently intended for `UpdateContext`. `SceneContext` and `CommandContext` do not directly provide `delta_seconds()`.