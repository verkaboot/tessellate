use bevy::{prelude::*, utils};
use canvas::brush::{BrushHardness, BrushSize};
use canvas::tool::ToolType;
use leafwing_input_manager::InputManagerBundle;
use ui::icon::Icon;
use ui::interaction::{trigger_on_resource_updated, trigger_watch_resource_init};
use ui::widget::color_picker::{ColorPickerWidget, HsvBoxMaterial, HueWheelMaterial};
use ui::widget::prelude::*;

use crate::input;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_on_resource_updated::<BrushSize>,
            trigger_watch_resource_init::<BrushSize>,
            trigger_on_resource_updated::<BrushHardness>,
            trigger_watch_resource_init::<BrushHardness>,
            trigger_on_resource_updated::<CurrentState<UiMode>>,
            trigger_watch_resource_init::<CurrentState<UiMode>>,
            watch_state::<UiMode>,
        ),
    );
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Default)]
enum UiMode {
    Paint,
    #[default]
    Terrain,
}

impl std::fmt::Display for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiMode::Paint => "Paint",
                UiMode::Terrain => "Terrain",
            }
        )
    }
}

impl RootState for UiMode {}

fn top_bar(root: &mut ChildBuilder) {
    root.panel(PanelDirection::Wide).with_children(|top_panel| {
        top_panel
            .button()
            .add(Icon::PaintView)
            .observe(set_root::<UiMode>(UiMode::Paint));
        top_panel
            .button()
            .add(Icon::TerrainView)
            .observe(set_root::<UiMode>(UiMode::Terrain));
        top_panel.text::<CurrentState<UiMode>>("Mode\n");
    });
}

pub fn setup(
    mut commands: Commands,
    hue_wheel_material: ResMut<Assets<HueWheelMaterial>>,
    hsv_box_material: ResMut<Assets<HsvBoxMaterial>>,
) {
    commands.insert_resource(CurrentState(UiMode::default()));
    commands.ui_root(UiMode::Paint).with_children(|root| {
        top_bar(root);
        root.flex_row().with_children(|row| {
            row.panel(PanelDirection::Tall).with_children(|side_bar| {
                side_bar
                    .button()
                    .add(Icon::Brush)
                    .observe(input::paint::set_brush(&ToolType::Paint));
                side_bar
                    .button()
                    .add(Icon::Eraser)
                    .observe(input::paint::set_brush(&ToolType::Erase));
                side_bar
                    .button()
                    .add(Icon::Layer)
                    .observe(input::paint::select_layer);
            });
            row.canvas()
                .observe(input::paint::activate_tool)
                .observe(input::paint::stop_tool);
            row.panel(PanelDirection::Tall)
                .with_children(|side_bar_right| {
                    side_bar_right
                        .button()
                        .add(Icon::ColorPicker)
                        .observe(input::paint::change_color);
                    // TODO: Make a way to not need to pass in materials as arguments
                    side_bar_right.color_picker(hue_wheel_material, hsv_box_material);
                    side_bar_right.slider::<BrushSize>("Brush Size", 1.0, 200.0);
                    side_bar_right.slider::<BrushHardness>("Brush Hardness", 0.1, 1.0);
                });
        });
        root.panel(PanelDirection::Wide);
    });

    commands
        .ui_root(UiMode::Terrain)
        .insert(InputManagerBundle::with_map(
            input::terrain::Action::input_map(),
        ))
        .with_children(|root| {
            top_bar(root);
            root.flex_row().with_children(|row| {
                row.panel(PanelDirection::Tall);
                row.canvas()
                    .observe(input::terrain::draw_terrain.map(utils::warn))
                    .observe(input::terrain::erase_terrain.map(utils::warn));
                row.panel(PanelDirection::Tall)
                    .with_children(|side_bar_right| {
                        side_bar_right
                            .inset_panel()
                            .with_children(|terrain_list_panel| {
                                terrain_list_panel.list();
                            });
                    });
            });
            root.panel(PanelDirection::Wide);
        });
}
