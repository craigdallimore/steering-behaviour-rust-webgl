## TODO

- [ ] Get livereload working with wasm
- [ ] Migrate the DOM bindings, etc from the javascript project
- [ ] Find a way to optimise JS size
- [ ] Find a way to optimise wasm size [link](https://rustwasm.github.io/docs/book/game-of-life/code-size.html)

- [ ] Make the grid start at top left

- [ ] implementation
  - [x] implement Arrive
  - [ ] implement CollisionAvoidance
  - [x] implement Evade
  - [x] implement Flee
  - [ ] implement FollowPathChaseRabbit
  - [ ] implement FollowPathPredict
  - [x] implement LookWhereYouAreGoing
  - [x] implement MatchVelocity
  - [ ] implement ObstacleAvoidance
  - [x] implement Pursue
  - [ ] implement Separation
  - [x] implement Wander

I want to be able to loop over the characters in my state.
For each character, I want to call an associated method on them called "apply_behaviour" which mutates just the character, and references the rest of the state

Options:
- explore sub structs
- copy/derive what is needed, maybe |state| { data }
  - probably _shouldn't_ do calcs against everthing in state
- just copy state each time
- go fully functional (then why use rust?)
- ask about this in rust group?

