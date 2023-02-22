use std::collections::HashMap;

use dfa::{Automata, Pair};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Q0,
    Q1,
    Q2,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Symbol {
    A,
    B,
}

#[test]
fn acceptor_construction() {
    let transiton: HashMap<Pair<State, Symbol>, State> = HashMap::from([
        (Pair(State::Q0, Symbol::A), State::Q1),
        (Pair(State::Q0, Symbol::B), State::Q0),
        (Pair(State::Q1, Symbol::A), State::Q2),
        (Pair(State::Q1, Symbol::B), State::Q1),
        (Pair(State::Q2, Symbol::A), State::Q2),
        (Pair(State::Q2, Symbol::B), State::Q2),
    ]);
    Automata::new(State::Q0, State::Q2, transiton);
}

#[test]
fn accept_aaa() {
    let transiton: HashMap<Pair<State, Symbol>, State> = HashMap::from([
        (Pair(State::Q0, Symbol::A), State::Q1),
        (Pair(State::Q0, Symbol::B), State::Q0),
        (Pair(State::Q1, Symbol::A), State::Q2),
        (Pair(State::Q1, Symbol::B), State::Q0),
        (Pair(State::Q2, Symbol::A), State::Q2),
        (Pair(State::Q2, Symbol::B), State::Q0),
    ]);
    let mut automata = Automata::new(State::Q0, State::Q2, transiton);

    let input = [Symbol::A, Symbol::A, Symbol::A];

    assert!(automata.accept(&input))
}

#[test]
fn does_not_accept_aba() {
    let transiton: HashMap<Pair<State, Symbol>, State> = HashMap::from([
        (Pair(State::Q0, Symbol::A), State::Q1),
        (Pair(State::Q0, Symbol::B), State::Q0),
        (Pair(State::Q1, Symbol::A), State::Q2),
        (Pair(State::Q1, Symbol::B), State::Q0),
        (Pair(State::Q2, Symbol::A), State::Q2),
        (Pair(State::Q2, Symbol::B), State::Q0),
    ]);
    let mut automata = Automata::new(State::Q0, State::Q2, transiton);

    let input = [Symbol::A, Symbol::B, Symbol::A];

    assert!(!automata.accept(&input))
}
