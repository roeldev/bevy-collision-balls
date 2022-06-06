use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct WindowTitleFpsPlugin {
    rate: f64,
}

impl WindowTitleFpsPlugin {
    pub fn with_steps_per_second(rate: f64) -> Self {
        Self { rate }
    }
}

impl Default for WindowTitleFpsPlugin {
    fn default() -> Self { Self::with_steps_per_second(2.) }
}

impl Plugin for WindowTitleFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(self.rate))
                .with_system(update),
        );
    }
}

fn update(
    mut windows: ResMut<Windows>,
    window_descriptor: Res<WindowDescriptor>,
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
        let window = windows.primary_mut();
        window.set_title(format!("{}: {}", window_descriptor.title, fps.value.to_string()));
    }
}
