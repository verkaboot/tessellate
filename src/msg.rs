use bevy::{ecs::system::IntoObserverSystem, prelude::*, utils};

use error::Result;

use crate::terrain;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Msg>()
        .add_systems(Update, update.map(utils::warn));
}

#[derive(Event, Clone, Copy)]
pub enum Msg {
    TerrainCanvasDragged,
}

pub fn update(mut commands: Commands, mut msg_listener: EventReader<Msg>) -> Result<()> {
    for msg in msg_listener.read() {
        match msg {
            Msg::TerrainCanvasDragged => {
                commands.run_system_cached(terrain::draw.map(utils::warn));
            }
        }
    }

    Ok(())
}

pub fn trigger_msg<T>(msg: Msg) -> impl Fn(Trigger<T>, EventWriter<'_, Msg>) {
    move |_trigger: Trigger<T>, mut msg_writer: EventWriter<Msg>| {
        msg_writer.send(msg);
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
