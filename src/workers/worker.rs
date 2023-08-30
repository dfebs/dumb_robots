use bevy::math::Vec2;
use crate::workers::name_generator::get_random_entry_from_file;
use rand::Rng;

#[derive(Debug)]
pub enum ActiveState {
    Active, // 'Active' could instead be split up into different types of work
    Halt,
    Annoying,
    Resting,
    Fighting,
    QuittingGroup
}

#[derive(Debug)]
pub enum PassiveState {
    Angry, // Chance to Halt out of defiance
    Happy, // Stat boosts
    Sick(Disease)
}

#[derive(Debug)]
pub enum Trait {
    Fighter,
    Hunter,
    Miner,
    Engineer,
    Scout,
    Healer,
    Immune, // can't get sick
    Coordinated, // more likely to be leader
}

// I like the idea of introducing conflict that goes against the player's desire to make large groups.
#[derive(Debug)]
pub enum Disease { 
    Cold, // highly contagious, low stat repercussions
    Worm, // I won't go into detail here, but they will very quickly get hungry. Stay away from these guys
    Hysteria, // More of a memetic threat than a biological one. Worker behavior is far less predictable. Could maybe take Some() value for a hyperfixation on an ActiveState  
}

#[derive(Debug)]
pub struct Worker {
    name: String,
    pub active_state: ActiveState,
    passive_states: Vec<PassiveState>,
    traits: Vec<Trait>,
    pub hp: i32,
    max_hp: i32,
    stamina: i32,
    max_stamina: i32,
    location: (i32, i32), // May not be needed based on changes I saw in JBA_brainstorm

    // My idea with relationships is having a ref to a worker
    // that would map to a numeric relationship value (1-10). 
    // However, that would require some lifetime related stuff which would be annoying.
    // There's almost definitely a better approach here.
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            ..Default::default()
        }
    }

    pub fn from_state(active_state: ActiveState, passive_states: Vec<PassiveState>) -> Self { // This could potentially just be expanded to entail some more or all of the properties.
        Worker {
            active_state,
            passive_states,
            ..Default::default()
        }
    }

    pub fn from_random() -> Self {
        let mut rng = rand::thread_rng();
        let max_hp = rng.gen_range(50..=100);
        let max_stamina = rng.gen_range(50..=100);
        // TODO: provide random assortment of traits and passive states. Give one random active state from subset of all active states.
        Worker {
            max_hp,
            max_stamina,
            hp:max_hp - rng.gen_range(0..=30),
            stamina: max_stamina - rng.gen_range(0..=30),
            ..Default::default()
        }
    }


}

impl Default for Worker {
    fn default() -> Worker {
        Worker { // TODO: have more randomization
            name: get_random_entry_from_file("src/workers/names/worker_names.txt"),
            active_state: ActiveState::Active,
            passive_states: vec![PassiveState::Happy],
            traits: vec![Trait::Engineer, Trait::Immune],
            hp: 75,
            max_hp: 100,
            stamina: 75,
            max_stamina: 100,
            location: (0, 0) // May not be needed based on changes I saw in JBA_brainstorm
        }
    }
}