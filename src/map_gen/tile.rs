use rand::prelude::*;

use bevy::{prelude::Vec2, utils::{HashSet, HashMap}};

#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub enum TileType {
    #[default]
    Empty,
    Worker, // TODO Add worker struct container and/or arc
    Slow,
    Tiring,
    Painful,
    Food,
    Normal,
    Obstruction,
    Player,
}

impl TileType {
    pub fn default_probs() -> Vec<f64> {
        vec![0.1, 0.005, 0.1, 0.1, 0.1, 0.05, 0.2, 0.1]
    }
    pub fn choose_rand_type(probabilities: Vec<f64>) -> TileType {
        // Check if the probabilities vector is empty
        if probabilities.is_empty() {
            panic!("Probabilities vector cannot be empty.");
        }

        // Check if the number of probabilities matches the number of TileTypes
        if probabilities.len() != TileType::Obstruction as usize + 1 {
            panic!("Number of probabilities must match the number of TileTypes.");
        }

        // Create a random number generator
        let mut rng = rand::thread_rng();

        // Generate a random number between 0 and 1
        let random_value: f64 = rng.gen();

        // Find the index of the selected TileType based on probabilities
        let mut cumulative_probability = 0.0;
        for (index, &probability) in probabilities.iter().enumerate() {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                match index {
                    0 => return TileType::Empty,
                    1 => return TileType::Worker,
                    2 => return TileType::Slow,
                    3 => return TileType::Tiring,
                    4 => return TileType::Painful,
                    5 => return TileType::Food,
                    6 => return TileType::Normal,
                    7 => return TileType::Obstruction,
                    _ => panic!("Invalid TileType enum index encountered."),
                }
            }
        }

        // Fallback to Empty if no valid TileType is found
        TileType::Empty
    }
}

/* What is the point of using hashset?
 * Contains on a hashset is quicker than
 * a Vec. I think that in most instances,
 * we are checking for the type (i.e. contains)
 * instead of actually indexing anything.
 * It might be a wrong intuition, idk...
 *
 * Also, I am using a hashset for tiletype because
 * this allows us to have overlapping types
 *
 * I ended up switching to encoding the location as a u64
 * because there was rounding error in Vec2. Please use
 * get_coords() when interacting with coordinates
 */
#[derive(Debug, Default)]
pub struct Tile {
    location: (i32,i32),
    types: HashSet<TileType>,
}

impl Tile {
    pub fn new_rand(location: (i32, i32)) -> Tile {
        let mut types: HashSet<TileType> = HashSet::new();
        let tile_type: TileType = TileType::choose_rand_type(TileType::default_probs());
        types.insert(tile_type);
        Tile { types, location }
    }

    pub fn has_type(&self, tile_type: TileType) -> bool {
        return self.types.contains(&tile_type);
    }

    pub fn set_as_player(&mut self){
        self.types.insert(TileType::Player);
    }
}
