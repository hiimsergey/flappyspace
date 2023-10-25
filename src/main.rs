// TODO
// add sounds    enter game    jump    clash
// remove unnecessary fonts, sprites and sfx
// predetermine window size
    // make bound dependent on them (see game.rs) 
// find a way to disable docs
    // if you succeed, write them here
// write SOURCE.txt
// rename the repo to flappyspace
// BONUS get better (jump) sounds
// BONUS write own sprites (with background)
use bevy::prelude::*;
mod menu;
mod game;
// mod dead;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    Dead
}

#[derive(Component)]
struct Ship {
    velocity: f32
}

#[derive(Component)]
struct Rock {
    velocity: f32
}

fn main() {
    App::new()
        .add_plugins((
            // Set 'Pixel Perfect' to prevent bluriness of PNGs
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            menu::MenuPlugin,
            game::GamePlugin,
            // TODO dead::DeadPlugin
        ))

        // Add black background
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))

        // Declare game state, set to default (Menu)
        .add_state::<GameState>()
        
        // Per-frame logic
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // TODO PLUS make it spin around
        // move up and down
        // rotate a bit
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: assets.load("sprites/ship.png"),
            // TODO refine these coords
            transform: Transform::from_xyz(0., 0., 1.)
                .with_scale(Vec3::splat(3.)),
            ..default()
        },
        Ship { velocity: 0. }
    ));
}

// Helper function to despawn all entities of a certain component
fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
