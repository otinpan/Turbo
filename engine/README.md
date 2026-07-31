# Turbo engine architecture

# Turbo Engine TODO
* ~~`App`、`World`、`Renderer`、`Input`、`Time`の責務を分ける~~
* ~~Entityを導入 -> ゲーム内オブジェクトをIDで管理する~~
* ~~Transform、MeshRenderer、Cameraなどのコンポーネント構造を決める~~
* ~~カメラの作成~~
* ~~入力、更新、描画を分けてゲームループを作る~~
* ~~カメラを操作できるように、キーボード入力を作る~~
  - ~~press~~
  - ~~down~~
  - ~~up~~
* ~~2D図形のヘルパー関数~~
* マテリアル
  - ~~Material構造体作成~~
    - ~~color~~
  - ~~renderer_vulkanに渡す~~
    - ~~RenderItemにmaterial_color~~
    - ~~command_buffer更新~~
      - ~~push constantのrangeを変える (pipeline.rs)~~
      - ~~update_secondary_bufferの更新 (command.rs)~~
    - ~~shader更新~~
* Pipeline複数生成
  - 一旦pipelineは１つだけ
  - VulkanDataのpipelineを`Vec<GraphicsPipeline>`にする
  - Material has RenderMode <-> RenderItem has pipeline_key
  - pipeline_key -> GraphicsPipeline
  - vulkan_renderer
    - type.rs : vulkan_data, renderitem
    - pipeline.rs : create_pipeline()
    - commands.rs
    - lib.rs : swapchain
* ~~Image複数作成~~
  - ~~Texture struct と textures: Vec<Texture> を作る~~
  - ~~create_texture_image(path) -> Texture に変える~~
  - ~~shader の set/binding を分ける~~
  - ~~create_descriptor_set_layout を global/material に分割~~
  - ~~pipeline layout に両方渡す~~
  - ~~create_global_descriptor_sets を作る~~
  - ~~create_material_descriptor_sets を作る~~
  - ~~RenderItem に material_index または texture_index を持たせる~~
  - ~~command buffer で global set と material set を bind~~

    

## ログ表示
```rust
log::trace!("trace"); // 詳細情報
log::debug!("debug"); // デバッグに役立つ内部情報
log::info!("info"); // 主要イベント
log::warn!("warn"); // 処理は継続できるが注意が必要
log::error!("error");
```

```
RUST_LOG=debug cargo run --release
RUST_LOG=debug cargo run
```
では
```
debug
info
warn
error
```
が表示される

```rust
#[cfg(debug_assertions)]
log::debug!("camera = {:?}, camera");
```
はreleaseではコンパイルされない

## コアシステム
* ~~アサーション~~
* メモリ管理
* 数学ライブラリ
* ~~Debugビルド用の設定~~
* ~~Releaseビルド用の設定~~
* 独自のアルゴリズムやデータ構造
  - Handle: オブジェクト識別子
  ```rust
  pub struct Handle<T> {
    index: u32,
    generation: u32,
    _marker: std::marker::PhantomData<T>,
  }
  ```

## グラフィックス
* ~~Transformを作成し、`RenderObject`が個別に動けるようにする~~
* マテリアル
* ~~カメラの作成~~
  - ~~平行移動~~
  - ~~前後移動~~
  - ~~方向変更 (Mouse)~~
* pipeline複数生成
* シェーダーを自由に選択できるようにする
* 2d図形のヘルパー関数
  - ~~三角形~~
  - ~~四角形~~
  - ~~直方体~~
  - ~~円~~
  - ~~ポリゴン~~
  - 平面
  - ~~球~~
  - 線分 (pipeline後)
* shaderのコンパイル手順の自動化
* フォント
* ビューポートと仮想スクリーンによるマルチディスプレイ


## ゲームループとアプリ構造
* ~~`App`、`World`、`Renderer`、`Input`、`Time`の責務を分ける~~
* ウィンドウイベント処理をアプリ本体から切り出す
* ~~Entityを導入し、ゲーム内オブジェクトをIDで管理する~~
* ~~Transform、MeshRenderer、Cameraなどのコンポーネント構造を検討する~~
* シーン内のオブジェクト追加、削除、検索を安全に行えるようにする
* 固定更新と可変更新の扱いを決める
* ポーズ、ステップ実行、リセットを実装する
* FPS、フレーム時間、描画オブジェクト数を表示またはログ出力する
* 設定ファイルからウィンドウサイズ、VSync、MSAA、アセットパスを読み込む

## アセット管理

* アセットディレクトリ構成を決める
* モデル、テクスチャ、シェーダーをアセットIDで参照できるようにする
* 同じアセットを二重ロードしないキャッシュを作る
* 読み込み失敗時に代替アセットや分かりやすいエラーを出す
* mipmap生成の有無をアセット単位で選べるようにする
* sampler設定をテクスチャごとに持てるようにする

## ビジュアルエフェクト
* パーティクルシステム
* デカールシステム
* ライトマップと環境マップ
* 動的シャドウ

## 描画品質
* PhongまたはBlinn-Phongの基本ライティングを実装する
* directional light、point light、spot lightを扱えるようにする
* 法線を使った陰影計算を追加する
* ガンマ補正とsRGBテクスチャの扱いを整理する
* アルファブレンドを実装し、透明オブジェクトを描画できるようにする
* 不透明パスと透明パスを分ける
* face culling、front face、depth test、depth writeの設定を整理する
* ワイヤーフレーム表示や法線表示などのデバッグ描画を追加する
* skyboxまたは背景描画を追加する
* シャドウマップを実装する
* bloom、tone mapping、color gradingなどのポストプロセスを検討する
* render passを複数パス構成に拡張できるようにする


## 入力と操作

* キーボード入力を押下、押している間、離した瞬間で扱えるようにする
* マウス移動、クリック、ホイール入力を扱えるようにする
* 入力マッピングを作り、キー設定を変更できるようにする
* カメラ操作、オブジェクト選択、デバッグ操作を入力システム経由にする

## 物理とゲーム機能
* Open Dynamics Engineのようなエンジンを探す

