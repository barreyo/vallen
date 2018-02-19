use fly_cam::FlyCamSystem;

use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};

pub struct CameraBundle;

impl Default for CameraBundle {
    fn default() -> Self {
        CameraBundle {}
    }
}

impl<'a, 'b> ECSBundle<'a, 'b> for CameraBundle {
    fn build(
        self,
        _world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        Ok(builder.add(FlyCamSystem::default(), "fly_cam_system", &["input_system"]))
    }
}
