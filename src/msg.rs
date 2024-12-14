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
    fn on<E: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self;
    fn on_key<E: Event>(
        &mut self,
        key: impl IntoIterator<Item = KeyCode> + Copy + Eq + Send + Sync + 'static,
        not_keys: impl IntoIterator<Item = KeyCode> + Copy + Eq + Send + Sync + 'static,
        effect: impl Event + Copy + Clone,
    ) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self {
        self.obs(move |_trigger: Trigger<E>, mut commands: Commands| {
            commands.trigger(effect.clone());
        })
    }

    fn on_key<E: Event>(
        &mut self,
        keys: impl IntoIterator<Item = KeyCode> + Copy + Eq + Send + Sync + 'static,
        not_keys: impl IntoIterator<Item = KeyCode> + Copy + Eq + Send + Sync + 'static,
        effect: impl Event + Copy + Clone,
    ) -> &mut Self {
        self.obs(
            move |_trigger: Trigger<E>,
                  mut commands: Commands,
                  inputs: Res<ButtonInput<KeyCode>>| {
                if inputs.all_pressed(keys) && !inputs.any_pressed(not_keys) {
                    commands.trigger(effect.clone());
                }
            },
        )
    }
}
