// TODO
// check and rewrite comments
// write docs (relearn)
// pedantic: own arguments AFTER system arguments
// reconsider chosen numbers
    // consider if they need to be const
    // consider making some numbers const too
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
    winit::WinitWindows
};
use winit::window::Icon;
use game::JUMP_VELOCITY;
use game::TOP_BOUND;

mod menu; mod about; mod game; mod dead;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    Menu,
    About,
    Game,
    Dead
}

#[derive(Component)]
struct Ship {
    velocity: f32
}

// Common component for rocks and scoreboard
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
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        mode: bevy::window::WindowMode::Windowed,
                        title: "Flappy Space".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(
                            2.1 * TOP_BOUND,
                            2.1 * TOP_BOUND
                        ),
                        ..default()
                    }),
                    ..default()
                }),
            menu::MenuPlugin,
            about::AboutPlugin,
            game::GamePlugin,
            dead::DeadPlugin
        ))
        // Add black background
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        // Declare game state, set to default (Menu)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

// TODO NOW animate
pub fn spawn_ship(
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

fn setup(
    mut commands: Commands,
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    commands.spawn(Camera2dBundle::default());

    // Add window icon
    // As of now, Bevy does not provide a proper API for setting the window icon.
    // StackOverflow, my beloved: <https://stackoverflow.com/a/76729516>
    let Some(primary) = windows.get_window(main_window.single()) else { return };

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

fn animate_ship(
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            match sprite.index {
                5 => sprite.index = 1,
                _ => sprite.index += 1
            }
        }
    }
}

// Helper function to despawn all entities of a certain component
fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
