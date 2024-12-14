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
    fn on_key<E: Event>(&mut self, key: KeyCode, effect: impl Event + Copy + Clone) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self {
        self.obs(
            move |_trigger: Trigger<E>,
                  mut commands: Commands,
                  inputs: Res<ButtonInput<KeyCode>>| {
                if !inputs.any_pressed([KeyCode::AltLeft, KeyCode::ControlLeft, KeyCode::ShiftLeft])
                {
                    commands.trigger(effect.clone());
                }
            },
        )
    }

    fn on_key<E: Event>(&mut self, key: KeyCode, effect: impl Event + Copy + Clone) -> &mut Self {
        self.obs(
            move |_trigger: Trigger<E>,
                  mut commands: Commands,
                  inputs: Res<ButtonInput<KeyCode>>| {
                if inputs.pressed(key) {
                    commands.trigger(effect.clone());
                }
            },
        )
    }
}
