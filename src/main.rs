use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::diagnostic::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};

const GRAVITY: f32 = 0.01;

//z is up, x and y is flat and the axis the donut is on

fn main() {
    //make app
    let mut app = App::new();
    //insert resources
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()))
    .insert_resource(WindowDescriptor{
        title: "not anymore spinny donut go brrrrr".to_string(),
        width: 750.0,
        height: 500.0,
        vsync: false,
        cursor_locked: false,
        cursor_visible: true,
        mode: WindowMode::Windowed, //in a window
        //mode: WindowMode::BorderlessFullscreen, //full screen
        ..Default::default()
    })
    //add events
    .add_event::<bevy::app::AppExit>()
    //add plugins
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    //add startup systems
    .add_startup_system(setup)
    .add_startup_system(setup_ui)
    //add systems
    .add_system(update_player)
    .add_system(update_camera)
    .add_system(update_ui)
    .add_system(spinny_donut)
    //run app
    .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle{
        ..Default::default()
    });
}

fn update_ui(
    mut context: ResMut<EguiContext>,
    diagnostic: Res<Diagnostics>,
    mut event: EventWriter<bevy::app::AppExit>,
    mut windows: ResMut<Windows>,
){
    egui::Window::new("frames").show(context.ctx_mut(), |ui| {
        ui.label(format!("frame time: {:.3}",
            match diagnostic.get(FrameTimeDiagnosticsPlugin::FRAME_TIME).unwrap().value() {
                Some(x) => x,
                None => 0.0
            }
        ));
        ui.label(format!("fps: {:.3}",
            match diagnostic.get(FrameTimeDiagnosticsPlugin::FPS).unwrap().value() {
                Some(x) => x,
                None => 0.0
            }
        ));
        ui.label(format!("frame count: {}",
            match diagnostic.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT).unwrap().value() {
                Some(x) => x,
                None => 0.0
            }
        ));
        if ui.button("exit").clicked() {
            event.send(bevy::app::AppExit);
        }
        let mut window = windows.get_primary_mut().unwrap();
        match window.mode() {
            WindowMode::Windowed => if ui.button("fullscreen").clicked() {
                window.set_mode(WindowMode::BorderlessFullscreen);
            }
            WindowMode::BorderlessFullscreen => if ui.button("windowed").clicked() {
                window.set_mode(WindowMode::Windowed);
            }
            _ => panic!("the fuck is going on with the window mode?")
        }
    });
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
        transform: Transform::from_xyz(0.0,0.0,0.0)
        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI/2.0)),
        ..Default::default()
    })
    .insert(Mass(1000.0))
    .insert(Rotate)
    .insert(Planet(5.0));

    //spawn the light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 0.0, 50.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });

    /*/spawn the player
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Icosphere{
            radius: 1.0,
            subdivisions: 1,
        })),
        material: materials.add(StandardMaterial { 
            base_color: Color::hex("ffffff").expect("god damn moron get the hex color right"),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0,10.0,1000.0),
        ..Default::default()
    })
    .insert(Player)
    .insert(Transform::from_xyz(0.0, 10.0, 50.0))
    .insert(Velocity(Vec3::default(), Vec3::default()))
    .insert(Mass(100.0));
    */
    //spawn the camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0))
            .looking_at(Vec3::default(), Vec3::Y),
        perspective_projection: PerspectiveProjection {
            ..Default::default()
        },
        ..PerspectiveCameraBundle::new_3d()
    });
}

fn update_player(
    time: Res<Time>,
    mut query: Query<(&mut Mass, &mut Velocity, &mut Transform), With<Player>>,
    query_planet: Query<(&mut Mass, &Planet), Without<Player>>,
){
    for (mass, mut velocity, mut transform) in query.iter_mut() {
        for (planet_mass, planet) in query_planet.iter() {
            let direction = calc_close_point_cirlce(transform.translation, planet.0);

            let distance: f32 = direction.distance(transform.translation);
            if direction.length() <= 1.5 {
                velocity.0 += 
                direction.normalize() * (GRAVITY * ((mass.0 * planet_mass.0)/(distance*distance)));
            }
        }
        transform.translation += velocity.0 * time.delta().as_secs_f32();
        //println!("{}", velocity.0)
    }
}

fn spinny_donut(time: Res<Time>, mut query: Query<&mut Transform, With<Rotate>>) {
    for mut i in query.iter_mut() {
        i.rotate(Quat::from_rotation_x(3.0 * time.delta().as_secs_f32()));
        i.rotate(Quat::from_rotation_z(1.0 * time.delta().as_secs_f32()));
    }
}

#[inline]
fn calc_close_point_cirlce(a: Vec3, radius: f32) -> Vec3 {
    let vec = a.truncate();
    let angle = vec.angle_between(Vec2::X);
    let res = Vec3::new(radius*angle.cos(), radius*angle.sin(), 0.0);
    //println!("{}",res);
    return res
}

fn update_camera() {

}

#[derive(Component)]
struct Rotate;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3,Vec3);

#[derive(Component)]
struct Planet(f32);