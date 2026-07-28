use anyhow::{Result, bail};
use cgmath::{vec2, vec3};
use renderer_vulkan::{Vertex, VulkanRenderer};
use turbo_math::Transform;

use super::EntityId;
use super::MeshHandle;
use super::MeshRenderer;
use super::World;

pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2=cgmath::Vector2<f32>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Triangle,
    Rectangle,
    Cube,
    Circle,
    Polygon,
    Sphere,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimitiveMesh {
    pub handle: MeshHandle,
    pub primitive_type: PrimitiveType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveShape {
    Triangle {
        points: [Vec3; 3],
        color: Vec3,
    },
    Rectangle {
        points: [Vec3; 4],
        color: Vec3,
    },
    Cube {
        points: [Vec3; 8],
        color: Vec3,
    },
    Circle {
        radius: f32,
        segments: u32,
        color: Vec3,
    },
    Polygon {
        points: Vec<Vec3>,
        color: Vec3,
    },
    Sphere{
        radius: f32,
        rings: u32, // the number of rings (parallel to the latitude)
        segments: u32, // the number of segments (parallel to the latitude)
        color: Vec3,
    }
}

impl PrimitiveShape {
    pub fn primitive_type(&self) -> PrimitiveType {
        match self {
            Self::Triangle { .. } => PrimitiveType::Triangle,
            Self::Rectangle { .. } => PrimitiveType::Rectangle,
            Self::Cube { .. } => PrimitiveType::Cube,
            Self::Circle { .. } => PrimitiveType::Circle,
            Self::Polygon { .. } => PrimitiveType::Polygon,
            Self::Sphere { .. } => PrimitiveType::Sphere,
        }
    }
}

// create mesh ////////////////////////////////////////////
pub unsafe fn create_primitive_mesh(
    renderer: &mut VulkanRenderer,
    shape: PrimitiveShape,
) -> Result<PrimitiveMesh> {
    let primitive_type = shape.primitive_type();
    let (vertices, indices) = build_primitive_mesh(shape);

    Ok(PrimitiveMesh {
        handle: MeshHandle(renderer.load_mesh_from_vertices(vertices, indices)?),
        primitive_type,
    })
}

// build mesh. create vertices and indices from points //////////////////////////////////////////////////
pub fn build_primitive_mesh(shape: PrimitiveShape) -> (Vec<Vertex>, Vec<u32>) {
    match shape {
        PrimitiveShape::Triangle { points, color } => build_triangle_mesh(points, color),
        PrimitiveShape::Rectangle { points, color } => build_rectangle_mesh(points, color),
        PrimitiveShape::Cube { points, color } => build_cube_mesh(points, color),
        PrimitiveShape::Circle {
            radius,
            segments,
            color,
        } => build_circle_mesh(radius, segments, color),
        PrimitiveShape::Polygon { points, color } => build_polygon_mesh(points, color),
        PrimitiveShape::Sphere {radius, rings,segments,color} => build_sphere_mesh(radius,rings,segments,color),
    }
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

fn build_sphere_mesh(
    radius: f32,
    rings: u32,
    segments: u32,
    color: Vec3
) -> (Vec<Vertex>, Vec<u32>) {
    let rings=rings.max(2);
    let segments=segments.max(3);

    let vertex_count=(rings+1)*(segments+1);
    let index_count=rings*segments*6;
    
    let mut vertices=Vec::with_capacity(vertex_count as usize);
    let mut indices=Vec::with_capacity(index_count as usize);

    for ring in 0..rings{
        let v=ring as f32/rings as f32;
        let theta=std::f32::consts::PI*v;

        for segment in 0..segments{
            let u=segment as f32/segments as f32;
            let phi=std::f32::consts::TAU*u;

            let x=radius*theta.sin()*phi.cos();
            let y=radius*theta.sin()*phi.sin();
            let z=radius*theta.cos();

            vertices.push(Vertex::new(
                vec3(x,y,z),
                color,
                vec2(u,v),
            ));
        }
    }

    let columns=segments+1;

    for ring in 0..rings{
        for segment in 0..segments{
            let a=ring*columns+segment;
            let b=a+columns;
            let c=b+1;
            let d=a+1;
            indices.extend_from_slice(&[
                a,b,d,
                d,b,c,
            ]);
        }
    }

    (vertices,indices)
}

// spawn primitice object //////////////////////////////////////////////
pub fn spawn_primitive_from_mesh(
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

pub unsafe fn spawn_triangle(
    world: &mut World,
    renderer: &mut VulkanRenderer,
    meshes: &mut Vec<PrimitiveMesh>,
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color :Vec3,
) -> Result<EntityId>{
    let center=(p0+p1+p2)/3.0;

    let primitive_mesh=create_primitive_mesh(
        renderer,
        PrimitiveShape::Triangle { 
            points: [
                p0-center,
                p1-center,
                p2-center,
            ], 
            color
        },
    )?;

    meshes.push(primitive_mesh);

    spawn_primitive_from_mesh(
        world,
        primitive_mesh.handle,
        Transform{
            position: center,
            ..Default::default()
        },
    )
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

pub unsafe fn update_primitive_mesh(
    renderer: &mut VulkanRenderer,
    mesh: PrimitiveMesh,
    shape: PrimitiveShape,
) -> Result<()> {
    if mesh.primitive_type != shape.primitive_type() {
        bail!(
            "Primitive shape {:?} does not match mesh type {:?}.",
            shape.primitive_type(),
            mesh.primitive_type
        );
    }

    let (vertices, indices) = build_primitive_mesh(shape);
    update_mesh(renderer, mesh.handle, vertices, indices)
}





// test ///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::vec3;

    fn white() -> Vec3 {
        vec3(1.0, 1.0, 1.0)
    }

    #[test]
    fn primitive_shape_reports_matching_type() {
        let cases = [
            (
                PrimitiveShape::Triangle {
                    points: [
                        vec3(0.0, 0.0, 0.0),
                        vec3(0.0, 1.0, 0.0),
                        vec3(0.0, 0.0, 1.0),
                    ],
                    color: white(),
                },
                PrimitiveType::Triangle,
            ),
            (
                PrimitiveShape::Rectangle {
                    points: [
                        vec3(0.0, -1.0, 1.0),
                        vec3(0.0, -1.0, -1.0),
                        vec3(0.0, 1.0, -1.0),
                        vec3(0.0, 1.0, 1.0),
                    ],
                    color: white(),
                },
                PrimitiveType::Rectangle,
            ),
            (
                PrimitiveShape::Circle {
                    radius: 1.0,
                    segments: 8,
                    color: white(),
                },
                PrimitiveType::Circle,
            ),
            (
                PrimitiveShape::Sphere { 
                    radius: 1.0, 
                    rings: 32, 
                    segments: 22, 
                    color: vec3(1.0,1.0,1.0)
                },
                PrimitiveType::Sphere,
            ),
        ];

        for (shape, primitive_type) in cases {
            assert_eq!(shape.primitive_type(), primitive_type);
        }
    }

    #[test]
    fn build_primitive_mesh_creates_expected_triangle_counts() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Triangle {
            points: [
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ],
            color: white(),
        });

        assert_eq!(vertices.len(), 3);
        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn build_primitive_mesh_creates_expected_rectangle_counts() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Rectangle {
            points: [
                vec3(0.0, -1.0, 1.0),
                vec3(0.0, -1.0, -1.0),
                vec3(0.0, 1.0, -1.0),
                vec3(0.0, 1.0, 1.0),
            ],
            color: white(),
        });

        assert_eq!(vertices.len(), 4);
        assert_eq!(indices, vec![0, 1, 2, 2, 3, 0]);
    }

    #[test]
    fn build_primitive_mesh_creates_expected_cube_counts() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Cube {
            points: [
                vec3(0.5, -0.5, 0.5),
                vec3(0.5, 0.5, 0.5),
                vec3(-0.5, 0.5, 0.5),
                vec3(-0.5, -0.5, 0.5),
                vec3(0.5, -0.5, -0.5),
                vec3(0.5, 0.5, -0.5),
                vec3(-0.5, 0.5, -0.5),
                vec3(-0.5, -0.5, -0.5),
            ],
            color: white(),
        });

        assert_eq!(vertices.len(), 24);
        assert_eq!(indices.len(), 36);
    }

    #[test]
    fn build_primitive_mesh_creates_expected_circle_counts() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Circle {
            radius: 1.0,
            segments: 8,
            color: white(),
        });

        assert_eq!(vertices.len(), 9);
        assert_eq!(indices.len(), 24);
    }

    #[test]
    fn build_primitive_mesh_clamps_circle_segments_to_three() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Circle {
            radius: 1.0,
            segments: 1,
            color: white(),
        });

        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 9);
    }

    #[test]
    fn build_primitive_mesh_creates_expected_polygon_counts() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Polygon {
            points: vec![
                vec3(0.0, -0.7, 0.7),
                vec3(0.0, -0.4, 0.5),
                vec3(0.0, 0.7, 0.5),
                vec3(0.0, 0.0, -0.6),
                vec3(0.0, -0.5, -0.4),
            ],
            color: white(),
        });

        assert_eq!(vertices.len(), 5);
        assert_eq!(indices.len(), 9);
    }

    #[test]
    fn build_primitive_mesh_returns_empty_polygon_for_too_few_points() {
        let (vertices, indices) = build_primitive_mesh(PrimitiveShape::Polygon {
            points: vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0)],
            color: white(),
        });

        assert!(vertices.is_empty());
        assert!(indices.is_empty());
    }
}
