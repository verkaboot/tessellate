pub mod interaction;
pub mod trigger;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((trigger::plugin, interaction::plugin))
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, setup);
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    PanCamera,
}

fn setup(mut commands: Commands) {
    use Action::*;

    let input_map = InputMap::new([(PanCamera, KeyCode::Space)]);
    commands.spawn(InputManagerBundle::with_map(input_map));
}
