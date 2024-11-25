use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    EraseModifier,
}

impl Action {
    pub fn input_map() -> InputMap<Self> {
        use Action::*;
        InputMap::default().with(EraseModifier, ModifierKey::Alt)
    }
}
