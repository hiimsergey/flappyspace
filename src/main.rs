// TODO
// write docs
// pedantic: own arguments AFTER system arguments
// BONUS exit app gracefully ??
// BONUS add app icon ??
// BONUS explosions ??
// TODO END END
// reconsider chosen numbers
    // consider if they need to be const
    // consider making some numbers const too
// check and rewrite comments
// remove unnecessary fonts, sprites and sfx
// predetermine window size
    // make bound dependent on them (see game.rs) 
// explain the "unclear" crashes in gameplay
use bevy::prelude::*;
use game::JUMP_VELOCITY;
mod menu;
mod game;
mod dead;

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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            // Set 'Pixel Perfect' to prevent bluriness of PNGs
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            menu::MenuPlugin,
            game::GamePlugin,
            dead::DeadPlugin
        ))

        // Add black background
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))

        // Declare game state, set to default (Menu)
        .add_state::<GameState>()
        
        // Per-frame logic
        .add_systems(Startup, setup)
        .add_systems(Update, animate_ship)
        .run();
}

// TODO NOW animate
fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>
) {
    let texture_atlas = TextureAtlas::from_grid(
        assets.load("sprites/ship.png"),
        Vec2::new(12., 10.),
        6, 1, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_xyz(0., 0., 1.)
                .with_scale(Vec3::splat(3.)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        Ship { velocity: JUMP_VELOCITY }
    ));
}

fn animate_ship(
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            match sprite.index {
                // If on last sprite, go back to first
                5 => sprite.index = 1,
                // If set to 0 (ship broken), don't animate anymore
                0 => {},
                // Otherwise just increment
                _ => sprite.index += 1
            }
        }
    }
}

// Helper function to despawn all entities of a certain component
fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
