mod evaluate;
mod game;
mod position;
mod sticks;

fn main() {
    let mut s = game::State::new();
    let opt = evaluate::Opt {
        ply: 3,
        rollout: 10,
    };
    loop {
        println!("{}", s);
        println!("{}", s.visual());
        if s.is_end() {
            break;
        }
        let sticks = sticks::Sticks::throw();
        println!("Throw {}", usize::from(sticks));
        let actions = s.actions(sticks);
        for i in 0..actions.len() {
            let mut st = s.clone();
            st.act(actions[i]);
            print!("{}", st);
            let p1 = 1. - evaluate::Evaluate::new(opt, st.clone()).evaluate();
            let p2 = 1. - evaluate::Evaluate::new(opt, st.clone()).evaluate();
            let p3 = 1. - evaluate::Evaluate::new(opt, st.clone()).evaluate();
            println!(
                "- {} : {: <10} [{:0.4}, {:0.4}, {:0.4}]",
                i, actions[i], p1, p2, p3
            );
        }
        if s.is_first() {
            let num = read_int(actions.len());
            let act = actions[num];
            println!("Player {}", act);
            s.act(act);
        } else {
            let num = rand::random::<usize>() % actions.len();
            let act = actions[num];
            println!("CPU {}", act);
            s.act(act);
        }
    }
}

fn read_int(max: usize) -> usize {
    loop {
        print!("> ");
        use std::io::Write;
        if let Err(e) = std::io::stdout().flush() {
            println!("Error: {}", e);
            continue;
        }
        let mut line = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut line) {
            println!("Error: {}", e);
            continue;
        }
        if line.trim() == "" && max == 1 {
            println!("0 (forced)");
            return 0;
        }
        let num = match line.trim().parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        if num >= max {
            println!("Error: out of range");
            continue;
        }
        return num;
    }
}
