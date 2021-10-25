use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum State {
    Locked,
    Unlocked,
}

#[derive(Debug)]
enum Event {
    Push,
    Coin,
}

#[allow(dead_code)]
fn next_state(state: State, event: Event) -> State {
    match state {
        State::Locked => match event {
            Event::Coin => State::Unlocked,
            Event::Push => State::Locked,
        },
        State::Unlocked => match event {
            Event::Coin => State::Unlocked,
            Event::Push => State::Locked,
        },
    }
}

fn main() {
    let mut state = State::Locked;
    let stdin = std::io::stdin();

    println!("State: {:?}", state);

    // User input
    print!("> ");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            "quit" => return,
            _unknown => eprintln!("ERROR: Only 'coin', 'push' OR 'quit' are allowed"),
        }
        println!("State: {:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
