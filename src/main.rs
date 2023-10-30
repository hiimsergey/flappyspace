use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution};
use bevy::winit::WinitWindows;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use winit::window::Icon;

use flappyspace::{TOP_BOUND, GameState};

mod menu; mod about; mod game; mod crashed;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                // Enables 'Pixel Perfect' to prevent bluriness
                .set(ImagePlugin::default_nearest())
                
                // Window settings
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        title: "Flappy Space".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(
                            2.1 * TOP_BOUND,
                            2.1 * TOP_BOUND
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                
                // Embeds assets into executable
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
            
            // Game plugins
            menu::MenuPlugin,
            about::AboutPlugin,
            game::GamePlugin,
            crashed::CrashedPlugin
        ))

        // Adds black background
        .insert_resource(ClearColor(Color::BLACK))

        // Declares game state, set to default (Menu)
        .add_state::<GameState>()

        // Startup logic
        .add_systems(Startup, setup)
        
        .run();
}

/// First system to run
/// 
/// Spawns default 2D camera bundle and checks for window icon
fn setup(
    mut commands: Commands,
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>
) {
    // Spawns default camera setup
    commands.spawn(Camera2dBundle::default());

    // Since Bevy doesn't provide a way to add a window icon yet, I use winit
    // to do so. Thus, bevy_embedded_assets doesn't include the icon in the
    // executable. The below code checks if the icon is next to it and adds it,
    // otherwise the game has no icon. But I think that's not so tragic.
    // Snippet source: <https://stackoverflow.com/a/76729516>
    if let Ok(img) = image::open("assets/icon.png") {
        let Some(primary) = windows.get_window(main_window.single())
        else { return };

        let (icon_rgba, icon_width, icon_height) = {
            let image = img.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        primary.set_window_icon(Some(icon));
    }
}
