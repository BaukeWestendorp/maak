use std::collections::VecDeque;

use bevy_ecs::{
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel},
    system::ScheduleSystem,
    world::World,
};

use crate::{plugin::Plugin, runner::Runner};

#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Setup;

#[derive(ScheduleLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Update;

pub struct Engine<R: Runner> {
    pub world: World,
    pub setup_schedule: Schedule,
    pub update_schedule: Schedule,
    plugin_queue: VecDeque<Box<dyn Plugin<R>>>,
    /// `None` after `run` is called.
    runner: Option<R>,
}

impl<R: Runner> Engine<R> {
    pub fn new(runner: R) -> Self {
        Self {
            world: World::default(),
            setup_schedule: Schedule::default(),
            update_schedule: Schedule::default(),
            plugin_queue: VecDeque::new(),
            runner: Some(runner),
        }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin<R> + 'static) {
        self.plugin_queue.push_back(Box::new(plugin));
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn setup(&mut self) {
        self.setup_schedule.run(&mut self.world);
    }

    pub fn update(&mut self) {
        self.update_schedule.run(&mut self.world);
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

    pub fn run(&mut self) {
        let runner = self
            .runner
            .take()
            .expect("Runner should only be taken once");
        runner.run(self);
    }

    pub fn build_plugins(&mut self) {
        for _ in 0..self.plugin_queue.len() {
            self.plugin_queue.pop_front().unwrap().build(self);
        }
    }
}
