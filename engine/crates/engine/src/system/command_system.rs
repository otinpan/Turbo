use anyhow::{Result, anyhow};
use cgmath::{Vector3, vec3};
use renderer_vulkan::{MeshHandle, PipelineKey, TextureHandle, VulkanRenderer};
use std::collections::HashMap;
use turbo_math::Transform;

use super::InputCommand;
use crate::app::DEFAULT_TEXTURE;
use crate::primitive::{
    PrimitiveMesh, PrimitiveShape, PrimitiveType, spawn_primitive_from_mesh, update_primitive_mesh,
};
use crate::{Resources, Input, Material, MeshRenderer, World};

pub type Vec3 = Vector3<f32>;

pub struct CommandContext<'a> {
    pub commands: &'a mut Vec<InputCommand>,
    pub world: &'a mut World,
    pub renderer: &'a mut VulkanRenderer,
    pub input: &'a Input,
    pub resources: &'a mut Resources,

    pub positions: &'a Vec<Vec3>,
}

#[derive(Clone, Debug)]
pub struct CommandSystem;

impl CommandSystem {
    pub fn update(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let commands = context.commands.drain(..).collect::<Vec<_>>();

        for command in commands {
            self.execute(command, context)?;
        }

        Ok(())
    }

    fn execute(&self, command: InputCommand, context: &mut CommandContext<'_>) -> Result<()> {
        match command {
            InputCommand::DespawnLast => {
                let id = context.world.registry.entities().last().copied();

                if let Some(id) = id {
                    context.world.despawn(id);
                }
            }
            InputCommand::SpawnVikingRoom => {
                self.spawn_viking_room_from_input(context)?;
            }
            InputCommand::SpawnPrimitive {
                primitive_type,
                pipeline_key,
                texture_name,
            } => {
                self.spawn_primitive_from_input(
                    primitive_type,
                    pipeline_key,
                    texture_name,
                    context,
                );
            }
            InputCommand::UpdatePrimitiveMeshes => {
                self.update_primitive_meshes_from_input(context)?;
            }
        }

        Ok(())
    }

    fn spawn_viking_room_from_input(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let viking_room_mesh3d = Self::use_model(&context.resources.models, "viking_room")?;
        let viking_room_debug_line = Self::use_model(&context.resources.models, "viking_room_debug_line")?;
        let viking_room_lit3d = Self::use_model(&context.resources.models, "viking_room_lit3d")?;
        let viking_texture = Self::use_texture(&context.resources.textures, "viking_room");
        let viking_meshes = [
            viking_room_mesh3d,
            viking_room_debug_line,
            viking_room_lit3d,
        ];
        let index = context
            .world
            .registry
            .entities()
            .iter()
            .filter(|entity| {
                context
                    .world
                    .registry
                    .get_component::<MeshRenderer>(**entity)
                    .is_some_and(|mesh_renderer| viking_meshes.contains(&mesh_renderer.mesh))
            })
            .count();

        if context.positions.len() > index {
            let variants = [
                (viking_room_mesh3d, PipelineKey::Mesh3D, 1.0),
                (viking_room_debug_line, PipelineKey::DebugLine3D, 1.0),
                (viking_room_mesh3d, PipelineKey::Transparent3D, 0.5),
                (viking_room_lit3d, PipelineKey::Lit3D, 1.0),
            ];
            let (mesh, pipeline_key, alpha) = variants[index];
            match MeshRenderer::new(
                mesh,
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    alpha,
                    use_texture: true,
                    texture: viking_texture,
                    pipeline_key,
                },
            ) {
                Ok(mesh_renderer) => {
                    context.world.spawn(
                        Transform {
                            position: context.positions[index],
                            ..Default::default()
                        },
                        Some(mesh_renderer),
                        None,
                        vec3(20.0, 0.0, 0.0),
                    );
                }
                Err(e) => {
                    log::error!("Failed to spawn triangle primitive: {e:?}");
                }
            };
        }

        Ok(())
    }

    fn spawn_primitive_from_input(
        &self,
        primitive_type: PrimitiveType,
        pipeline_key: PipelineKey,
        texture_name: Option<&'static str>,
        context: &mut CommandContext<'_>,
    ) {
        let position = Self::mouse_position_on_spawn_plane(context.input);
        let texture = texture_name
            .map(|name| Self::use_texture(&context.resources.textures, name))
            .unwrap_or(DEFAULT_TEXTURE);

        if let Some(mesh) = Self::primitive_handle(&context.resources.primitive_meshes, primitive_type) {
            if let Err(e) = spawn_primitive_from_mesh(
                context.world,
                mesh,
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    use_texture: true,
                    texture,
                    pipeline_key,
                    ..Default::default()
                },
                Transform {
                    position,
                    ..Default::default()
                },
            ) {
                log::error!("Failed to spawn {primitive_type:?} primitive: {e:?}");
            }
        }
    }

    fn update_primitive_meshes_from_input(&self, context: &mut CommandContext<'_>) -> Result<()> {
        if let Some(mesh) = Self::primitive_mesh(&context.resources.primitive_meshes, PrimitiveType::Polygon) {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    mesh,
                    PrimitiveShape::Polygon {
                        points: vec![
                            vec3(0.0, -0.7, 0.3),
                            vec3(0.0, -0.4, 0.2),
                            vec3(0.0, 0.7, 0.5),
                            vec3(0.0, 0.2, -0.2),
                            vec3(0.0, -0.5, -0.45),
                        ],
                        color: vec3(1.0, 0.0, 0.0),
                    },
                )?;
            }
        }
        if let Some(mesh) = Self::primitive_mesh(&context.resources.primitive_meshes, PrimitiveType::Sphere) {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    mesh,
                    PrimitiveShape::Sphere {
                        radius: 2.0,
                        rings: 20,
                        segments: 20,
                        color: vec3(0.0, 1.0, 1.0),
                    },
                )?;
            }
        }
        if let Some(mesh) = Self::primitive_mesh(&context.resources.primitive_meshes, PrimitiveType::Rectangle)
        {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    mesh,
                    PrimitiveShape::Rectangle {
                        points: [
                            vec3(0.0, -0.2, 0.2),
                            vec3(0.0, -0.2, -0.2),
                            vec3(0.0, 0.2, -0.2),
                            vec3(0.0, 0.2, 0.2),
                        ],
                        color: vec3(1.0, 1.0, 1.0),
                    },
                )?;
            }
        }

        Ok(())
    }

    fn primitive_handle(
        primitive_meshes: &[PrimitiveMesh],
        primitive_type: PrimitiveType,
    ) -> Option<MeshHandle> {
        Self::primitive_mesh(primitive_meshes, primitive_type).map(|mesh| mesh.handle)
    }

    fn primitive_mesh(
        primitive_meshes: &[PrimitiveMesh],
        primitive_type: PrimitiveType,
    ) -> Option<PrimitiveMesh> {
        primitive_meshes
            .iter()
            .find(|mesh| mesh.primitive_type == primitive_type)
            .copied()
    }

    fn mouse_position_on_spawn_plane(input: &Input) -> Vec3 {
        let mouse = input.mouse_position();
        let window_size = input.window_size();
        let width = window_size.x.max(1.0);
        let height = window_size.y.max(1.0);
        let aspect = width / height;
        let world_height = 4.0;

        let x = mouse.x / width - 0.5;
        let y = 0.5 - mouse.y / height;

        vec3(0.0, x * world_height * aspect, y * world_height)
    }

    fn use_texture(textures: &HashMap<String, TextureHandle>, name: &str) -> TextureHandle {
        textures.get(name).copied().unwrap_or(DEFAULT_TEXTURE)
    }

    fn use_model(models: &HashMap<String, MeshHandle>, name: &str) -> Result<MeshHandle> {
        models
            .get(name)
            .copied()
            .ok_or_else(|| anyhow!("Model not found: {name}"))
    }
}
