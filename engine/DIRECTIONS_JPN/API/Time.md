# Time API

Time APIは、フレーム間の経過時間や、アプリケーション開始からの経過時間を取得するためのAPIです。
`UpdateContext`から使用できます。

```rust
use kani_volcano_engine::prelude::*;

let delta_time = context.delta_seconds();
```

KaniVolcanoでは、毎フレームの更新開始時に`Time`が更新されます。
`TimeApi`は、その時間情報を`Scene::update()`や`UpdateSystem`から読み取るための窓口です。

## delta_seconds

```rust
let delta_time: f32 = context.delta_seconds();
```

`delta_seconds()`は、前回のフレームから現在のフレームまでに経過した秒数を返します。
戻り値の単位は秒です。

フレームレートに依存しない移動や回転を行うときに使います。
例えば、毎秒`2.0`だけ移動させたい場合は、速度に`delta_seconds()`を掛けます。

```rust
let speed = 2.0;
let delta_time = context.delta_seconds();

transform.position.x += speed * delta_time;
```

このように書くことで、フレームレートが高い環境でも低い環境でも、ほぼ同じ速度で移動します。

## elapsed_seconds

```rust
let elapsed: f32 = context.elapsed_seconds();
```

`elapsed_seconds()`は、アプリケーション開始から現在までに経過した秒数を返します。
戻り値の単位は秒です。

時間に応じて周期的に変化する処理や、一定時間後に挙動を変える処理に使えます。

```rust
let t = context.elapsed_seconds();
let y = t.sin();

transform.position.y = y;
```

## UpdateSystemで使う例

`UpdateSystem`では、`UpdateContext`からTime APIを呼び出せます。

```rust
use anyhow::Result;
use kani_volcano_engine::prelude::*;

pub struct MoveSystem;

pub struct MoveComponent {
    pub velocity: cgmath::Vector3<f32>,
}

impl Component for MoveComponent {}

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

この例では、`MoveComponent`の速度に`delta_seconds()`を掛けて、フレーム間の移動量を計算しています。

## 回転に使う例

`RotatorSystem`では、`delta_seconds()`を使って回転量を計算しています。

```rust
impl UpdateSystem for RotatorSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let delta_time = context.delta_seconds();

        for (_, transform, rotator) in context.query2_mut::<Transform, Rotator>() {
            transform.rotate(rotator.speed * delta_time);
        }

        Ok(())
    }
}
```

`rotator.speed`が1秒あたりの回転速度だとすると、`rotator.speed * delta_time`がそのフレームで実際に回転する量になります。

## Sceneのupdateで使う例

Sceneの`update()`にも`UpdateContext`が渡されるため、Time APIを使えます。

```rust
impl Scene for TutorialScene {
    fn name(&self) -> String {
        "TutorialScene".to_string()
    }

    fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let elapsed = context.elapsed_seconds();

        if elapsed > 3.0 {
            log::debug!("3 seconds passed");
        }

        Ok(())
    }

    fn on_exit(&mut self, context: &mut SceneContext<'_>) -> Result<()> {
        Ok(())
    }
}
```

## 内部の流れ

`App::update()`では、毎フレーム最初に`self.time.update()`が呼ばれます。
そこで現在時刻と前回フレーム時刻の差分から`delta_seconds`が計算され、開始時刻との差分から`elapsed_seconds`が計算されます。

```rust
self.time.update();
```

その後、更新処理に渡される`UpdateContext`が`Time`を参照し、`delta_seconds()`や`elapsed_seconds()`を使えるようになります。

## 注意点

`delta_seconds()`は前回フレームからの経過時間です。
移動量や回転量をフレームレートに依存させたくない場合に使います。

`elapsed_seconds()`はアプリケーション開始からの経過時間です。
アニメーション、タイマー、周期的な動きなどに使えます。

`TimeApi`は現状`UpdateContext`で使うAPIです。
`CommandContext`や`SceneContext`から直接`delta_seconds()`を呼ぶ用途にはなっていません。
Commandで時間情報を使いたい場合は、必要な値を別のComponentなどに保持しておく設計にすると扱いやすくなります。