use bevy::{ecs::system::EntityCommands, hierarchy::ChildBuild, prelude::*};

pub trait Spawn {
    fn spawn_<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn_<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        Commands::spawn(self, bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn_<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}
