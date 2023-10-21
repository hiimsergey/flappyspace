// TODO PLAN
// maybe learn about states so that you have good starting conditions
    // fish is in middle, stones dont spawn
    // maybe there is still some animation, but without stones
// add states: gameover, menu
// start off with fish floating in middle
// TODO UPDATES
// add random texture:    fish every new game    stone in-game
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.6, 0.8);
const UPPER_BOUND: f32 = 400.;
const BOTTOM_BOUND: f32 = -UPPER_BOUND;
const GRAVITY: f32 = 40.;
const JUMP_VELOCITY: f32 = 800.;

#[derive(Component)]
struct Fish {
    velocity: f32
}

#[derive(Component)]
struct Stone;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            bevy::window::close_on_esc,
            update_fish
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(
                shape::Circle::default().into()
            ).into(),
            material: materials.add(ColorMaterial::from(
                Color::rgb(3., 3., 0.5)
            )),
            transform: Transform::from_translation(Vec3::new(0., -50., 1.))
                .with_scale(Vec3::new(30., 30., 0.)),
            ..default()
        },
        Fish { velocity: 0. },
    ));
}

// TODO
// get some texture
    // make it dynamic (rotate it)
    // one special texture for leap and dive
// Update the position of the fish: jumping, falling, textures
fn update_fish(
    key: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Fish)>,
    time: Res<Time>
) {
    for (mut transform, mut fish) in query.iter_mut() {
        // Alter position depending on velocity and time
        transform.translation.y += fish.velocity * time.delta_seconds();
        fish.velocity -= GRAVITY;

        // Alter velocity based on keyboard events
        if key.just_pressed(KeyCode::Space) {
            fish.velocity = JUMP_VELOCITY;
        }
        if key.just_pressed(KeyCode::L) {
            fish.velocity = JUMP_VELOCITY * 2.;
        }
        if key.just_pressed(KeyCode::D) {
            fish.velocity = -JUMP_VELOCITY;
        }

        // Add upper space bound
        if transform.translation.y < BOTTOM_BOUND {
            transform.translation.y = BOTTOM_BOUND;
        }

        // Add lower space bound
        if transform.translation.y > UPPER_BOUND {
            transform.translation.y = UPPER_BOUND;
        }
    }
}
