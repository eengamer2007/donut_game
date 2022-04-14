use bevy::prelude::*;

fn main() {
    //make app
    let mut app = App::new();
    //insert resources
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()))
    //add plugins
    .add_plugins(DefaultPlugins)
    //add startup systems
    .add_startup_system(setup)
    //add systems
    .add_system(update_player)
    //run app
    .run();
}

fn setup(
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
            base_color: Color::hex("ffffff").expect("god damn moron get the hex color right"),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        ..Default::default()
    })
    .insert(Mass(1000.0))
    .insert(Planet);
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
    commands.spawn()
    .insert(Player)
    .insert(Transform::default())
    .insert(Velocity(Vec3::default()))
    .insert(Mass(100.0));
    //.with_children(|parent| {
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0))
                .looking_at(Vec3::default(), Vec3::Y),
            perspective_projection: PerspectiveProjection {
                ..Default::default()
            },
            ..PerspectiveCameraBundle::new_3d()
        });
    //});
}

fn update_player(
    time: Res<Time>,
    mut query: Query<(&mut Mass, &mut Velocity, &mut Transform), With<Player>>,
){
    for (_mass, velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta().as_secs_f32();

    }
}

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Planet;