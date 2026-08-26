use anyhow::{Result, anyhow};
use cgmath::vec3;
use renderer_vulkan::VertexLayout;

use super::{Command, CommandContext};
use crate::AssetApi;
use crate::RenderCommandApi;
use crate::primitive::{PrimitiveShape, PrimitiveType};
#[derive(Clone, Debug)]
pub struct UpdatePrimitiveMeshesCommand;

impl Command for UpdatePrimitiveMeshesCommand {
    fn id(&self) -> String {
        "update_primitive_meshes".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let polygon_mesh3d = context
            .primitive_asset_id(PrimitiveType::Polygon, VertexLayout::Mesh3D)
            .ok_or_else(|| anyhow!("not found polygon mesh3d"))?;
        context.update_primitive_mesh(
            polygon_mesh3d,
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

        let sphere_lit3d = context
            .primitive_asset_id(PrimitiveType::Sphere, VertexLayout::Lit3D)
            .ok_or_else(|| anyhow!("not found sphere_lit3d"))?;

        context.update_primitive_mesh(
            sphere_lit3d,
            PrimitiveShape::Sphere {
                radius: 2.0,
                rings: 20,
                segments: 20,
                color: vec3(0.0, 1.0, 1.0),
            },
        );

        let rectangle_ui2d = context
            .primitive_asset_id(PrimitiveType::Rectangle, VertexLayout::Ui2D)
            .ok_or_else(|| anyhow!("not found rectangle ui2d"))?;
        context.update_primitive_mesh(
            rectangle_ui2d,
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
