use super::tile::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct World {
    size: i32,
    tiles: HashMap<(i32,i32),Tile>,
}

impl World {
    /* We will probably want to make better
     * world generation at some point, but this
     * will at least get us started
     */
    pub fn generate(size: i32) -> World {
        let mut tiles: HashMap<(i32,i32),Tile> = HashMap::new();
        for j in 0..size {
            for i in 0..size {
                let coord = (i,j);
                let tile = Tile::new_rand(coord);
                if !tile.has_type(TileType::Obstruction){
                    tiles.insert(coord, tile);
                }
            }
        }
        World { size, tiles}
    }
}

#[cfg(test)]
mod tests { // TODO: Make tests better and add assertions. Also, probably will want to move these somewhere else at some point

    use super::*;

    #[test]
    fn test_world_gen() {
        println!();
        println!("Running World Generation Test");
        let world = World::generate(32);
        //TODO: reimplement test
        println!();
    }
}
