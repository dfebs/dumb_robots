use crate::workers::worker::{Worker, ActiveState, PassiveState, Disease}; // This feels a tad goofy; there's probably a better way to organize this
use crate::workers::name_generator::get_random_entry_from_file;
use std::ops;

#[derive(Debug)]
pub struct WorkerGroup {
    workers: Vec<Worker>,
    leader: Worker,
    group_name: String
}

impl WorkerGroup {

    // Used when game starts and groups are being made for the first time
    // TODO have more randomness
    fn new() -> Self { 
        WorkerGroup { 
            leader: Worker::new(),
            workers: vec![Worker::new(), Worker::new(), Worker::new()],
            group_name: get_random_entry_from_file("src/workers/names/group_names.txt")
        }
    }

    fn from_workers(leader: Worker, workers: Vec<Worker>) -> Self {
        WorkerGroup {
            leader,
            workers,
            group_name: get_random_entry_from_file("src/workers/names/group_names.txt")
        }
    }

    fn absorb(&mut self, mut other_group: WorkerGroup) {
        self.workers.push(other_group.leader);
        self.workers.append(&mut other_group.workers); 
        // If I'm correct, self.workers takes all the workers and leader from other_group, then other_group goes out of scope
    }

    fn remove_quitters(&mut self) -> Vec<Worker> { 
        let quitter_indices : Vec<usize> = self.workers
                .iter()
                .enumerate()
                .filter(|(_, worker)| matches!(worker.active_state, ActiveState::QuittingGroup))
                .map(| (index, _) | index)
                .rev()
                .collect();

        quitter_indices
                .iter()
                .map(|index| self.workers.swap_remove(*index))
                .collect()
        
    }

    pub fn merge(first_group: WorkerGroup, second_group: WorkerGroup) -> WorkerGroup {
        let mut leading_group;
        if first_group.leader.hp > second_group.leader.hp { // for now we'll use hp as basis for who the new leader will be
            leading_group = first_group;
            leading_group.absorb(second_group)
        } else {
            leading_group = second_group;
            leading_group.absorb(first_group)
        }

        leading_group
    }
}

#[cfg(test)]
mod tests { // TODO: Make tests better and add assertions. Also, probably will want to move these somewhere else at some point
    use crate::workers::worker;

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