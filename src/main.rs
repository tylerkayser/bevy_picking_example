use bevy::prelude::*;
use bevy_mod_picking::*;

mod camera;
mod cube;
mod terrain;

fn setup(commands: &mut Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(camera::CameraOrtho3dBundle {
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Transform::from_translation(Vec3::new(0.0, 100.0, 100.0))
                    .looking_at(Vec3::default(), Vec3::unit_y())
            },
            ..Default::default()
        })
        .with(PickSource::default());
}

fn main() {
    use bevy::render::camera::camera_system;

    App::build()
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(cube::CubePlugin)
        .add_plugin(terrain::TerrainPlugin)
        // .add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .add_system_to_stage(
            bevy::app::stage::POST_UPDATE,
            camera_system::<camera::NormOrthoProjection>.system(),
        )
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
