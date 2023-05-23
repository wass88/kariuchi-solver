mod game;
mod position;
mod sticks;

fn main() {
    let mut s = game::State::new();
    loop {
        println!("{}", s);
        if s.is_end() {
            break;
        }
        let sticks = sticks::Sticks::throw();
        println!("Throw {}", usize::from(sticks));
        let actions = s.actions(sticks);
        for i in 0..actions.len() {
            println!("- {} : {}", i, actions[i]);
        }
        print!("> ");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num = line.trim().parse::<usize>().unwrap();
        let act = actions[num];
        s.act(act);
    }
}
