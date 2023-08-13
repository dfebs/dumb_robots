use crate::map_gen::*;

/* I figured that we could use our tests as make shift background
 * monitor until we get a UI running in bevy
 *
 * Use the make file to run the tests, otherwise the print macros won't work
 * If you can't use the Makefile for some reason, you can also run cargo
 * like so:
 *
 *     cargo test -- --nocapture
 */

#[test]
fn test_world_gen() {
    println!();
    println!("Running World Generation Test");
    let world = World::generate(32);
    world.print();
    println!();
}
