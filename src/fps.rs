use bevy::prelude::*;
use bevy_framepace;

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((bevy_framepace::FramepacePlugin, bevy_framepace::debug::DiagnosticsPlugin))
        .add_systems(Startup, setup);
    }
}


fn setup(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    settings.limiter =  bevy_framepace::Limiter::from_framerate(60.0);
}