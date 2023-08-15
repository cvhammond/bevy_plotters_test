use bevy::prelude::*;
mod menu;
mod pan_orbit_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .add_startup_system(pan_orbit_camera::spawn_camera)
        .add_startup_system(setup)
        .add_system(pan_orbit_camera::pan_orbit_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //    commands.spawn(Camera3dBundle {
    //        transform: Transform::from_xyz(0.0, 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    //        ..default()
    //    });
    //

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });
    let mesh = meshes.add(shape::Cube::default().into());

    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.1, 0.1, 0.0)),
        ..default()
    });

}
