use std::ops::Range;
use bevy::prelude::Color;

/// Upper spacial bound of the ship
pub const TOP_BOUND: f32 = 425.;

/// Lower spacial bound of the ship
pub const BOTTOM_BOUND: f32 = -TOP_BOUND;

/// Number that the ship's velocity is being decreased by every frame
pub const GRAVITY: f32 = 40.;

/// Number that the ship's velocity gets set to after a jump
pub const JUMP_VELOCITY: f32 = 500.;



/// X-coordinate at which rocks despawn
pub const ROCK_DESPAWN_X: f32 = -500.;

/// Range of possible x-coordinates new rocks spawn at
pub const ROCK_SPAWN_X_RANGE: Range<u16> =
    -ROCK_DESPAWN_X as u16..(-ROCK_DESPAWN_X + 150.) as u16;

/// Range of possible values of y-distances between rocks in a single wave
pub const ROCK_DISTANCE_RANGE: Range<u8> = 128..255;

/// Range of possible scaling values of rocks
/// 
/// *diameter = height/width in pixels \* scale*
pub const ROCK_SIZE_RANGE: Range<u8> = 2..6;

/// Range of possible numbers of quarter seconds between new rock waves
pub const ROCK_SPAWN_RATE: Range<u8> = 3..5;

/// Constant velocity of every rock
pub const ROCK_VELOCITY: f32 = 200.;



/// Font size of heading text
pub const HEADING_FONT_SIZE: f32 = 100.;

/// Y-coordinate of heading text
pub const HEADING_Y: f32 = 200.;

/// Font size of input hints ("Press <key> to <action>")
pub const INPUT_HINT_FONT_SIZE: f32 = 50.;

/// Y-coordinate of input hint ("Press <key> to <action>"), if there is one
/// input hint
pub const INPUT_HINT_ONE_Y: f32 = -220.;

/// Y-coordinate of upper input hint ("Press <key> to <action>"), if there are
/// multiple input hints
pub const INPUT_HINT_UPPER_Y: f32 = -170.;

/// Y-coordinate of lower input hint ("Press <key> to <action>"), if there are
/// multiple input hints
pub const INPUT_HINT_LOWER_Y: f32 = -270.;



/// Font size of content in About screen
pub const ABOUT_TEXT_FONT_SIZE: f32 = 30.;

/// Color text spawned in About screen
pub const ABOUT_TEXT_COLOR: Color = Color::YELLOW;
