use bevy::prelude::*;
use std::{ops::Range, time::Duration};

#[derive(PartialEq, Component, Clone, Debug, Reflect)]
#[reflect(Component)]
/// Playback settings which adjust the playback of a vello asset.
///
/// You can add this component directly to a `VelloAssetBundle` entity to adjust playback settings.
pub struct PlaybackSettings {
    /// Whether to automatically start the animation.
    pub autoplay: bool,
    /// The direction of the animation.
    pub direction: PlaybackDirection,
    /// The speed of the animation as a multiplier. 1.0 is normal speed. Anything less than 1 is slower, and anything greater than 1 is faster.
    pub speed: f32,
    /// A duration of time spent idle between loops.
    pub intermission: Duration,
    /// Whether to loop, and how many.
    pub looping: PlaybackLoopBehavior,
    /// The segments (frames) of the animation to play. Values out of range will be ignored.
    pub segments: Range<f32>,
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            autoplay: true,
            direction: PlaybackDirection::default(),
            speed: 1.0,
            intermission: Duration::ZERO,
            looping: PlaybackLoopBehavior::default(),
            segments: f32::MIN..f32::MAX,
        }
    }
}

/// The direction to play the segments of a lottie animation.
#[derive(PartialEq, Component, Default, Clone, Copy, Debug, Reflect)]
pub enum PlaybackDirection {
    /// Play in the default direction, first frame to last frame.
    #[default]
    Normal = 1,
    /// Play in the reverse direction, last frame to first frame.
    Reverse = -1,
}

/// How often to loop.
#[derive(PartialEq, Component, Default, Clone, Copy, Debug, Reflect)]
pub enum PlaybackLoopBehavior {
    /// Do not loop. This is equivalent to `PlaybackLoopBehavior::Amount(0)`.
    DoNotLoop,
    /// Complete a specified number of loops.
    Amount(usize),
    /// Loop continuously.
    #[default]
    Loop,
}