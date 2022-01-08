pub struct Store<State: 'static, Action> {
    state: &'static mut State,
    reducer: Box<dyn Fn(&'static mut State, Action) -> ()>,
}

impl<State: 'static, Action> Store<State, Action> {
    pub fn new(
        reducer: Box<dyn Fn(&'static mut State, Action) -> ()>,
        initial_state: &'static mut State,
    ) -> Store<State, Action> {
        Store {
            state: initial_state,
            reducer,
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }
    pub fn dispatch(self, a: Action) {
        (self.reducer)(self.state, a);
    }
}
