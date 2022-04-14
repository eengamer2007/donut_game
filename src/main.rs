use bevy::prelude::*;

fn main() {
    //make app
    let mut app = App::new();
    //add plugins
    app.add_plugins(DefaultPlugins)
    //add startup systems
    .add_startup_system(startup)
    //add systems
    .add_system(rotator_system)
    //run app
    .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    //spawn torus
    commands.spawn_bundle(PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Torus{
                radius: 5.0,
                ring_radius: 1.5,
                subdivisions_segments: 25,
                subdivisions_sides: 25,
            })),
            material: materials.add(StandardMaterial { 
                base_color: Color::hex("ffd891").expect("god damn moron get the hex color right"),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        }
    ).insert(Rotator);
    //spawn the light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
    //spawn the camera
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 8.0))
            .looking_at(Vec3::default(), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 0.01,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_3d()
    });
}

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_y(1.5 * time.delta_seconds());
    }
}

#[derive(Component)]
struct Rotator;

#[derive(Component)]
struct Weight(f32);

#[derive(Component)]
struct Player{
    velocity: Vec3,
}