use bevy::prelude::*;

const FRAME_DURATION: f32 = 0.15;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite_frames);
}

#[derive(Component)]
pub struct AnimationFrameQueue {
    frames: &'static [usize],
    timer: Timer,
    current_index: usize,
}

impl AnimationFrameQueue {
    pub fn new(frames: &'static [usize]) -> Self {
        Self {
            frames,
            timer: Timer::from_seconds(FRAME_DURATION, TimerMode::Repeating),
            current_index: 0,
        }
    }

    pub fn tick_and_advance(&mut self, time: &Time, sprite: &mut TextureAtlas) {
        self.timer.tick(time.delta());

        if self.timer.just_finished() {
            self.current_index = (self.current_index + 1) % self.frames.len();
            sprite.index = self.frames[self.current_index];
        }
    }
}

fn animate_sprite_frames(
    time: Res<Time>,
    mut query: Query<(&mut AnimationFrameQueue, &mut Sprite)>,
) {
    for (mut queue, mut sprite) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            queue.tick_and_advance(&time, atlas);
        } else {
            continue;
        }
    }
}
