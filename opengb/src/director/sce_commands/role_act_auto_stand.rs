use super::{map_role_id, SceneRoleExtensions};
use crate::director::sce_director::SceCommand;
use crate::director::sce_state::SceState;
use crate::scene::ScnScene;
use imgui::Ui;
use radiance::scene::CoreScene;

#[derive(Clone)]
pub struct SceCommandRoleActAutoStand {
    role_id: String,
    auto_play_idle: i32,
}

impl SceCommand for SceCommandRoleActAutoStand {
    fn initialize(&mut self, scene: &mut CoreScene<ScnScene>, state: &mut SceState) {}

    fn update(
        &mut self,
        scene: &mut CoreScene<ScnScene>,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        scene
            .get_role_entity(&self.role_id)
            .set_auto_play_idle(self.auto_play_idle == 1);
        true
    }
}

impl SceCommandRoleActAutoStand {
    pub fn new(role_id: i32, auto_play_idle: i32) -> Self {
        Self {
            role_id: map_role_id(role_id).to_string(),
            auto_play_idle,
        }
    }
}
