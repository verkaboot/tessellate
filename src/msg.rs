use bevy::{prelude::*, utils};

use error::Result;

use crate::terrain;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Msg>()
        .add_systems(Update, update.map(utils::warn));
}

#[derive(Event)]
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
