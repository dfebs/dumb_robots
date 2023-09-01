use super::{tile::Tile, cardinal::Cardinal};
use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;

pub fn generate_tile(size: i32) -> HashSet<(i32,i32)> {
    let mut tiles: HashSet<(i32,i32)> = HashSet::new();
    let mut rng = rand::thread_rng();
    for j in 0..size {
        for i in 0..size {
            let prob: f64 = rng.gen(); // generates a float between 0 and 1
            let coords = (i as i32, j as i32);

            if prob > 0.2 {
                tiles.insert(coords);
            }
        }
    }

    return tiles;
}

pub fn get_distances(
    source: (i32, i32),
    tiles: &HashMap<(i32,i32), Tile>,
) -> HashMap<(i32, i32), (usize, Cardinal)>{
    let mut distances: HashMap<(i32, i32), (usize, Cardinal)> = HashMap::new();
    let mut c_stack: VecDeque<((i32,i32), (Cardinal, usize))> = VecDeque::new();
    for neighbor in Cardinal::iter_all(){
        if tiles.contains_key(&neighbor.delta_from(source)){
            c_stack.push_front((source, (neighbor, 0)));
        }
    }

    loop{
        //print_tiles(40, tiles, &distances);
        if let Some((coord, (cardinal, count))) = c_stack.pop_back(){
            (distances, c_stack) = traverse(coord, count, cardinal, &tiles, c_stack, distances);
        } else {
            break;
        }
    }

    distances
}


pub fn traverse(
    source: (i32, i32), 
    mut count: usize, 
    cardinal: Cardinal, 
    tiles: &HashMap<(i32,i32), Tile>, 
    mut c_stack: VecDeque<((i32,i32), (Cardinal, usize))>,
    mut distances: HashMap<(i32,i32), (usize, Cardinal)>
) -> (HashMap<(i32,i32), (usize, Cardinal)>, VecDeque<((i32,i32), (Cardinal, usize))>){
    let mut current_coord = source;

    loop{
        current_coord = cardinal.delta_from(current_coord);
        count += 1;
        if !tiles.contains_key(&current_coord) || distances.contains_key(&current_coord){
            break;
        } else {
            distances.insert(current_coord, (count, cardinal.inverse()));
        }
        let mut min = usize::MAX;
        for neighbor in Cardinal::iter_all(){
            let delta = neighbor.delta_from(current_coord);
            if let Some((size, _stored_cardinal)) = distances.get_mut(&delta){
                if *size < min {
                    min = *size;
                    if let Some((_current_size, current_coord)) = distances.get_mut(&current_coord){
                        *current_coord = neighbor;
                    }
                }
            } else {
                if neighbor!=cardinal{
                    c_stack.push_front((current_coord, (neighbor, count)));
                }
            }
        }
    }

    (distances, c_stack)
}

pub fn print_tiles( 
    source: (i32,i32), 
    size: i32, 
    tiles: &HashMap<(i32,i32), Tile>, 
    distances: &HashMap<(i32,i32), (usize, Cardinal)>
) {
    let mut to_print = String::new();
    for j in 0..size {
        for i in 0..size {
            let coords = (i as i32, j as i32);
            if coords == source {
                to_print.push('0');
            } else if let Some((_distance, cardinal)) = distances.get(&coords){
                to_print.push(cardinal.get_char_eq());
            } else if tiles.contains_key(&coords){
                to_print.push('.');
            } else {
                to_print.push('#');
            }

        }
        to_print.push('\n');
    }
    println!("{}", to_print);
}

pub fn print_tiles_dis( 
    source: (i32,i32), 
    size: i32, 
    tiles: &HashMap<(i32,i32), Tile>, 
    distances: &HashMap<(i32,i32), (usize, Cardinal)>
) {
    let mut to_print = String::new();
    for j in 0..size {
        for i in 0..size {
            let coords = (i as i32, j as i32);
            if coords == source {
                to_print.push_str("  0");
            } else if let Some((distance, _cardinal)) = distances.get(&coords){
                if *distance < 10 as usize {
                    to_print.push_str(&format!("  {}",distance));
                } else {
                    to_print.push_str(&format!(" {}",distance));
                }
            } else if tiles.contains_key(&coords){
                to_print.push_str("  .");
            } else {
                to_print.push_str("  #");
            }

        }
        to_print.push('\n');
    }
    println!("{}", to_print);
}

