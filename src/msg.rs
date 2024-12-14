use bevy::{ecs::system::IntoObserverSystem, prelude::*, utils};

use crate::terrain;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Msg>().add_observer(update);
}

#[derive(Event, Clone, Copy)]
pub enum Msg {
    TerrainCanvasDragged,
}

pub fn update(trigger: Trigger<Msg>, mut commands: Commands) {
    match trigger.event() {
        Msg::TerrainCanvasDragged => {
            commands.run_system_cached(terrain::draw.map(utils::warn));
        }
    }
}

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
    fn on<E: Event>(&mut self, msg: Msg) -> &mut Self;
}

impl<T: CustomObserve> On for T {
    fn on<E: Event>(&mut self, msg: Msg) -> &mut Self {
        self.obs(trigger_msg::<E>(msg))
    }
}

pub fn trigger_msg<T>(msg: Msg) -> impl Fn(Trigger<T>, Commands) {
    move |_trigger: Trigger<T>, mut commands: Commands| {
        commands.trigger(msg);
    }
}
