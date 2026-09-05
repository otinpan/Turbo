# Input API

Input APIは、キーボード、マウス、ウィンドウサイズなどの入力状態を取得するためのAPIです。
`UpdateContext`と`CommandContext`から使用できます。

```rust
use kani_volcano_engine::prelude::*;
use winit::keyboard::KeyCode;

if context.key_down(KeyCode::KeyW) {
    // Wキーが押されている間の処理
}
```

KaniVolcanoでは、`winit`から届いた入力イベントを`Input`に記録します。
`InputApi`は、その記録された入力状態をScene、System、Commandから読み取るための窓口です。

## 入力状態の種類

Input APIでは、キーやマウスボタンの状態を次の3種類で取得できます。

* `down`: 押されている間、毎フレーム`true`
* `pressed`: 押された瞬間のフレームだけ`true`
* `released`: 離された瞬間のフレームだけ`true`

`pressed`、`released`、`mouse_delta`のような一時的な入力状態は、毎フレームの最後にリセットされます。
一方、`down`はキーやボタンが押され続けている間維持されます。

## キーボード入力

### キーが押されているか

```rust
let forward: bool = context.key_down(KeyCode::KeyW);
```

指定したキーが現在押されているかを取得します。
押されている間は毎フレーム`true`になります。
移動処理のように、押している間ずっと続けたい処理に向いています。

```rust
if context.key_down(KeyCode::KeyW) {
    transform.position.x += 1.0 * context.delta_seconds();
}
```

### キーが押された瞬間か

```rust
let pressed: bool = context.key_pressed(KeyCode::Space);
```

指定したキーが押された瞬間だけ`true`になります。
ジャンプ、決定、オブジェクト生成のように、1回だけ実行したい処理に向いています。

```rust
if context.key_pressed(KeyCode::Space) {
    let entity = context.spawn();
}
```

### キーが離された瞬間か

```rust
let released: bool = context.key_released(KeyCode::Space);
```

指定したキーが離された瞬間だけ`true`になります。
キーを離したタイミングで処理したい場合に使います。

## マウス入力

### マウス座標

```rust
let mouse = context.mouse_position();
```

現在のマウス座標を取得します。
戻り値は`Vector2<f32>`です。
座標はウィンドウ左上を原点とし、右方向が`x`、下方向が`y`です。

`SpawnPrimitiveCommand`では、マウス座標とウィンドウサイズから、オブジェクトを生成するワールド座標を計算しています。

```rust
fn mouse_position_on_spawn_plane(context: &CommandContext<'_>) -> cgmath::Vector3<f32> {
    let mouse = context.mouse_position();
    let window_size = context.window_size();
    let width = window_size.x.max(1.0);
    let height = window_size.y.max(1.0);
    let aspect = width / height;
    let world_height = 4.0;

    let x = mouse.x / width - 0.5;
    let y = 0.5 - mouse.y / height;

    vec3(0.0, x * world_height * aspect, y * world_height)
}
```

### マウス移動量

```rust
let delta = context.mouse_delta();
```

前回のフレームから現在までのマウス移動量を取得します。
カメラ操作やドラッグ操作に使えます。

`CameraSystem`では、右クリック中のマウス移動量を使ってカメラの向きを更新しています。

```rust
let mouse_delta = context.mouse_delta();
let right_mouse_down = context.mouse_button_down(MouseButton::Right);

if right_mouse_down {
    camera.yaw -= mouse_delta.x * mouse_sensitivity;
    camera.pitch -= mouse_delta.y * mouse_sensitivity;
}
```

### マウスボタンが押されているか

```rust
let down: bool = context.mouse_button_down(MouseButton::Left);
```

指定したマウスボタンが押されている間、毎フレーム`true`になります。

### マウスボタンが押された瞬間か

```rust
let pressed: bool = context.mouse_button_pressed(MouseButton::Left);
```

指定したマウスボタンが押された瞬間だけ`true`になります。

### マウスボタンが離された瞬間か

```rust
let released: bool = context.mouse_button_released(MouseButton::Left);
```

指定したマウスボタンが離された瞬間だけ`true`になります。

## ウィンドウサイズ

```rust
let size = context.window_size();
```

現在のウィンドウサイズを取得します。
戻り値は`Vector2<f32>`で、`x`が幅、`y`が高さです。

マウス座標を正規化したり、画面上の位置をワールド座標に変換したりするときに使います。

```rust
let mouse = context.mouse_position();
let size = context.window_size();

let normalized_x = mouse.x / size.x.max(1.0);
let normalized_y = mouse.y / size.y.max(1.0);
```

## Commandとの連携

Input APIは入力状態を直接読むためのAPIですが、KaniVolcanoにはキー入力とCommandを紐づける仕組みもあります。
`SceneContext`の`bind_input_command()`を使うと、指定したキー入力が発生したときにCommandを実行できます。

```rust
fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()> {
    context.bind_input_command(
        KeyCode::Digit1,
        InputTrigger::Pressed,
        SpawnPrimitiveCommand {
            primitive_type: PrimitiveType::Triangle,
            pipeline_key: PipelineKey::Lit3D,
            texture_name: None,
        },
    );

    Ok(())
}
```

`bind_input_command()`には、キー、入力の種類、実行するCommandを渡します。

```rust
context.bind_input_command(KeyCode, InputTrigger, Command);
```

`InputTrigger`には次の3種類があります。

* `InputTrigger::Pressed`: キーが押された瞬間にCommandを実行
* `InputTrigger::Down`: キーが押されている間、毎フレームCommandを実行
* `InputTrigger::Released`: キーが離された瞬間にCommandを実行

内部では、`InputSystem`が毎フレーム入力状態を確認し、条件に合うCommandを`CommandQueue`に追加します。
その後、`CommandSystem`が`CommandContext`を使ってCommandを実行します。

## UpdateSystemで使う例

`UpdateSystem`では、`UpdateContext`からInput APIを呼び出せます。
押されている間ずっと処理したい場合は、`key_down()`を使います。

```rust
use anyhow::Result;
use kani_volcano_engine::prelude::*;
use winit::keyboard::KeyCode;

pub struct PlayerMoveSystem;

impl UpdateSystem for PlayerMoveSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let speed = 2.0;
        let dt = context.delta_seconds();
        let move_left = context.key_down(KeyCode::KeyA);
        let move_right = context.key_down(KeyCode::KeyD);

        for (_, transform) in context.query1_mut::<Transform>() {
            if move_left {
                transform.position.y -= speed * dt;
            }
            if move_right {
                transform.position.y += speed * dt;
            }
        }

        Ok(())
    }
}
```

## Commandで使う例

`Command`では、`CommandContext`からInput APIを呼び出せます。
Commandが実行された瞬間のマウス位置を使って、図形を生成できます。

```rust
use anyhow::Result;
use cgmath::{vec3, Vector3};
use kani_volcano_engine::prelude::*;

pub struct SpawnAtMouseCommand;

type Vec3 = Vector3<f32>;

impl Command for SpawnAtMouseCommand {
    fn id(&self) -> String {
        "spawn_at_mouse".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let position = mouse_position_on_spawn_plane(context);

        context.spawn_rectangle_3d(
            position,
            0.3,
            0.3,
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            1.0,
            None,
            PipelineKey::Lit3D,
        )?;

        Ok(())
    }
}

fn mouse_position_on_spawn_plane(context: &CommandContext<'_>) -> Vec3 {
    let mouse = context.mouse_position();
    let window_size = context.window_size();
    let width = window_size.x.max(1.0);
    let height = window_size.y.max(1.0);
    let aspect = width / height;
    let world_height = 4.0;

    let x = mouse.x / width - 0.5;
    let y = 0.5 - mouse.y / height;

    vec3(0.0, x * world_height * aspect, y * world_height)
}
```

## 注意点

`key_pressed()`と`key_released()`は、その入力が発生したフレームだけ`true`になります。
押しっぱなしの処理には`key_down()`を使います。

`mouse_delta()`はフレーム中に発生したマウス移動量です。
毎フレームの最後に`0.0`へ戻ります。

`mouse_position()`はウィンドウ上の座標です。
3D空間上の座標として使う場合は、ウィンドウサイズやカメラ情報を使って変換する必要があります。

`bind_input_command()`は現在キー入力に対応しています。
マウスボタンを直接Commandに紐づけるAPIは、現状では用意されていません。
マウス入力を使いたい場合は、`UpdateSystem`や`Command`内で`mouse_button_down()`などを呼び出します。