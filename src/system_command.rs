use std::{
    fmt::{Debug, Formatter, Result},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::{
    ecs::{
        schedule::{ExecutorKind, ScheduleLabel},
        system::Command,
    },
    prelude::*,
};

pub struct SystemCommand<T: 'static + Send + Sync>(pub T);

impl<T: 'static + Send + Sync> Command for SystemCommand<T> {
    fn write(self, world: &mut World) {
        world.insert_resource(CommandIn(self.0));
        world.run_schedule(CommandScheduleLabel::<T>::default());
        world.remove_resource::<CommandIn<T>>();
    }
}

#[derive(Resource)]
pub struct CommandIn<T: 'static + Send + Sync>(pub T);

#[derive(ScheduleLabel)]
struct CommandScheduleLabel<T: 'static + Send + Sync> {
    phantom_data: PhantomData<T>,
}

impl<T: 'static + Send + Sync> Default for CommandScheduleLabel<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync> Clone for CommandScheduleLabel<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T: 'static + Send + Sync> PartialEq for CommandScheduleLabel<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T: 'static + Send + Sync> Eq for CommandScheduleLabel<T> {}

impl<T: 'static + Send + Sync> Hash for CommandScheduleLabel<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(0);
    }
}

impl<T: 'static + Send + Sync> Debug for CommandScheduleLabel<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        Result::Ok(())
    }
}

pub trait RegisterCommand {
    fn register_command<T: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystemConfig<P>,
    ) -> &mut Self;
}

impl RegisterCommand for App {
    fn register_command<T: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystemConfig<P>,
    ) -> &mut Self {
        let mut schedule = Schedule::new();
        schedule
            .set_executor_kind(ExecutorKind::SingleThreaded)
            .add_system(system);
        self.add_schedule(CommandScheduleLabel::<T>::default(), schedule)
    }
}
