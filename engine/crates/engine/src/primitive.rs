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
    let (vertices, indices) = build_triangle_mesh(points, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

pub unsafe fn create_rectangle_mesh(
    renderer: &mut VulkanRenderer,
    points: [Vec3; 4],
    color: Vec3,
) -> Result<MeshHandle> {
    let (vertices, indices) = build_rectangle_mesh(points, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

pub unsafe fn create_cube_mesh(
    renderer: &mut VulkanRenderer,
    points: [Vec3; 8],
    color: Vec3,
) -> Result<MeshHandle> {
    let (vertices, indices) = build_cube_mesh(points, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

fn build_triangle_mesh(points: [Vec3; 3], color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex::new(points[0], color, vec2(0.0, 0.0)),
        Vertex::new(points[1], color, vec2(1.0, 0.0)),
        Vertex::new(points[2], color, vec2(0.5, 1.0)),
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}

fn build_rectangle_mesh(points: [Vec3; 4], color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex::new(points[0], color, vec2(0.0, 0.0)),
        Vertex::new(points[1], color, vec2(1.0, 0.0)),
        Vertex::new(points[2], color, vec2(1.0, 1.0)),
        Vertex::new(points[3], color, vec2(0.0, 1.0)),
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    (vertices, indices)
}

pub fn build_cube_mesh(points: [Vec3; 8], color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let faces = [
        [0, 1, 2, 3],
        [4, 7, 6, 5],
        [0, 4, 5, 1],
        [2, 6, 7, 3],
        [1, 5, 6, 2],
        [0, 3, 7, 4],
    ];

    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    for face in faces {
        let base = vertices.len() as u32;

        vertices.push(Vertex::new(points[face[0]], color, vec2(0.0, 0.0)));
        vertices.push(Vertex::new(points[face[1]], color, vec2(1.0, 0.0)));
        vertices.push(Vertex::new(points[face[2]], color, vec2(1.0, 1.0)));
        vertices.push(Vertex::new(points[face[3]], color, vec2(0.0, 1.0)));

        indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
    }

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

pub fn spawn_rectangle(
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

pub fn spawn_cube(world: &mut World, mesh: MeshHandle, transform: Transform) -> Result<EntityId> {
    Ok(world.spawn(
        transform,
        Some(MeshRenderer { mesh }),
        None,
        vec3(0.0, 0.0, 0.0),
    ))
}
