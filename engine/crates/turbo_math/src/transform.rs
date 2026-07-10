use cgmath::{Deg, Matrix4, Vector3, vec3};
pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles
    pub scale: Vec3,
}

impl Transform {
    // create Mat4
    pub fn matrix(&self) -> Mat4 {
        Matrix4::from_translation(self.position)
            * Matrix4::from_angle_x(Deg(self.rotation.x))
            * Matrix4::from_angle_y(Deg(self.rotation.y))
            * Matrix4::from_angle_z(Deg(self.rotation.z))
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation += delta;
    }

    pub fn scale(&mut self, delta: Vec3) {
        self.scale += delta;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}

// test /////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::Vector4;

    fn assert_vec3_near(actual: Vec3, expected: Vec3) {
        let epsilon = 0.0001;

        assert!(
            (actual.x - expected.x).abs() < epsilon,
            "x: expected {}, got {}",
            expected.x,
            actual.x
        );
        assert!(
            (actual.y - expected.y).abs() < epsilon,
            "y: expected {}, got {}",
            expected.y,
            actual.y
        );
        assert!(
            (actual.z - expected.z).abs() < epsilon,
            "z: expected {}, got {}",
            expected.z,
            actual.z
        );
    }

    #[test]
    fn default_transform_is_values() {
        let t = Transform::default();

        assert_eq!(t.position, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.rotation, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.scale, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn translate_transform() {
        let mut t = Transform::default();

        t.translate(vec3(1.0, 2.0, 3.0));

        assert_eq!(t.position, vec3(1.0, 2.0, 3.0));
        assert_eq!(t.rotation, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.scale, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn rotate_transform() {
        let mut t = Transform::default();

        t.rotate(vec3(10.0, 20.0, 30.0));

        assert_eq!(t.position, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.rotation, vec3(10.0, 20.0, 30.0));
        assert_eq!(t.scale, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn scale_transform() {
        let mut t = Transform::default();

        t.scale(vec3(1.0, 2.0, 3.0));

        assert_eq!(t.position, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.rotation, vec3(0.0, 0.0, 0.0));
        assert_eq!(t.scale, vec3(2.0, 3.0, 4.0));
    }

    #[test]
    fn matrix_applies_scale_rotation_and_translation() {
        let t = Transform {
            position: vec3(1.0, 2.0, 3.0),
            rotation: vec3(0.0, 0.0, 90.0),
            scale: vec3(2.0, 3.0, 4.0),
        };

        let transformed = t.matrix() * Vector4::new(1.0, 0.0, 0.0, 1.0);

        assert_vec3_near(
            vec3(transformed.x, transformed.y, transformed.z),
            vec3(1.0, 4.0, 3.0),
        );
        assert!((transformed.w - 1.0).abs() < 0.0001);
    }
}