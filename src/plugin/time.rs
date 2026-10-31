use std::time::{Duration, Instant};

use bevy_ecs::{resource::Resource, system::ResMut};

use crate::{engine::Engine, plugin::Plugin, runner::Runner};

pub struct TimePlugin;

impl<R: Runner> Plugin<R> for TimePlugin {
    fn build(&self, engine: &mut Engine<R>) {
        let now = Instant::now();
        engine.world().insert_resource(Time {
            startup_time: now,
            current_time: now,
            delta_time: Duration::ZERO,
        });

        engine.add_setup_system(|mut time: ResMut<Time>| {
            time.update();
        });

        engine.add_update_system(|mut time: ResMut<Time>| {
            time.update();
        });
    }
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct Time {
    startup_time: Instant,
    current_time: Instant,
    delta_time: Duration,
}

impl Time {
    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        (self.current_time - self.startup_time).as_secs_f32()
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.current_time;
        self.current_time = now;
    }
}
