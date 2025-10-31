use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

fn main() {
    App::new()
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .run();
}
