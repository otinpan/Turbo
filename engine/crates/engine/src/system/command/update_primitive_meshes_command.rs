use anyhow::Result;
use cgmath::vec3;

use super::{Command, CommandContext};
use crate::primitive::{PrimitiveShape, PrimitiveType, update_primitive_mesh};

#[derive(Clone, Debug)]
pub struct UpdatePrimitiveMeshesCommand;

impl Command for UpdatePrimitiveMeshesCommand {
    fn id(&self) -> String {
        "update_primitive_meshes".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        if let Some(mesh) = primitive_mesh(context, PrimitiveType::Polygon) {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    context.resources,
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
        if let Some(mesh) = primitive_mesh(context, PrimitiveType::Sphere) {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    context.resources,
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
        if let Some(mesh) = primitive_mesh(context, PrimitiveType::Rectangle) {
            unsafe {
                update_primitive_mesh(
                    context.renderer,
                    context.resources,
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
}

fn primitive_mesh(
    context: &CommandContext<'_>,
    primitive_type: PrimitiveType,
) -> Option<crate::PrimitiveMesh> {
    context
        .resources
        .primitive_meshes
        .iter()
        .find(|mesh| mesh.primitive_type == primitive_type)
        .copied()
}
