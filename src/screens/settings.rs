//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use std::any::type_name;
use bevy::{audio::Volume, prelude::*, ui::Val::*};
use bevy_persistent::{Persistent, StorageFormat};
use serde::{Deserialize, Serialize};
use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen);

    app.register_type::<GlobalVolumeLabel>();
    app.add_systems(
        Update,
        (
            update_volume_label.run_if(in_state(Screen::Settings)),
            save_volume
                .run_if(resource_changed::<GlobalVolume>.and(not(resource_added::<GlobalVolume>))),
        ),
    );
}

fn spawn_settings_screen(mut commands: Commands, global_volume: Res<GlobalVolume>) {
    commands.spawn((
        widget::ui_root("Settings Screen"),
        StateScoped(Screen::Settings),
        children![
            widget::header("Settings"),
            (
                Name::new("Settings Grid"),
                Node {
                    display: Display::Grid,
                    row_gap: Px(10.0),
                    column_gap: Px(30.0),
                    grid_template_columns: RepeatedGridTrack::px(2, 400.0),
                    ..default()
                },
                children![
                    (
                        widget::label("Audio Volume"),
                        Node {
                            justify_self: JustifySelf::End,
                            ..default()
                        }
                    ),
                    volume_widget(),
                ],
            ),
            widget::button("Back", enter_title_screen),
        ],
    ));
}

fn volume_widget() -> impl Bundle {
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_volume),
            (
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() - 0.1;
    global_volume.volume = Volume::Linear(new_factor.max(MIN_VOLUME));
}

fn raise_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let new_factor = global_volume.volume.to_linear() + 0.1;
    global_volume.volume = Volume::Linear(new_factor.min(MAX_VOLUME));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_volume_label(
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
    global_volume: Res<GlobalVolume>,
) {
    let factor = global_volume.volume.to_linear();
    let percent = (factor * 100.0).round();
    let text = format!("{}%", percent);
    label.0 = text;
}

#[derive(Serialize, Deserialize)]
struct GlobalVolumeWrapper(GlobalVolume);

fn save_volume(mut commands: Commands, global_volume: Res<GlobalVolume>) {
    let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    commands.insert_resource(
        Persistent::<GlobalVolume>::builder()
            .name("global volume")
            .format(StorageFormat::Ron)
            .path(config_dir.join("settings.toml"))
            .default(
                GlobalVolumeWrapper(GlobalVolume {
                    volume: global_volume.volume
                })
            )
            .build()
            .expect("failed to initialize global volume")
    );
}

fn enter_title_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
