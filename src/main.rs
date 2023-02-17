mod system_command;

use bevy::prelude::*;

use self::system_command::{AddSystemCommand, RegisterSystemCommand};

fn main() {
    App::new()
        .init_resource::<Numbers>()
        .register_system_command(count_to)
        .register_system_command(write_number)
        .add_startup_systems((add_commands, apply_system_buffers, read_numbers).chain())
        .run();
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Numbers(Vec<u32>);

struct CountTo(u32);

struct WriteNumber(u32);

fn add_commands(mut commands: Commands) {
    commands.add_system_command(CountTo(3));
    commands.add_system_command(WriteNumber(100));
    commands.add_system_command(CountTo(2));
}

fn count_to(command: In<CountTo>, mut commands: Commands) {
    let CountTo(number) = command.0;
    for i in 1..=number {
        commands.add_system_command(WriteNumber(i));
    }
}

fn write_number(command: In<WriteNumber>, mut numbers: ResMut<Numbers>) {
    let WriteNumber(number) = command.0;
    numbers.push(number);
}

fn read_numbers(numbers: Res<Numbers>) {
    for number in &**numbers {
        println!("{number}");
    }
}
