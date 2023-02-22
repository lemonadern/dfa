use core::hash::Hash;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct Pair<S, T>(pub S, pub T);
type TransitionMap<S, T> = HashMap<Pair<S, T>, S>;

#[allow(dead_code)]
pub struct Automata<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    current: S,
    transition: TransitionMap<S, T>,
    // TODO: 受理状態が複数存在できるように
    accepting: S,
}

impl<S, T> Automata<S, T>
where
    S: Eq + Hash + Clone + Copy,
    T: Eq + Hash + Clone + Copy,
{
    pub fn new(initial_state: S, final_state: S, transition: TransitionMap<S, T>) -> Self {
        Self {
            current: initial_state,
            transition,
            accepting: final_state,
        }
    }

    fn transit(&mut self, input: &T) -> () {
        // 決定性有限オートマトンでは、入力があるとき必ず遷移先が存在するため失敗しない
        let next_state = self.transition.get(&Pair(self.current, *input)).unwrap();

        self.current = *next_state;
    }

    pub fn accept(&mut self, input: &[T]) -> bool {
        input.iter().for_each(|x| self.transit(x));
        self.current == self.accepting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
