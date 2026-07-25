pub type Vec3 = cgmath::Vector3<f32>;
use anyhow::Result;
use cgmath::{vec2, vec3};
use renderer_vulkan::{Vertex, VulkanRenderer};
use turbo_math::Transform;

use super::EntityId;
use super::MeshHandle;
use super::MeshRenderer;
use super::World;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Triangle,
    Rectangle,
    Cube,
    Circle,
    Polygon,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimitiveMesh {
    pub handle: MeshHandle,
    pub primitive_type: PrimitiveType,
}

// create mesh ////////////////////////////////////////////
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

pub unsafe fn create_circle_mesh(
    renderer: &mut VulkanRenderer,
    radius: f32,
    segments: u32,
    color: Vec3,
) -> Result<MeshHandle> {
    let (vertices, indices) = build_circle_mesh(radius, segments, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

pub unsafe fn create_polygon_mesh(
    renderer: &mut VulkanRenderer,
    points: Vec<Vec3>,
    color: Vec3,
) -> Result<MeshHandle> {
    let (vertices, indices) = build_polygon_mesh(points, color);
    Ok(MeshHandle(
        renderer.load_mesh_from_vertices(vertices, indices)?,
    ))
}

// build mesh. create vertices and indices from points //////////////////////////////////////////////////
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

pub fn build_circle_mesh(radius: f32, segments: u32, color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let segments = segments.max(3);
    let mut vertices = Vec::with_capacity(segments as usize + 1);
    let mut indices = Vec::with_capacity(segments as usize * 3);

    vertices.push(Vertex::new(vec3(0.0, 0.0, 0.0), color, vec2(0.5, 0.5)));

    for i in 0..segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        let y = angle.cos() * radius;
        let z = angle.sin() * radius;
        let u = angle.cos() * 0.5 + 0.5;
        let v = angle.sin() * 0.5 + 0.5;

        vertices.push(Vertex::new(vec3(0.0, y, z), color, vec2(u, v)));
    }

    for i in 0..segments {
        let current = i + 1;
        let next = if i + 1 == segments { 1 } else { i + 2 };

        indices.extend_from_slice(&[0, current, next]);
    }

    (vertices, indices)
}

fn build_polygon_mesh(points: Vec<Vec3>, color: Vec3) -> (Vec<Vertex>, Vec<u32>) {
    let size = points.len();

    if size < 3 {
        return (Vec::new(), Vec::new());
    }

    let mut vertices = Vec::with_capacity(size);
    let mut indices = Vec::with_capacity((size - 2) * 3);

    let mut min_y = points[0].y;
    let mut max_y = points[0].y;
    let mut min_z = points[0].z;
    let mut max_z = points[0].z;

    for p in &points {
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
        min_z = min_z.min(p.z);
        max_z = max_z.max(p.z);
    }

    let width = max_y - min_y;
    let height = max_z - min_z;

    for p in &points {
        let u = if width.abs() > f32::EPSILON {
            (p.y - min_y) / width
        } else {
            0.0
        };
        let v = if height.abs() > f32::EPSILON {
            (p.z - min_z) / height
        } else {
            0.0
        };

        vertices.push(Vertex::new(*p, color, vec2(u, v)));
    }

    indices.extend(triangulate_polygon_yz(&points));

    (vertices, indices)
}

// create triangle from polygon using era clipping
fn triangulate_polygon_yz(points: &[Vec3]) -> Vec<u32> {
    let size = points.len();
    let mut indices = Vec::with_capacity((size - 2) * 3);
    let area = signed_area_yz(points);

    if area.abs() <= f32::EPSILON {
        return indices;
    }

    let mut remaining: Vec<usize> = if area > 0.0 {
        (0..size).collect()
    } else {
        (0..size).rev().collect()
    };

    while remaining.len() > 3 {
        let mut ear_index = None;

        for i in 0..remaining.len() {
            let prev = remaining[(i + remaining.len() - 1) % remaining.len()];
            let curr = remaining[i];
            let next = remaining[(i + 1) % remaining.len()];

            if is_ear_yz(points, prev, curr, next, &remaining) {
                ear_index = Some(i);
                indices.extend_from_slice(&[prev as u32, curr as u32, next as u32]);
                break;
            }
        }

        if let Some(i) = ear_index {
            remaining.remove(i);
        } else {
            return fan_triangulate(size);
        }
    }

    indices.extend_from_slice(&[
        remaining[0] as u32,
        remaining[1] as u32,
        remaining[2] as u32,
    ]);

    indices
}

fn signed_area_yz(points: &[Vec3]) -> f32 {
    let mut area = 0.0;

    for i in 0..points.len() {
        let current = points[i];
        let next = points[(i + 1) % points.len()];
        area += current.y * next.z - next.y * current.z;
    }

    area * 0.5
}

fn is_ear_yz(points: &[Vec3], prev: usize, curr: usize, next: usize, remaining: &[usize]) -> bool {
    let a = points[prev];
    let b = points[curr];
    let c = points[next];

    if cross_yz(a, b, c) <= f32::EPSILON {
        return false;
    }

    for &index in remaining {
        if index == prev || index == curr || index == next {
            continue;
        }

        if point_in_triangle_yz(points[index], a, b, c) {
            return false;
        }
    }

    true
}

fn cross_yz(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    (b.y - a.y) * (c.z - a.z) - (b.z - a.z) * (c.y - a.y)
}

fn point_in_triangle_yz(point: Vec3, a: Vec3, b: Vec3, c: Vec3) -> bool {
    let ab = cross_yz(a, b, point);
    let bc = cross_yz(b, c, point);
    let ca = cross_yz(c, a, point);

    ab >= -f32::EPSILON && bc >= -f32::EPSILON && ca >= -f32::EPSILON
}

fn fan_triangulate(size: usize) -> Vec<u32> {
    let mut indices = Vec::with_capacity((size - 2) * 3);

    for i in 1..(size - 1) {
        indices.extend_from_slice(&[0, i as u32, (i + 1) as u32]);
    }

    indices
}

// spawn primitice object //////////////////////////////////////////////
pub fn spawn_primitive(
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

// update mesh ///////////////////////////////////////////////////////////
// refered mesh in VulkanData update vertices and indices
pub unsafe fn update_mesh(
    renderer: &mut VulkanRenderer,
    mesh: MeshHandle,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
) -> Result<()> {
    renderer.update_mesh_from_vertices(mesh.0, vertices, indices)
}

pub unsafe fn update_polygon_mesh(
    renderer: &mut VulkanRenderer,
    mesh: MeshHandle,
    points: Vec<Vec3>,
    color: Vec3,
) -> Result<()> {
    
    let (vertices, indices) = build_polygon_mesh(points, color);
    update_mesh(renderer, mesh, vertices, indices)
}

pub unsafe fn update_triangle_mesh(
    renderer: &mut VulkanRenderer,
    mesh: MeshHandle,
    points: [Vec3; 3],
    color: Vec3,
) -> Result<()> {
    let (vertices, indices) = build_triangle_mesh(points, color);
    update_mesh(renderer, mesh, vertices, indices)
}
