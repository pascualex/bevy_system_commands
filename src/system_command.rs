use bevy::{ecs::system::Command, prelude::*};

struct SystemCommand<T: 'static + Send + Sync>(T);

impl<T: 'static + Send + Sync> Command for SystemCommand<T> {
    fn write(self, world: &mut World) {
        world.resource_scope(|world, mut cache: Mut<SystemCommandCache<T>>| {
            cache.0.run(self.0, world);
            cache.0.apply_buffers(world);
        });
    }
}

#[derive(Resource)]
struct SystemCommandCache<T: 'static + Send + Sync>(Box<dyn System<In = T, Out = ()>>);

pub trait RegisterSystemCommand {
    fn register_system_command<T: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T, (), P>,
    ) -> &mut Self;
}

impl RegisterSystemCommand for App {
    fn register_system_command<T: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T, (), P>,
    ) -> &mut Self {
        let mut system = IntoSystem::into_system(system);
        system.initialize(&mut self.world);
        self.insert_resource(SystemCommandCache(Box::new(system)));
        self
    }
}

pub trait AddSystemCommand {
    fn add_system_command<T: 'static + Send + Sync>(&mut self, command: T);
}

impl<'w, 's> AddSystemCommand for Commands<'w, 's> {
    fn add_system_command<T: 'static + Send + Sync>(&mut self, command: T) {
        self.add(SystemCommand(command));
    }
}
