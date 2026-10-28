use bevy_ecs::{
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel},
    system::ScheduleSystem,
    world::World,
};

use crate::plugin::Plugin;

#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Setup;

#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Update;

pub type RunnerFn = Box<dyn FnOnce(App)>;

pub struct App {
    runner: Option<RunnerFn>,
    pub world: World,
    pub setup_schedule: Schedule,
    pub update_schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_runner(&mut self, runner: impl FnOnce(App) + 'static) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        plugin.build(self);
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn add_setup_system<M>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.setup_schedule.add_systems(systems);
        self
    }

    pub fn add_update_system<M>(
        &mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.update_schedule.add_systems(systems);
        self
    }

    pub fn run(mut self) {
        if let Some(runner) = self.runner.take() {
            runner(self);
        } else {
            panic!(
                "No runner set for the app. Please set a runner using `set_runner` before calling `run`."
            );
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            runner: None,
            world: World::default(),
            setup_schedule: Schedule::new(Setup),
            update_schedule: Schedule::new(Update),
        }
    }
}
