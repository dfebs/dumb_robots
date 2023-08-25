
use super::tile::*;

#[derive(Debug, Default)]
pub struct World {
    size: usize,
    tiles: Vec<Tile>,
    // TODO add workers here
}

impl World {
    /* We will probably want to make better
     * world generation at some point, but this
     * will at least get us started
     */
    pub fn generate(size: usize) -> World {
        let mut tiles: Vec<Tile> = Vec::new();
        for j in 0..size {
            for i in 0..size {
                tiles.push(Tile::new_rand((i, j), size));
            }
        }
        World { size, tiles }
    }
    pub fn has_tile(&self, x: f32, y: f32) {}
    pub fn print(&self) {
        let mut col_counter = 0;
        let mut to_print = String::new();
        for tile in &self.tiles {
            if col_counter % self.size == 0 {
                to_print.push('\n');
            }
            if tile.has_type(TileType::Obstruction) {
                to_print.push('#');
            } else if tile.has_type(TileType::Player) {
                to_print.push('0');
            } else if tile.has_type(TileType::Food) {
                to_print.push('F');
            } else if tile.has_type(TileType::Worker) {
                to_print.push('W');
            } else {
                to_print.push('.');
            }
            col_counter += 1;
        }
        println!("{}", to_print);
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
        world.print();
        println!();
    }
}
