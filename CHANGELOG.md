# Change Log

### 2023-08-18
- Added ability to remove workers that want to leave the group using `remove_quitters`. This will take the quitters out of the group, and return them as a vec
- Added some unit tests for `remove_quitters`

### 2023-08-13

- Created base structs for Worker and WorkerGroup
- Added tests to illustrate WorkerGroup initialization (cargo test -- --nocapture to see it in action)
- Added ability to create Worker and WorkerGroup with random names