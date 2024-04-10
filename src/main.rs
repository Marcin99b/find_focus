use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    println!("{}", args[1])
}

/*
Idea
User set timerange when is focused, tired, bored, distracted etc
with optional information what he/she was doing (what caused this state)

App save it and try to find pattern when these states occurs mostly
*/