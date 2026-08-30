# チュートリアル
まずTurboEngineを使えるようにしましょう。`Cargo.toml`に以下を加えてください。
```
[dependencies]
anyhow = "1"
cgmath = "0.18"
turbo_engine = { git = "https://github.com/otinpan/turbo" }
turbo_math = { git = "https://github.com/otinpan/turbo" }
```

`main.rs`を編集して、ビルドしてみましょう
```rust
use anyhow::{Result};
use turbo_engine::prelude::*;
fn main()  -> Result<()>{
    pretty_env_logger::init();
    run(
        EngineConfig {
            title: "Vulkan Tutorial".to_string(), 
            width: 1024, 
            height: 768
        },
        |app|{
            Ok(())
        }
    )
}
```
`run`関数を呼び出します。1つ目の引数では`EngineConfig`でwindowのタイトル、幅、高さを設定します。第2引数ではクロージャを呼び出します。今後はここでモデルや画像のロード、シーンの登録などを記述していきます。この段階でビルドしてみましょう。おそらく１分くらいかかりますが成功するはずです。
```
PS D:\MyGame\turbo_tutorial> cargo build
   Compiling turbo_tutorial v0.1.0 (D:\MyGame\turbo_tutorial)
```
では`cargo run`で実行してみましょう。
```
The vertical field of view cannot be below zero, found: 0.0 rad
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\turbo_tutorial.exe` (exit code: 101)The vertical field of view cannot be below zero, found: 0.0 rad
```
panicしてしまいました。この原因は、Sceneとカメラをまだ作っていないからです。ではSceneを作成し、その中でカメラを作りましょう。Sceneとはゲーム内の1つの画面や場面をまとめて管理する単位です。`tutorial_scene.rs`に`TutorialScene`として作りましょう。
```rust
pub struct TutorialScene{}

impl Scene for TutorialScene{
    fn name(&self) -> String {
        "TutorialScene".to_string()
    }

    // this is called when this scene is spawned.
    fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()>{
        // Entity作成
        let camera=context.spawn();

        // Component付与
        context.add_component(
            camera,
            Transform{
                position: vec3(1.0,0.0,0.0),
                ..Default::default()
            }
        );
        context.add_component(
            camera,
            Camera{
                target: vec3(0.0,0.0,0.0),
                fov_y: 45.0,
                near: 0.1, // if objects are within `near`, they are not rendered.
                far: 100.0, // if objects are far than `far`, they are not rendered.
                yaw: std::f32::consts::PI, // horizontal way to move
                pitch: 0.0, // vertical way to move
            },
        );
        Ok(())
    }

    // this is called every frame.
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>{
        Ok(())
    }

    // this is called, when this scene is despawed.
    fn on_exit(&mut self, context: &mut SceneContext<'_>) -> Result<()>{
        Ok(())
    }

}
```

Sceneは`Scene`トレイトを持つ構造体として作ります。`Scene`トレイトは
* `name()`: このSceneの名前を返す関数
* `on_enter()`: このSceneに切り替わったときに呼ばれる関数
* `update()`: このScene特有の更新関数。毎フレーム呼ばれる
* `on_exit()`: このSceneから他のSceneに切り替わる際に呼ばれる関数

を実装します。固定カメラを作るだけなので、`on_enter()`にカメラを作成します。
```rust
let camera=context.spwan();
```
でEntityを作成します。この時点では、ただIdを作成しただけなのでカメラの機能はありません。次に、Componentを付与します。
```rust
context.add_component(
    camera,
    Transform{
        position: vec3(1.0,0.0,0.0),
        ..Default::default()
    }
);
context.add_component(
    camera,
    Camera{
        target: vec3(0.0,0.0,0.0),
        fov_y: 45.0,
        near: 0.1, // if objects are within `near`, they are not rendered.
        far: 100.0, // if objects are far than `far`, they are not rendered.
        yaw: std::f32::consts::PI, // horizontal way to move
        pitch: 0.0, // vertical way to move
    },
);
```
`add_component(entity, component)`でEntityに対してComponentを付与します。カメラは`Transform`コンポーネントと`Camera`コンポーネントを持つ必要があります。この2つを持って初めてカメラとして画面を描画するのです。これは、このエンジンが`Transform`と`Camera`コンポーネントを持つ、Entityの座標情報から画面の見方をVulkanに伝えているからです。
main関数に作ったSceneを加えましょう。
```rust
fn main()  -> Result<()>{
    pretty_env_logger::init();
    run(
        EngineConfig {
            title: "Vulkan Tutorial".to_string(), 
            width: 1200, 
            height: 600
        },
        |app|{
            app.add_scene(TutorialScene{})?;
            app.set_current_scene("TutorialScene")?;
            Ok(())
        }
    )
}
```
`add_scene()`でSceneを登録し、`set_current_scene()`で登録されたSceneを適用します。実行すると以下のようにwindowが作成されます。
![](../../assets/window.png)

しかし、これでは面白くないですね。図形を作成しましょう。  
`on_enter()`に図形を作成します。
```rust
fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()>{
    self.create_camera(context)?;

    let sphere=context.spawn_sphere_3d(
        vec3(-1.0,-0.7,0.0), // center
        0.2, // radius
        32, // rings
        32, // segments
        vec3(1.0,0.0,0.0), // color
        0.5, // alpha
        None, // texture
        PipelineKey::Transparent3D, // pipeline
    );
    let cube=context.spawn_cube_3d(
        vec3(-1.0,0.0,0.0), // center
        0.4, // length
        vec3(0.0,0.0,45.0), // rotation
        vec3(1.0,1.0,0.0), // color
        1.0, // alpha
        None, // texture
        PipelineKey::DebugLine3D, // pipeline
    );
    let triangle=context.spawn_triangle_2d(
        vec2(0.5,0.5), // vertex0
        vec2(0.8,-0.2), // vertex1
        vec2(0.4,-0.4), // vertex2
        vec3(0.0,0.0,1.0), // color
        0.5, // alpha
        None, // texture
    );
    Ok(())
}
```
するとこのように描画されるはずです。
![](../../assets/tutorial_primitives.png)

ここでは詳しいことは述べませんが、位置や色、透明度、texture等を指定することで簡単に図形を作成し描画させることが出来ます。  
次はモデルと画像をロードして描画してみましょう。まずは`main.rs`でモデルと画像をロードします。
```rust
...
    |app|{
        load_assets(app)?;
        app.add_scene(TutorialScene{})?;
        app.set_current_scene("TutorialScene")?;
        Ok(())
    }

...
fn load_assets(app: &mut App) -> Result<()>{
    unsafe{
        // load model from assets/models
        app.load_model(
            "viking_room_lit3d",
            "assets/models/viking_room.obj",
            PipelineKey::Lit3D,
            false,
        )?;

        // load textures from assets/textures/
        app.load_texture(
            "ghost",
            "assets/textures/ghost.png",
        )?;
        app.load_texture(
            "viking_room",
            "assets/textures/viking_room.png",
        )?;
    }

    Ok(())
}
```
そしてSceneでモデル用のEntityを作成します。
```rust
fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()>{
    self.create_camera(context)?;
    self.create_primitives(context)?;

    let viking_texture=context
        .texture("viking_room")
        .unwrap_or(context.default_texture());

    let viking_room=context.spawn_model(
        "viking_room_lit3d",
        Transform { 
            position: vec3(-3.0,0.0,0.0), 
            rotation: vec3(0.0,0.0,-45.0),
            scale: vec3(1.0,1.0,1.0),
        },
        Material { 
            color: vec3(1.0,1.0,1.0),
            alpha: 1.0,
            use_texture: true, 
            texture: viking_texture,
            pipeline_key: PipelineKey::Lit3D // this Pipeline key must match selected model's PipelineKey
        }
    );

    Ok(())
}
```
![](../../assets/tutorial_model.png)

`spawn_model(model_name, Transform, Material)`でロードしたモデルを作成します。これも先ほどのカメラや図形同様、EntityでありComponentも自動的に付与されています。`Material`とはどのように描画するかをしてします。ここで大事なのは、Materialフィールドの`model_name`と`pipeline`が`load_model()`で指定したモデルの名前と`pipeline`で一致している必要があるということです。これが一致していなかったら、描画されません。また、`viking_texture`を指定していますが、
```rust
let viking_texture=context
    .texture("viking_room")
    .unwrap_or(context.default_texture());
```
画像がまだ登録されていなかったら、真っ白な画像が適用されます。  
次は背景を作ってみましょう

次にComponentとSystem


