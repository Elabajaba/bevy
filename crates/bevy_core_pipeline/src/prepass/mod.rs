//! Run a prepass before the main pass to generate depth and/or normals texture, sometimes called a thin g-buffer.
//! These textures are useful for various screen-space effects and reducing overdraw in the main pass.
//!
//! To enable the prepass, you need to add a prepass component to a [`crate::prelude::Camera3d`].
//!
//! [`DepthPrepass`]
//! [`NormalPrepass`]
//!
//! The textures are automatically added to the default mesh view bindings. You can also get the raw textures
//! by querying the [`ViewPrepassTextures`] component on any camera with a prepass component.
//!
//! When using the default mesh view bindings you should be able to use `prepass_depth()`
//! and `prepass_normal()` to load the related textures. These functions are defined in `bevy_pbr::utils`.
//! See the `shader_prepass` example that shows how to use it.
//!
//! The prepass runs for each `Material`, you can control if the prepass should run by setting the `prepass_enabled`
//! flag on the `MaterialPlugin`.
//!
//! Currently only works for 3D.

pub mod node;

use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::{
    render_phase::{CachedRenderPipelinePhaseItem, DrawFunctionId, EntityPhaseItem, PhaseItem},
    render_resource::{CachedRenderPipelineId, Extent3d, TextureFormat},
    texture::CachedTexture,
};
use bevy_utils::FloatOrd;

pub const DEPTH_PREPASS_FORMAT: TextureFormat = TextureFormat::Depth32Float;
pub const NORMAL_PREPASS_FORMAT: TextureFormat = TextureFormat::Rgb10a2Unorm;

/// If added to a [`crate::prelude::Camera3d`] then depth values will be copied to a separate texture available to the main pass.
#[derive(Component, Default, Reflect)]
pub struct DepthPrepass;

/// If added to a [`crate::prelude::Camera3d`] then vertex world normals will be copied to a separate texture available to the main pass. It will also handle normal maps.
#[derive(Component, Default, Reflect)]
pub struct NormalPrepass;

/// Textures that are written to by the prepass.
///
/// This component will only be present if any of the relevant prepass components are also present.
#[derive(Component)]
pub struct ViewPrepassTextures {
    /// The depth texture generated by the prepass.
    /// Exists only if [`DepthPrepass`] is added to the `ViewTarget`
    pub depth: Option<CachedTexture>,
    /// The normals texture generated by the prepass.
    /// Exists only if [`NormalPrepass`] is added to the `ViewTarget`
    pub normal: Option<CachedTexture>,
    /// The size of the textures.
    pub size: Extent3d,
}

/// Opaque phase of the 3D prepass.
///
/// Sorted front-to-back by the z-distance in front of the camera.
///
/// Used to render all 3D meshes with materials that have no transparency.
pub struct Opaque3dPrepass {
    pub distance: f32,
    pub entity: Entity,
    pub pipeline_id: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
}

impl PhaseItem for Opaque3dPrepass {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        radsort::sort_by_key(items, |item| item.distance);
    }
}

impl EntityPhaseItem for Opaque3dPrepass {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedRenderPipelinePhaseItem for Opaque3dPrepass {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline_id
    }
}

/// Alpha mask phase of the 3D prepass.
///
/// Sorted front-to-back by the z-distance in front of the camera.
///
/// Used to render all meshes with a material with an alpha mask.
pub struct AlphaMask3dPrepass {
    pub distance: f32,
    pub entity: Entity,
    pub pipeline_id: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
}

impl PhaseItem for AlphaMask3dPrepass {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        FloatOrd(self.distance)
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        radsort::sort_by_key(items, |item| item.distance);
    }
}

impl EntityPhaseItem for AlphaMask3dPrepass {
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedRenderPipelinePhaseItem for AlphaMask3dPrepass {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline_id
    }
}
