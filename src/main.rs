use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    Q0,
    Q1,
    Q2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Input {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pair(State, Input);

// fn transiton(state: State, input: Input) -> State {}
fn main() {
    let transiton: HashMap<Pair, State> = HashMap::from([
        (Pair(State::Q0, Input::A), State::Q1),
        (Pair(State::Q0, Input::B), State::Q0),
        (Pair(State::Q1, Input::A), State::Q2),
        (Pair(State::Q1, Input::B), State::Q1),
        (Pair(State::Q2, Input::A), State::Q2),
        (Pair(State::Q2, Input::B), State::Q2),
    ]);

    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", State::Q0);
        println!("{:?}", Input::A);
    }
}
