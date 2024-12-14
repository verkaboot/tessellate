use bevy::{ecs::system::IntoObserverSystem, prelude::*};

trait CustomObserve {
    fn obs<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

impl CustomObserve for EntityCommands<'_> {
    fn obs<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        self.observe(system)
    }
}

pub trait On {
    fn on<E1: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self {
        self.obs(move |_trigger: Trigger<E>, mut commands: Commands| {
            commands.trigger(effect.clone());
        })
    }
}
