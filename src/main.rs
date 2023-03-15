use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod lib;
use lib::{Target, Wanderer, Circle, Path};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
const CAMERA_SPEED: f32 = 5.0;

fn main() {

   App::new()
   .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
   .add_plugins(DefaultPlugins.set(WindowPlugin {
       window: WindowDescriptor {
           width: SCREEN_WIDTH,
           height: SCREEN_HEIGHT,
           title: "Wanderer Algorithm".to_string(),
           resizable: true,
           ..Default::default()
       },
       ..default()
   }))
   .insert_resource(Path::new())
   .add_startup_system(spawn_camera)
   .add_startup_system(setup_scene)
   .add_system(camera_movement)
   .add_system(update_wanderer)
   .add_system(update_target)
   .add_system(update_path)
   .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }
    );
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::X;
        }
        transform.translation += direction * CAMERA_SPEED;


        if keyboard_input.pressed(KeyCode::X) {
            transform.scale *= 1.01;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            transform.scale *= 0.99;
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    const PI: f32 = std::f32::consts::PI;
    let radius: f32 = 16.0;
    let theta: f32 = PI / 2.0;

    let target_x = radius * theta.cos();
    let target_y = radius * theta.sin();

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        }
    )
    .insert(Wanderer::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        2.0,
        0.1,
        Vec2::new(target_x, target_y),
    ));

    // visual radius
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(radius))).into(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.2))),
        ..default()
    }).insert(Circle);



    // visual target
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(radius/10.0))).into(),
        transform: Transform {
            translation: Vec3::new(target_x, target_y, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::RED)),
        ..default()
    }).insert(Target::new(
        Vec2::new(target_x, target_y), 
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),  
        2.0,
        0.1,
        radius,
    ));

}

fn update_wanderer(
    mut query: Query<(&mut Wanderer, &mut Transform)>,
    mut path: ResMut<Path>
) {
    for (mut wanderer, mut transform) in query.iter_mut() {

        let target = wanderer.clone().target;
        wanderer.movement();
        wanderer.seek(target);

        path.add_point(wanderer.pos);
        transform.translation = Vec3::new(wanderer.pos.x, wanderer.pos.y, 0.0);
    }
}

fn update_target (
    mut target_query: Query<(&mut Target, &mut Transform), Without<Circle>>,
    mut wanderer_query: Query<&mut Wanderer>,
    mut circle_query: Query<(&mut Circle, &mut Transform), Without<Target>>,
) {

    for (mut target, mut transform) in target_query.iter_mut() {

        for mut wanderer in wanderer_query.iter_mut() {
            target.movement();
            target.shift_theta(&mut wanderer);
            wanderer.set_target(target.pos);
            transform.translation = Vec3::new(target.pos.x, target.pos.y, 0.0);

            for (mut _circle, mut circle_transform) in circle_query.iter_mut() {
                circle_transform.translation = Vec3::new(wanderer.pos.x, wanderer.pos.y, 0.0);
            }
        }

    }
}

fn update_path (
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    path: Res<Path>,
) {


    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(5.0))).into(),
        transform: Transform {
            translation: Vec3::new(path.points.last().unwrap().x, path.points.last().unwrap().y, 0.0),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 1.0, 0.2))),
        ..default()
    });
}