use bevy::ecs::Bundle;
use bevy::math::Mat4;
use bevy::render::{
    camera::{Camera, CameraProjection, DepthCalculation, VisibleEntities},
    render_graph::base::camera::CAMERA_3D,
};
use bevy::transform::components::{GlobalTransform, Transform};

#[derive(Bundle)]
pub struct CameraOrtho3dBundle {
    pub camera: Camera,
    pub orthographic_projection: NormOrthoProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for CameraOrtho3dBundle {
    fn default() -> Self {
        CameraOrtho3dBundle {
            camera: Camera {
                name: Some(CAMERA_3D.to_string()),
                ..Default::default()
            },
            orthographic_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
pub struct NormOrthoProjection {
    aspect: f32,
}

impl CameraProjection for NormOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(-self.aspect, self.aspect, -1.0, 1.0, 0.0, 1000.0)
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for NormOrthoProjection {
    fn default() -> Self {
        Self { aspect: 1.0 }
    }
}
