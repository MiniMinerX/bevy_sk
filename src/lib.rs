use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use crate::materials::pbr::PbrPlugin;
use crate::skytex::SkyTexPlugin;

pub mod materials;
pub mod skytex;

pub struct XrUsefulSetupPlugin;

impl Plugin for XrUsefulSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.insert_resource(bevy::render::render_asset::RenderAssetBytesPerFrame::new(
                4096,
            ))
        });
    }
}

pub struct SkPlugins;

impl PluginGroup for SkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<SkPlugins>()
            .add(XrUsefulSetupPlugin)
            .add(PbrPlugin)
            .add(SkyTexPlugin)
    }
}