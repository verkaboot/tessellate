use bevy::{ecs::system::IntoObserverSystem, prelude::*, utils};

use error::Result;
use input::trigger::OnDrag;

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
                println!("B");
                commands.run_system_cached(terrain::draw.map(utils::warn));
            }
        }
    }

    Ok(())
}

pub fn on<T>(msg: Msg) -> impl Fn(Trigger<T>, EventWriter<'_, Msg>) {
    move |_trigger: Trigger<T>, mut msg_writer: EventWriter<Msg>| {
        println!("A");
        msg_writer.send(msg);
    }
}

trait Observe {
    fn obs<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self;
}

impl Observe for EntityCommands<'_> {
    fn obs<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> &mut Self {
        self.observe(system)
    }
}

pub trait On {
    fn x<E: Event>(&mut self, msg: Msg) -> &mut Self;
}

impl<T: Observe> On for T {
    fn x<E: Event>(&mut self, msg: Msg) -> &mut Self {
        self.obs(on::<E>(msg))
    }
}
