use std::{
    fmt::{Debug, Formatter, Result},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::{
    ecs::{schedule::ScheduleLabel, system::Command},
    prelude::*,
};

struct SystemCommand<T: 'static + Send + Sync>(T);

impl<T: 'static + Send + Sync> Command for SystemCommand<T> {
    fn write(self, world: &mut World) {
        world.insert_resource(CommandIn(self.0));
        world.run_schedule(CommandScheduleLabel::<T>::default());
        world.remove_resource::<CommandIn<T>>();
    }
}

#[derive(Resource)]
struct CommandIn<T: 'static + Send + Sync>(T);

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

trait RegisterCommand {
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
        self.init_schedule(CommandScheduleLabel::<T>::default())
            .add_system_to_schedule(CommandScheduleLabel::<T>::default(), system)
    }
}

fn main() {
    App::new()
        .init_resource::<Numbers>()
        .register_command::<CountTo, _>(count_to)
        .register_command::<WriteNumber, _>(write_number)
        .add_startup_systems((add_commands, apply_system_buffers, read_numbers).chain())
        .run();
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Numbers(Vec<u32>);

struct CountTo(u32);

struct WriteNumber(u32);

fn add_commands(mut commands: Commands) {
    commands.add(SystemCommand(CountTo(3)));
    commands.add(SystemCommand(WriteNumber(100)));
    commands.add(SystemCommand(CountTo(2)));
}

fn count_to(command: Res<CommandIn<CountTo>>, mut commands: Commands) {
    let CountTo(number) = command.0;
    for i in 1..=number {
        commands.add(SystemCommand(WriteNumber(i)));
    }
}

fn write_number(command: Res<CommandIn<WriteNumber>>, mut numbers: ResMut<Numbers>) {
    let WriteNumber(number) = command.0;
    numbers.push(number);
}

fn read_numbers(numbers: Res<Numbers>) {
    for number in &**numbers {
        println!("{number}");
    }
}
