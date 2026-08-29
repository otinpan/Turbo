#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod basic_3d_scene;

use anyhow::Result;
use basic_3d_scene::Basic3dScene;
use turbo_engine::{App, EngineConfig, PipelineKey, run};

fn main() -> Result<()> {
    pretty_env_logger::init();

    run(
        EngineConfig {
            title: "Vulkan Tutorial (Rust)".to_string(),
            width: 1024,
            height: 768,
        },
        |app| {
            load_assets(app)?;
            app.add_scene(Basic3dScene::default())?;
            app.set_current_scene("Basic3dScene")?;
            Ok(())
        },
    )
}

fn load_assets(app: &mut App) -> Result<()> {
    // load models
    unsafe {
        load_models(app)?;
        load_textures(app)?;
        load_skybox_textures(app)?;
    }
    Ok(())
}

unsafe fn load_models(app: &mut App) -> Result<()> {
    app.load_model(
        "viking_room",
        "assets/models/viking_room.obj",
        PipelineKey::Mesh3D,
        false,
    )?;
    app.load_model(
        "viking_room_debug_line",
        "assets/models/viking_room.obj",
        PipelineKey::DebugLine3D,
        false,
    )?;
    app.load_model(
        "viking_room_lit3d",
        "assets/models/viking_room.obj",
        PipelineKey::Lit3D,
        false,
    )?;
    Ok(())
}

unsafe fn load_textures(app: &mut App) -> Result<()> {
    app.load_texture("viking_room", "assets/textures/viking_room.png")?;
    app.load_texture("face", "assets/textures/texture.png")?;
    app.load_texture("ghost", "assets/textures/ghost.png")?;
    app.load_texture("escapee", "assets/textures/escapee.png")?;

    Ok(())
}

unsafe fn load_skybox_textures(app: &mut App) -> Result<()> {
    app.load_skybox_texture(
        "escapee",
        [
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
        ],
    )?;
    app.load_skybox_texture(
        "ghost",
        [
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
        ],
    )?;

    Ok(())
}
