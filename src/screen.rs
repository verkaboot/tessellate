use bevy::prelude::*;
use canvas::brush::{BrushHardness, BrushSize};
use canvas::tool::ToolType;
use input::trigger::{trigger_on_resource_updated, trigger_watch_resource_init};
use ui::icon::Icon;
use ui::widget::color_picker::{ColorPickerWidget, HsvBoxMaterial, HueWheelMaterial};
use ui::widget::prelude::*;

use crate::event;

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
            .queue(Icon::PaintView)
            .observe(set_root::<UiMode>(UiMode::Paint));
        top_panel
            .button()
            .queue(Icon::TerrainView)
            .observe(set_root::<UiMode>(UiMode::Terrain));
        top_panel.text::<CurrentState<UiMode>>();
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
                    .queue(Icon::Brush)
                    .observe(event::button::set_brush(ToolType::Paint));
                side_bar
                    .button()
                    .queue(Icon::Eraser)
                    .observe(event::button::set_brush(ToolType::Erase));
                side_bar
                    .button()
                    .queue(Icon::Layer)
                    .observe(event::button::select_layer);
            });
            row.canvas()
                .observe(event::paint::activate_tool)
                .observe(event::paint::stop_tool)
                .observe(event::camera::pan)
                .observe(event::camera::zoom);
            row.panel(PanelDirection::Tall)
                .with_children(|side_bar_right| {
                    side_bar_right
                        .button()
                        .queue(Icon::ColorPicker)
                        .observe(event::button::change_color);
                    // TODO: Make a way to not need to pass in materials as arguments
                    side_bar_right.color_picker(hue_wheel_material, hsv_box_material);
                    side_bar_right.slider::<BrushSize>("Brush Size", 1.0, 200.0);
                    side_bar_right.slider::<BrushHardness>("Brush Hardness", 0.1, 1.0);
                });
        });
        root.panel(PanelDirection::Wide);
    });

    commands.ui_root(UiMode::Terrain).with_children(|root| {
        top_bar(root);
        root.flex_row().with_children(|row| {
            row.panel(PanelDirection::Tall);
            row.canvas()
                .observe(event::terrain::draw)
                .observe(event::terrain::erase)
                .observe(event::camera::pan)
                .observe(event::camera::zoom);

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
