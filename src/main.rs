// //! This example demonstrates how to use the `Camera::viewport_to_world_2d` method.

// use bevy::{color::palettes::basic::WHITE, prelude::*};

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(Update, draw_cursor)
//         .run();
// }

// fn draw_cursor(
//     camera_query: Single<(&Camera, &GlobalTransform)>,
//     windows: Query<&Window>,
//     mut gizmos: Gizmos,
// ) {
//     let (camera, camera_transform) = *camera_query;

//     let Ok(window) = windows.get_single() else {
//         return;
//     };

//     let Some(cursor_position) = window.cursor_position() else {
//         return;
//     };
//     // Calculate a world position based on the cursor's position.
//     let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
//         return;
//     };
//     gizmos.circle_2d(point, 10., WHITE);
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d);
// }
////////////////////////////////////////////////
////////////////////////////////////////////////
////////////////////////////////////////////////
////////////////////////////////////////////////
////////////////////////////////////////////////
// use bevy::{
//     image::{ImageLoaderSettings, ImageSampler},
//     prelude::*,
//     ui::widget::NodeImageMode,
//     winit::WinitSettings,
// };

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(UiScale(2.))
//         // Only run the app when there is user input. This will significantly reduce CPU/GPU use for UI-only apps.
//         .insert_resource(WinitSettings::desktop_app())
//         .add_systems(Startup, setup)
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let image =
//         asset_server.load_with_settings("castle.png", |settings: &mut ImageLoaderSettings| {
//             // Need to use nearest filtering to avoid bleeding between the slices with tiling
//             settings.sampler = ImageSampler::nearest();
//         });

//     let slicer = TextureSlicer {
//         // `numbered_slices.png` is 48 pixels square. `BorderRect::square(16.)` insets the slicing line from each edge by 16 pixels, resulting in nine slices that are each 16 pixels square.
//         border: BorderRect::square(64.),
//         // With `SliceScaleMode::Tile` the side and center slices are tiled to fill the side and center sections of the target.
//         // And with a `stretch_value` of `1.` the tiles will have the same size as the corresponding slices in the source image.
//         center_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
//         sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
//         ..default()
//     };

//     // ui camera
//     commands.spawn(Camera2d);

//     commands
//         .spawn(Node { ..default() })
//         .with_children(|parent| {
//             for [columns, rows] in [[3., 3.]] {
//                 {
//                     parent.spawn((
//                         ImageNode {
//                             image: image.clone(),
//                             image_mode: NodeImageMode::Sliced(slicer.clone()),
//                             ..default()
//                         },
//                         Node {
//                             width: Val::Px(16. * columns),
//                             height: Val::Px(16. * rows),
//                             ..default()
//                         },
//                     ));
//                 }
//             }
//         });
// }


////////////////////////////////////////////////
 


use bevy::prelude::*;
use bevy::window::WindowResolution;
use td_game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                title: "TD GAME".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}