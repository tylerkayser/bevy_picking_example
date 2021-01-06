use bevy::prelude::*;

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(cube_setup_system.system())
            .add_system(cube_waypoint_system.system());
    }
}

pub struct Cube {
    speed: f32,
    pub waypoint: Option<Vec3>,
}

fn cube_setup_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Cube {
            speed: 10.0,
            waypoint: None,
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
                ..Default::default()
            });
        });
}

fn cube_waypoint_system(mut query: Query<(&mut Cube, &mut Transform)>, time: Res<Time>) {
    for (mut cube, mut transform) in query.iter_mut() {
        if let Some(waypoint) = cube.waypoint {
            let direction = Vec3 {
                y: 0.,
                ..(waypoint - transform.translation)
            }
            .normalize();
            let distance = Vec3 {
                y: 0.,
                ..transform.translation
            }
            .distance(waypoint);

            if distance < cube.speed * time.delta_seconds() {
                transform.translation = waypoint;
                cube.waypoint = None;
            } else {
                transform.translation += direction * time.delta_seconds() * cube.speed;
            }
        }
    }
}
