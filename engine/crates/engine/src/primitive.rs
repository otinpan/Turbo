pub type Vec3 = cgmath::Vector3<f32>;
use anyhow::Result;
use cgmath::{vec2, vec3};
use renderer_vulkan::{Vertex, VulkanRenderer};
use turbo_math::Transform;

use super::EntityId;
use super::MeshHandle;
use super::MeshRenderer;
use super::World;

pub unsafe fn create_triangle_mesh(
    renderer: &mut VulkanRenderer,
    points: [Vec3; 3],
    color: Vec3,
) -> Result<MeshHandle> {
    let (vertices, indices) = triangle_2d(points, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

fn triangle_2d(points: [Vec3; 3], color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex::new(points[0], color, vec2(0.0, 0.0)),
        Vertex::new(points[1], color, vec2(1.0, 0.0)),
        Vertex::new(points[2], color, vec2(0.5, 1.0)),
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}

pub fn spawn_triangle(
    world: &mut World,
    mesh: MeshHandle,
    transform: Transform,
) -> Result<EntityId> {
    Ok(world.spawn(
        transform,
        Some(MeshRenderer { mesh }),
        None,
        vec3(0.0, 0.0, 0.0),
    ))
}
