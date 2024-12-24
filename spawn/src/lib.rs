use bevy::{ecs::system::EntityCommands, hierarchy::ChildBuild, prelude::*};

pub trait Spawn {
    fn custom_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn custom_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        Commands::spawn(self, bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn custom_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}
