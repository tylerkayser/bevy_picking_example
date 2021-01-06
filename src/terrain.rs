use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::cube::Cube;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(terrain_setup_system.system())
            .add_system(terrain_clicking_system.system());
    }
}

fn terrain_setup_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100. })),
            material: materials.add(Color::rgb(0.58, 0.96, 0.64).into()),
            ..Default::default()
        })
        .with(PickableMesh::default());
}

fn terrain_clicking_system(
    mut query: Query<&mut Cube>,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
) {
    let intersection = pick_state.top(Group::default());

    if !mouse_button_inputs.pressed(MouseButton::Left) || intersection.is_none() {
        return;
    }

    for mut cube in query.iter_mut() {
        cube.waypoint = Some(*intersection.unwrap().1.position() * 100.);
    }
}
