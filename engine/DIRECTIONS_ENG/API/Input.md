# Input API

Input API provides keyboard, mouse, and window-size state.

It is available from `UpdateContext` and `CommandContext`.

```rust
if context.key_down(KeyCode::KeyW) {
    // Runs while W is held down.
}
```

KaniVolcano stores input events from `winit` in `Input`. `InputApi` reads that stored state.

## Input States

Keyboard keys and mouse buttons have three state checks.

- `down`: `true` every frame while held
- `pressed`: `true` only on the frame it was pressed
- `released`: `true` only on the frame it was released

`pressed`, `released`, and `mouse_delta` are cleared at the end of each frame.

## Keyboard

```rust
context.key_down(KeyCode::KeyW);
context.key_pressed(KeyCode::Space);
context.key_released(KeyCode::Space);
```

Use `key_down()` for continuous movement. Use `key_pressed()` for one-shot actions such as jumping or spawning an object.

## Mouse

```rust
let position = context.mouse_position();
let delta = context.mouse_delta();
```

`mouse_position()` returns the current cursor position in window coordinates. The origin is the top-left of the window.

`mouse_delta()` returns how much the mouse moved during the current frame.

Mouse buttons use the same three state checks.

```rust
context.mouse_button_down(MouseButton::Right);
context.mouse_button_pressed(MouseButton::Left);
context.mouse_button_released(MouseButton::Left);
```

## Window Size

```rust
let size = context.window_size();
```

Returns the current window size as `Vector2<f32>`. `x` is width and `y` is height.

This is useful when converting mouse position to normalized screen coordinates or world coordinates.

```rust
let mouse = context.mouse_position();
let size = context.window_size();

let x = mouse.x / size.x.max(1.0);
let y = mouse.y / size.y.max(1.0);
```

## Command Binding

`SceneContext::bind_input_command()` binds a key input to a command.

```rust
context.bind_input_command(
    KeyCode::Digit1,
    InputTrigger::Pressed,
    SpawnPrimitiveCommand {
        primitive_type: PrimitiveType::Triangle,
        pipeline_key: PipelineKey::Lit3D,
        texture_name: None,
    },
);
```

`InputTrigger` has three variants.

- `InputTrigger::Pressed`: run when the key is pressed
- `InputTrigger::Down`: run every frame while the key is held
- `InputTrigger::Released`: run when the key is released

Internally, `InputSystem` checks input every frame and pushes matching commands into `CommandQueue`.

## Example: Camera Input

```rust
let mouse_delta = context.mouse_delta();
let right_mouse_down = context.mouse_button_down(MouseButton::Right);
let move_forward = context.key_down(KeyCode::KeyW);

if right_mouse_down {
    camera.yaw -= mouse_delta.x * mouse_sensitivity;
    camera.pitch -= mouse_delta.y * mouse_sensitivity;
}
```

## Note

`bind_input_command()` currently binds keyboard input. For mouse input, read mouse state inside `UpdateSystem` or `Command`.