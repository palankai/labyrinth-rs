use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..NodeBundle::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            title(parent, &asset_server);
            play_button(parent, &asset_server);
            //quit_button(parent, &asset_server);
        });
}

fn title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    parent
        .spawn(NodeBundle {
            style: TITLE_NODE_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            // Text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Labyrinth",
                        make_title_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id()
}

fn play_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..ButtonBundle::default()
            },
            PlayButton {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "(P)lay",
                        make_button_text_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id()
}
