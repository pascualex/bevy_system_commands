mod system_command;

use bevy::prelude::*;

use self::system_command::{CommandIn, RegisterCommand, SystemCommand};

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
