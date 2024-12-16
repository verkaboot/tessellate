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
    fn on_filtered<E: Event>(
        &mut self,
        key: impl IntoIterator<Item = (Input, Filter)> + Copy + Eq + Send + Sync + 'static,
        effect: impl Event + Copy + Clone,
    ) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E: Event>(&mut self, effect: impl Event + Copy + Clone) -> &mut Self {
        self.obs(move |_trigger: Trigger<E>, mut commands: Commands| {
            commands.trigger(effect.clone());
        })
    }

    fn on_filtered<E: Event>(
        &mut self,
        input_filter: impl IntoIterator<Item = (Input, Filter)> + Copy + Eq + Send + Sync + 'static,
        effect: impl Event + Copy + Clone,
    ) -> &mut Self {
        self.obs(
            move |_trigger: Trigger<E>,
                  mut commands: Commands,
                  key: Res<ButtonInput<KeyCode>>,
                  mouse_button: Res<ButtonInput<MouseButton>>| {
                let cond = input_filter.into_iter().all(|input| match input {
                    (Input::Key(key_code), Filter::Pressed) => key.pressed(key_code),
                    (Input::Key(key_code), Filter::NotPressed) => !key.pressed(key_code),
                    (Input::Mouse(button), Filter::Pressed) => mouse_button.pressed(button),
                    (Input::Mouse(button), Filter::NotPressed) => !mouse_button.pressed(button),
                });
                if cond {
                    commands.trigger(effect.clone());
                }
            },
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
}

pub fn key(key_code: KeyCode) -> Input {
    Input::Key(key_code)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Pressed,
    NotPressed,
}
