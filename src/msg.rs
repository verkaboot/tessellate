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
    fn on<E1: Event, E2: Event + Copy + Clone>(&mut self, conditions: E1, effect: E2) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E1: Event, E2: Event + Copy + Clone>(
        &mut self,
        _conditions: E1,
        effect: E2,
    ) -> &mut Self {
        self.obs(move |_trigger: Trigger<E1>, mut commands: Commands| {
            commands.trigger(effect.clone());
        })
    }
}
