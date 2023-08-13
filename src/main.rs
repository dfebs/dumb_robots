#[cfg(test)]
mod tests;

mod map_gen;
use crate::map_gen::World;

use bevy::prelude::*;

fn main() {
    App::new().run();
}
