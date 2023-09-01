use bevy::math::Vec2;
use super::name_generator::get_random_entry_from_file;

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
    pub hp: i32,
    max_hp: i32,
    stamina: i32,
    max_stamina: i32,
    location: Vec2, // May not be needed based on changes I saw in JBA_brainstorm

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
}

impl Default for Worker {
    fn default() -> Worker {
        Worker { // TODO: have more randomization
            name: get_random_entry_from_file("src/workers/names/worker_names.txt"),
            active_state: ActiveState::Active,
            passive_states: vec![PassiveState::Happy],
            hp: 75,
            max_hp: 100,
            stamina: 75,
            max_stamina: 100,
            location: Vec2::new(0.0, 0.0) // May not be needed based on changes I saw in JBA_brainstorm
        }
    }
}

#[cfg(test)]
mod tests { // TODO: Make tests better and add assertions. Also, probably will want to move these somewhere else at some point


    use crate::workers::worker_group::WorkerGroup;
    use super::*;

    #[test]
    fn create_new_worker_group() {
        let worker_group = WorkerGroup::new();
        println!("{:#?}", worker_group)
    }

    #[test]
    fn create_worker_group_from_workers() {
        let workers = vec![Worker::new(), Worker::new(), Worker::new()];
        let leader = Worker::new();
        let worker_group = WorkerGroup::from_workers(leader, workers);
        println!("{:#?}", worker_group)
    }

    #[test]
    fn remove_quitters() {
        let workers = vec![
            Worker::from_state(ActiveState::QuittingGroup, vec![PassiveState::Happy]), 
            Worker::from_state(ActiveState::Active, vec![PassiveState::Angry]), 
            Worker::from_state(ActiveState::QuittingGroup, vec![PassiveState::Happy, PassiveState::Sick(Disease::Worm)]),
            Worker::from_state(ActiveState::Annoying, vec![PassiveState::Sick(Disease::Cold)]),
            Worker::from_state(ActiveState::Resting, vec![PassiveState::Sick(Disease::Cold)]),
        ];
        let leader = Worker::new();

        let mut worker_group = WorkerGroup::from_workers(leader, workers);
        let quitters = worker_group.remove_quitters();

        assert_eq!(quitters.len(), 2);
        assert_eq!(worker_group.workers.len(), 3);
    }

    #[test]
    fn remove_quitters_no_quitters() {
        let workers = vec![
            Worker::from_state(ActiveState::Active, vec![PassiveState::Angry]), 
            Worker::from_state(ActiveState::Annoying, vec![PassiveState::Sick(Disease::Cold)]),
            Worker::from_state(ActiveState::Resting, vec![PassiveState::Sick(Disease::Cold)]),
        ];
        let leader = Worker::new();

        let mut worker_group = WorkerGroup::from_workers(leader, workers);
        let quitters = worker_group.remove_quitters();

        assert_eq!(quitters.len(), 0);
        assert_eq!(worker_group.workers.len(), 3);
    }

    #[test]
    fn remove_quitters_empty_group() {
        let workers = vec![];
        let leader = Worker::new();

        let mut worker_group = WorkerGroup::from_workers(leader, workers);
        let quitters = worker_group.remove_quitters();

        assert_eq!(quitters.len(), 0);
        assert_eq!(worker_group.workers.len(), 0);
    }

    #[test]
    fn remove_quitters_all_quitters() {
        let workers = vec![
            Worker::from_state(ActiveState::QuittingGroup, vec![PassiveState::Angry]), 
            Worker::from_state(ActiveState::QuittingGroup, vec![PassiveState::Sick(Disease::Cold)]),
            Worker::from_state(ActiveState::QuittingGroup, vec![PassiveState::Sick(Disease::Cold)]),
        ];
        let leader = Worker::new();

        let mut worker_group = WorkerGroup::from_workers(leader, workers);
        let quitters = worker_group.remove_quitters();

        assert_eq!(quitters.len(), 3);
        assert_eq!(worker_group.workers.len(), 0);
    }
}
