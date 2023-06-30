use std::{collections::HashMap, hash::Hash, fmt::Debug};

use serde_json::Value;
use crate::{FiniteStateMachine, State, Transition, BTSet, Symbol, DeterministicFiniteAutomaton};

pub trait AutomateJsonIO{
    fn from_json(content_json : &Value) -> Self;
    fn from_json_file(path : &str) -> Self;
}

pub trait AutomateTrait<T : Clone + Hash +Debug>{
    fn get_fsm(&self) -> &FiniteStateMachine;
    fn get_start(&self) -> &T;
    fn get_starts(&self) -> &T;
    fn get_delta(&self) -> &HashMap<Transition<State>, T>;
    fn get_states(&self) -> &BTSet<State> {
        self.get_fsm().get_states()
    }
    fn get_alphabet(&self)-> &BTSet<Symbol> {
        self.get_fsm().get_alphabet()
    }
    fn get_ends(&self)-> &BTSet<State>  {
        self.get_fsm().get_ends()
    }
    fn accept(&self, _word: &str) -> bool;
    fn to_dfa(&self) -> DeterministicFiniteAutomaton;
}
