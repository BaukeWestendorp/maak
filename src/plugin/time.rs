use std::time::{Duration, Instant};

use bevy_ecs::{resource::Resource, system::ResMut};

use crate::{app::App, plugin::Plugin};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        let now = Instant::now();
        app.world().insert_resource(Time {
            app_startup_time: now,
            current_time: now,
            delta_time: Duration::ZERO,
        });

        app.add_setup_system(|mut time: ResMut<Time>| {
            time.update();
        });

        app.add_update_system(|mut time: ResMut<Time>| {
            time.update();
        });
    }
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct Time {
    app_startup_time: Instant,
    current_time: Instant,
    delta_time: Duration,
}

impl Time {
    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        (self.current_time - self.app_startup_time).as_secs_f32()
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.current_time;
        self.current_time = now;
    }
}
