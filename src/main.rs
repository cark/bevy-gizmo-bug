use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_camera)
        // drawing a blue gizmo works as expected
        .add_systems(Update, regular_gizmo)
        // we trigger the DrawRedGizmo event on each frame
        .add_systems(Update, trigger_red_gizmo)
        // we try to draw a gizmo from this callback
        .observe(on_red_gizmo_triggered)
        .run();
}

#[derive(Event)]
struct DrawRedGizmo;

fn init_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn regular_gizmo(mut gizmos: Gizmos) {
    gizmos.circle_2d(Vec2::new(-50.0, 0.0), 40.0, BLUE);
}

fn trigger_red_gizmo(mut cmd: Commands) {
    cmd.trigger(DrawRedGizmo);
}

fn on_red_gizmo_triggered(_trigger: Trigger<DrawRedGizmo>, mut gizmos: Gizmos) {
    // that gizmo isn't shown.
    gizmos.circle_2d(Vec2::new(50.0, 0.0), 40.0, RED);
}
