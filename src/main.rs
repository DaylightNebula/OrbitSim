use bevy::prelude::*;
use orbit::*;

pub mod orbit;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OrbitPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // all mass and radius are divided by 10 ^ 6

    // earth
    commands.spawn((
            PbrBundle {
            // mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                radius: 6.371,
                subdivisions: 7
            }).unwrap()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Orbit {
            mass: 5.9722 * (10.0_f32.powf(18.0)),
            acceleration: Vec3::ZERO,
            velocity: Vec3::ZERO
        }
    ));

    // moon
    commands.spawn((
            PbrBundle {
            // mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                radius: 3.080, // should be 1.080, larger so it can be seen lol
                subdivisions: 7
            }).unwrap()),
            material: materials.add(Color::rgb(0.7, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 384.4 }),
            ..default()
        },
        Orbit {
            mass: 7.34767309 * (10.0_f32.powf(16.0)),
            acceleration: Vec3::ZERO,
            velocity: Vec3::ZERO
        }
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { color: Color::WHITE, illuminance: 10000.0, ..Default::default() },
        ..Default::default()
    });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 800.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}