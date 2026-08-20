use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::VertexLayout;

use super::{Command, CommandContext};
use crate::primitive::{PrimitiveShape, PrimitiveType};

#[derive(Clone, Debug)]
pub struct UpdatePrimitiveMeshesCommand;

impl Command for UpdatePrimitiveMeshesCommand {
    fn id(&self) -> String {
        "update_primitive_meshes".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        context.update_primitive_mesh(
            PrimitiveType::Polygon,
            VertexLayout::Mesh3D,
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
        );

        context.update_primitive_mesh(
            PrimitiveType::Sphere,
            VertexLayout::Lit3D,
            PrimitiveShape::Sphere {
                radius: 2.0,
                rings: 20,
                segments: 20,
                color: vec3(0.0, 1.0, 1.0),
            },
        );

        context.update_primitive_mesh(
            PrimitiveType::Rectangle,
            VertexLayout::Ui2D,
            PrimitiveShape::Rectangle {
                points: [
                    vec3(0.0, -0.2, 0.2),
                    vec3(0.0, -0.2, -0.2),
                    vec3(0.0, 0.2, -0.2),
                    vec3(0.0, 0.2, 0.2),
                ],
                color: vec3(1.0, 1.0, 1.0),
            },
        );

        Ok(())
    }
}
