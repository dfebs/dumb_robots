use crate::workers::worker_group::WorkerGroup;

// Still heavily under construction
// Some events should pause the game. Maybe when an event happens, the game zooms in on the group that is associated with the event.
// Sometimes events can be notifications that happen as a result of the group finding something within their field of vision. 
// Maybe they find a computer or something that could have info, but could come at the cost of combat.

struct Event {
    decision: Some(Decision), // Some events are just notifications and you can't immediately do anything about them
    affected_group: &WorkerGroup
}

struct Decision {
    prompt: String,
    choices: Vec<String>
}