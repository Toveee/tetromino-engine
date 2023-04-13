use bevy::{prelude::*, window::PrimaryWindow};

macro_rules! window_center_transform {
    ($window:expr) => {{
        Transform::from_xyz($window.width() / 2.0, $window.height() / 2.0, 0.0)
    }};
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(spawn_block).run();
}

const BLOCK_SIZE: f32 = 64.0;

pub fn spawn_block(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    println!("Hello. It's me.");
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            ..default()
        },
        transform: window_center_transform!(window),
        ..default()
    });
}