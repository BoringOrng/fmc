// The 'fmc_mod_api' module is the core of the game. It takes care of chunk management,
// networking, physics, etc... Things that are specific to the game are found in
// the 'fmc_vanilla' module, mobs, items and the like.
use fmc_mod_api::{
    networking::Server,
    particle_effects::ParticleEffects,
    players::{Camera, Player},
    prelude::*,
    protocol::messages,
};

// Your mod must expose a struct named `Mod` that implements `bevy::app::Plugin` to be recognized as a mod.
pub struct Mod;
impl Plugin for Mod {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, barf);
    }
}

// Spawns a particle effect in front of all players
fn barf(
    mut server: ResMut<Server>,
    players: Query<(Entity, &Transform, &Camera), With<Player>>,
    particle_effects: Res<ParticleEffects>,
) {
    for (player_entity, transform, camera) in players.iter() {
        let camera_position = transform.translation + camera.translation;
        let forward = camera.forward();

        let position = camera_position + forward;

        let explosion_id = particle_effects
            .get_id("explosion")
            .expect("explosion particle effect should be registered");

        server.send_one(
            player_entity,
            messages::ParticleEffect {
                id: explosion_id,
                position,
                rotation: Quat::IDENTITY,
                texture: String::from("blocks/grass_top.png"),
                color: Vec4::ONE,
            },
        );
    }
}

// This provides a convenient way to test the mod. The content is in lib.rs
fn main() {
    // While developing we want all artifacts to go to a separate directory.
    if std::env::var_os("CARGO").is_some() {
        std::fs::create_dir("server").ok();
        std::env::set_current_dir("server").unwrap();
    }

    App::new()
        // This is a mod for fmc_vanilla, replace fmc_vanilla with the game you want to mod in Cargo.toml
        // Use the games docs to discover functionality you can modify.
        .add_plugins(fmc_vanilla::DefaultPlugins)
        .add_plugins(Mod)
        .run();
}
