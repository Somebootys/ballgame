
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
pub const Player_size  : f32 = 64.0;

pub const ENEMY_NUMBER: usize = 3;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_startup_system(spawn_enemy)
        .run();
}

//system stuff
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_enemy(mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>) 
    
    {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_NUMBER {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));

    }
}

    
    


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub const PLAYER_SPEED: f32 = 500.0;
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,) {

       


        if let Ok(mut transform) = player_query.get_single_mut(){
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
    
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
            
        }

    }

    pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>,
        window_query: Query<&Window, With<PrimaryWindow>> ){
            let window = window_query.get_single().unwrap();
            if let Ok(mut player_transform) = player_query.get_single_mut(){

                let half_player_size = Player_size / 2.0;
                let x_min: f32 = 0.0 + half_player_size;
                let x_max: f32 = window.width() - half_player_size;
                let y_min: f32 = 0.0 + half_player_size;
                let y_max: f32 = window.height() - half_player_size;

                let mut translation = player_transform.translation;
                if translation.x < x_min {
                    translation.x = x_min;
                }
                if translation.x > x_max {
                    println!("width: {}", window.width()/ 2.0);
                    translation.x = x_max;
                }
                if translation.y < y_min{
                    translation.y = y_min;
                }
                if translation.y > y_max {
                    translation.y = y_max;
                }
                player_transform.translation = translation; 
            }

    }
    
   
    




