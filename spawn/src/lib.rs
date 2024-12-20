use bevy::{ecs::system::EntityCommands, hierarchy::ChildBuild, prelude::*};

pub trait Spawn {
    fn ui_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn ui_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn ui_spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}
