
use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){
    let atlas = TextureAtlas::from_grid(
        asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
        Vec2::splat(32.),
        11, 1, None, None
    );

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite { index: 0, ..Default::default() },
        ..Default::default()
    }, Player));
}