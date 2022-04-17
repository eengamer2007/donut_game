use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::diagnostic::*;
use bevy::input::mouse::MouseMotion;

use bevy_egui::{egui, EguiContext, EguiPlugin};

const GRAVITY: f32 = 1000.0;
const SPEED: f32 = 0.1;
const ROTATE_SPEED: f32 = 0.01;

//z is up, x and y is flat and the axis the donut is on

fn main() {
    //make app
    let mut app = App::new();
    //insert resources
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()))
    //.insert_resource(Msaa{ samples: 4 })
    .insert_resource(WindowDescriptor{
        title: "not anymore spinny donut go brrrrr".to_string(),
        width: 750.0,
        height: 500.0,
        vsync: true,
        cursor_locked: false,
        cursor_visible: true,
        mode: WindowMode::Windowed, //in a window
        //mode: WindowMode::BorderlessFullscreen, //full screen
        ..Default::default()
    })
    .insert_resource(CameraRotation{x: 0.0, y: 0.0, z: 0.0})
    .insert_resource(Menu::None)
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
    .add_system(update_fps_ui)
    //.add_system(spinny_donut)
    //.add_system(print_curor_info)
    //run app
    .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle{
        ..Default::default()
    });
}

fn update_fps_ui(
    mut context: ResMut<EguiContext>,
    diagnostic: Res<Diagnostics>,
    mut event: EventWriter<bevy::app::AppExit>,
    mut windows: ResMut<Windows>,
    player: Query<&Transform, With<Player>>,
){
    egui::Window::new("")
    .anchor(egui::Align2::LEFT_TOP, (0.0,0.0))
    .auto_sized()
    .title_bar(false)
    .show(context.ctx_mut(), |ui| {
        ui.label(format!("frame time: {:.3}",
            diagnostic.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .unwrap().value().unwrap_or(f64::NAN)
        ));
        ui.label(format!("fps: {:.3}",
            diagnostic.get(FrameTimeDiagnosticsPlugin::FPS)
            .unwrap().value().unwrap_or(f64::NAN)
        ));
        ui.label(format!("frame count: {}",
            diagnostic.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            .unwrap().value().unwrap_or(f64::NAN)
        ));
        if ui.button("exit").clicked() {
            event.send(bevy::app::AppExit);
        }
        let window = windows.get_primary_mut().unwrap();
        if ui.add(
            egui::RadioButton::new(window.mode() == WindowMode::Windowed,
            "windowed")
        ).clicked() {
            window.set_mode(WindowMode::Windowed);
        }
        if ui.add(
            egui::RadioButton::new(window.mode() == WindowMode::BorderlessFullscreen,
            "borderless fullscreen")
        ).clicked() {
            window.set_mode(WindowMode::BorderlessFullscreen);
        }
        if ui.add(
            egui::RadioButton::new(window.mode() == WindowMode::Fullscreen,
            "fullscreen")
        ).clicked() {
            window.set_mode(WindowMode::Fullscreen);
        }
        if ui.add(
            egui::RadioButton::new(window.mode() == WindowMode::SizedFullscreen,
            "sized fullscreen")
        ).clicked() {
            window.set_mode(WindowMode::SizedFullscreen);
        }
        if ui.add(egui::Button::new(format!("vsync: {}", window.vsync()))).clicked() {
            window.set_vsync(!window.vsync())
        }
        for i in player.iter(){
            ui.label(format!("{}", i.translation));
        }
        for i in player.iter(){
            ui.label(format!("{}", i.rotation));
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
            radius: 10.,
            ring_radius: 3.,
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
    //.insert(SpinnyBoi)
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

    //spawn the player
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Icosphere{
            radius: 5.0,
            subdivisions: 1,
        })),
        material: materials.add(StandardMaterial { 
            base_color: Color::hex("ffffff").expect("god damn moron get the hex color right"),
            ..Default::default()
        }),
        transform: Transform::from_xyz(5.0,25.0,0.0)
        .looking_at(Vec3::default(), Vec3::Z),
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity(Vec3::default()))
    .insert(Mass(100.0));

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
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_rotation: ResMut<CameraRotation>
){
    for (mass, mut velocity, mut transform) in query.iter_mut() {
        let mut forces = Vec3::default();
        for (planet_mass, planet) in query_planet.iter() {
            let direction = calc_close_point_cirlce(transform.translation, planet.0);
            //println!("{direction}");
            let distance: f32 = direction.distance(transform.translation);
            //println!("{distance}");
            if direction.length() <= 1.5 {
                forces += 
                direction.normalize() * (GRAVITY * ((mass.0 * planet_mass.0)/(distance*distance)));
            }
        }
        //println!("{forces}");
        velocity.0 += forces/mass.0;
        if keyboard.just_pressed(KeyCode::W) {
            velocity.0 += -Vec3::Y * SPEED;
        } else if keyboard.just_released(KeyCode::W) {
            velocity.0 -= -Vec3::Y * SPEED;
        }
        if keyboard.just_pressed(KeyCode::S) {
            velocity.0 += Vec3::Y * SPEED;
        } else if keyboard.just_released(KeyCode::S) {
            velocity.0 -= Vec3::Y * SPEED;
        }
        if keyboard.just_pressed(KeyCode::A) {
            velocity.0 += Vec3::X * SPEED;
        } else if keyboard.just_released(KeyCode::A) {
            velocity.0 -= Vec3::X * SPEED;
        }
        if keyboard.just_pressed(KeyCode::D) {
            velocity.0 += -Vec3::X * SPEED;
        } else if keyboard.just_released(KeyCode::D) {
            velocity.0 -= -Vec3::X * SPEED;
        }
        if keyboard.just_pressed(KeyCode::Q) {
            velocity.0 += Vec3::Z * SPEED;
        } else if keyboard.just_released(KeyCode::Q) {
            velocity.0 -= Vec3::Z * SPEED;
        }
        if keyboard.just_pressed(KeyCode::E) {
            velocity.0 += -Vec3::Z * SPEED;
        } else if keyboard.just_released(KeyCode::E) {
            velocity.0 -= -Vec3::Z * SPEED;
        }
        for i in mouse_motion_events.iter(){
            camera_rotation.z +=  i.delta.x * ROTATE_SPEED;
            camera_rotation.x += -i.delta.y * ROTATE_SPEED;
            //transform.rotate(Quat::from_rotation_z(i.delta.x * ROTATE_SPEED));
            //transform.rotate(Quat::from_rotation_x(-i.delta.y * ROTATE_SPEED));
        }
        transform.rotation = camera_rotation.to_quat();
        transform.translation += velocity.0 * time.delta().as_millis() as f32;
        //println!("{}", transform.translation);
    }
}

fn spinny_donut(time: Res<Time>, mut query: Query<&mut Transform, With<SpinnyBoi>>) {
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

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
){
    for mut cam in camera.iter_mut() {
        for player in player.iter() {
            cam.translation = player.translation;
            cam.rotation    = player.rotation;
        }
    }
}

fn print_curor_info(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_motion_events: EventReader<MouseMotion>,
){
    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }
}

struct CameraRotation{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl CameraRotation {
    fn to_quat(&self) -> Quat {
        Quat::from_scaled_axis(Vec3::new(self.x,self.y, self.z))
    }
}

#[derive(Component)]
struct SpinnyBoi;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Planet(f32);

enum Menu{
    None,
    Pause
}