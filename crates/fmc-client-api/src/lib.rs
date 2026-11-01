mod transform;

pub trait Plugin: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn update(&mut self);

    fn handle_server_data(&mut self, _data: Vec<u8>) {}

    fn set_update_frequency(&mut self) -> Option<f32> {
        None
    }
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        #[unsafe(export_name = "init-plugin")]
        pub extern "C" fn __init_plugin() {
            fmc_client_api::register_plugin(|| {
                Box::new(<$plugin_type as fmc_client_api::Plugin>::new())
            });
        }
    };
}

thread_local! {
    static PLUGIN: OnceLock<RefCell<Box<dyn Plugin>>> = OnceLock::new();
}

#[doc(hidden)]
pub fn register_plugin(build_plugin: fn() -> Box<dyn Plugin>) {
    PLUGIN.with(|p| p.set(RefCell::new(build_plugin())));
}

fn with_plugin<F, R>(f: F) -> R
where
    F: FnOnce(&mut dyn Plugin) -> R,
{
    PLUGIN.with(|p| {
        let cell = p.get().expect("plugin not initialized");
        let mut guard = cell.borrow_mut();

        f(guard.as_mut())
    })
}

mod wit {
    use std::hash::{Hash, Hasher};

    wit_bindgen::generate!({
        skip: ["init-plugin"],
        path: "./src/api.wit",
        with: {
            "fmc:api/math": crate::math,
            "fmc:api/transform": crate::transform
        },
    });

    impl Hash for Key {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let discriminant = std::mem::discriminant(self);
            core::hash::Hash::hash(&discriminant, state);
        }
    }
}

use std::{cell::RefCell, sync::OnceLock};

pub use wit::{
    Key, KeyboardKey, delta_time, get_block, get_block_state, get_camera_transform,
    get_model_transform, get_models, get_player_transform, keyboard_input, log,
    set_camera_transform, set_player_transform,
};

pub mod math {
    pub use bevy::math::*;
}

pub mod prelude {
    pub use crate::math::{DQuat, IVec3, Quat, Vec3};
    pub use crate::transform::Transform;
}

wit::export!(Component with_types_in wit);

struct Component;

impl wit::Guest for Component {
    fn set_update_frequency() -> Option<f32> {
        with_plugin(|p| p.set_update_frequency())
    }

    fn update() {
        with_plugin(|p| p.update());
    }

    fn handle_server_data(data: Vec<u8>) {
        with_plugin(|p| p.handle_server_data(data));
    }
}
