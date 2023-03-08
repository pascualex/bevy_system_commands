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
struct Numbers(Vec<i32>);

struct CountTo(i32);

struct WriteNumber(i32);

fn add_commands(mut commands: Commands) {
    commands.add_system_command(CountTo(3));
    commands.add_system_command(WriteNumber(100));
    commands.add_system_command(CountTo(2));
}

fn count_to(In(CountTo(number)): In<CountTo>, mut commands: Commands) {
    for i in 1..=number {
        commands.add_system_command(WriteNumber(i));
    }
}

fn write_number(In(WriteNumber(number)): In<WriteNumber>, mut numbers: ResMut<Numbers>) {
    numbers.push(number);
}

fn read_numbers(numbers: Res<Numbers>) {
    for number in &**numbers {
        println!("{number}");
    }
}
