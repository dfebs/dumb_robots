use super::tile::*;

#[derive(Debug, Default)]
pub struct World {
    size: usize,
    tiles: Vec<Tile>,
    open_tile_count: usize,
    // TODO add workers here
}


#[derive(Debug, Default)]
pub enum Cardinal {
    U,
    R,
    D,
    L,
}

impl World {
    /* We will probably want to make better
     * world generation at some point, but this
     * will at least get us started
     */
    pub fn generate(size: usize) -> World {
        let mut open_tile_count: usize = 0;
        let mut tiles: Vec<Tile> = Vec::new();
        for j in 0..size {
            for i in 0..size {
                let tile = Tile::new_rand((i, j), size);
                if !tile.has_type(TileType::Obstruction){
                    open_tile_count += 1;
                }
                tiles.push(tile);
            }
        }
        World { size, tiles, open_tile_count}
    }
    pub fn get_directions_from(&self, x:i32, y:i32){
        let count = self.open_tile_count;
        if let Some(mut tile) = self.get_open_tile(x, y){
            let coord = encode_coord(x, y);
            tile.distance_map.insert(coord, count);
            for (cardinal, tile) in self.get_open_neighbors(x, y){
                let (dx, dy) = tile.get_coords();
                self.follow_direction(count, dx, dy, cardinal);

            }
        }
    }
    pub fn follow_direction(&self, mut starting_count: usize, x:i32, y:i32, cardinal: Cardinal) -> Vec<(&mut Tile, Vec<&mut Tile>)> {
        let mut dx = x; 
        let mut dy = y;
        let sx: i32;
        let sy: i32;
        match cardinal {
            U => {sx =  0; sy =  1},
            R => {sx =  1; sy =  0},
            D => {sx =  0; sy = -1},
            L => {sx = -1; sy =  0},
        }
        let mut tile_list: Vec<(&mut Tile, Vec<(Cardinal, &mut Tile)>)> = Vec::new();
        
        loop {
            dx += sx;
            dy += sy;

            if let Some(mut tile) = self.get_open_tile(dx, dy){
                starting_count -= 1;
                let neighbors = self.get_open_neighbors(dx, dy);
                let coord = encode_coord(dx, dy);
                tile.distance_map.insert(coord, starting_count);
                tile_list.push((&mut tile, neighbors));
            } else {
                break;
            }
        }

        return tile_list;
    }
    pub fn get_open_neighbors(&self, x:i32, y:i32) -> Vec<(Cardinal,&mut Tile)> {
        let mut neighbors: Vec<&mut Tile> = Vec::new();

        if let Some(mut tile) = self.get_open_tile(x, y + 1){
            neighbors.push((Cardinal::U,&mut tile));
        }
        if let Some(mut tile) = self.get_open_tile(x + 1, y){
            neighbors.push((Cardinal::R,&mut tile));
        }
        if let Some(mut tile) = self.get_open_tile(x, y - 1){
            neighbors.push((Cardinal::D,&mut tile));
        }
        if let Some(mut tile) = self.get_open_tile(x - 1, y){
            neighbors.push((Cardinal::L,&mut tile));
        }
        
        return neighbors;
    }

    pub fn get_open_tile(&self, x: i32, y: i32) -> Option<&mut Tile>{
        let pos = self.get_vec_pos(x, y);
        if let Some(mut tile) = self.tiles.get(pos){
            if tile.has_type(TileType::Obstruction) {
                None
            } else {
                // TODO add arc mutex crap :( maybe just arc??
                Some(&mut tile)
            }
        } else {
            None
        }
    }
    pub fn get_vec_pos(&self, x: i32, y: i32) -> usize { 
        let pos = (y*self.size as i32) + x;
        assert!(pos>=0,"World tiles vector tried calling to negative index");
        let pos = pos as usize;
        return pos;
    }
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
