use bevy::prelude::*;
use flappyspace::*;

/// Custom game plugin for all things on the menu screen
pub struct MenuPlugin;

/// Tag component for entites added on the menu screen
#[derive(Component)]
struct OnMenuScreen;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), (spawn_menu_text, spawn_ship))
            .add_systems(
                Update,
                (animate_ship, lobby_input, rotate_text)
                    .run_if(in_state(GameState::Menu))
            )
            .add_systems(OnExit(GameState::Menu), cleanup::<OnMenuScreen>);
    }
}

/// Spawns the game title and keyboard input hints
fn spawn_menu_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "Flappy Space",
            HEADING_FONT_SIZE,
            Color::WHITE,
            HEADING_Y
        ), OnMenuScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press Enter to start",
            INPUT_HINT_FONT_SIZE,
            Color::WHITE,
            INPUT_HINT_UPPER_Y
        ), OnMenuScreen, TextRotation
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press A for About",
            INPUT_HINT_FONT_SIZE,
            Color::WHITE,
            INPUT_HINT_LOWER_Y
        ), OnMenuScreen, TextRotation
    ));
}

/// Spawns the only Ship entity in the game at the center of the screen
/// 
/// The ship will be shown in the menu screen and in the gameplay.
fn spawn_ship(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>
) {
    // Loads the sprite sheet
    let texture_atlas = TextureAtlas::from_grid(
        assets.load("sprites/ship.png"),
        Vec2::new(12., 10.),
        6, 1, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawns the entity
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            // Sets the spawn coordinates
            // z = 1.1 lets the ship appear before the rocks and score
            transform: Transform::from_xyz(0., 0., 1.1)
                .with_scale(Vec3::splat(3.)),
            ..default()
        },
        ShipAnimationTimer(Timer::from_seconds(0.4, TimerMode::Repeating)),
        Ship { velocity: JUMP_VELOCITY }
    ));
}
