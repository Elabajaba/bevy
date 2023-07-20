//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_internal::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    render::{
        camera::CameraRenderGraph,
        primitives::Frustum,
        view::{ColorGrading, VisibleEntities},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // This is a Camera3dbundle, but inserted as it's individual components so we can disable them more easily
    commands.spawn((
        CameraRenderGraph::new(bevy::core_pipeline::core_3d::graph::NAME),
        Camera::default(),
        Projection::default(),
        VisibleEntities::default(),
        Frustum::default(),
        Transform::default(),
        GlobalTransform::default(),
        Camera3d::default(),
        // Commenting our the Tonemapping line stops the leak from happening for whatever reason.
        Tonemapping::None,
        DebandDither::Disabled,
        ColorGrading::default(),
    ));
}
