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
        let tries = 3;
        let p = evaluate_par(s.clone(), &actions, opt, tries);
        for i in 0..actions.len() {
            print!("- {} : {: <15} [", i, format!("{}", actions[i]));
            for k in 0..tries {
                print!("{:0.4}", p[i][k]);
                if k < tries - 1 {
                    print!(", ");
                } else {
                    println!("]")
                }
            }
        }
        if s.is_first() {
            let num = read_int(actions.len());
            let act = actions[num];
            println!("Player {}", act);
            s.act(act);
        } else {
            let i = p
                .iter()
                .map(|a| a.iter().sum::<f64>())
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index)
                .unwrap();
            let act = actions[i];
            println!("CPU {}", act);
            s.act(act);
        }
    }
}

fn evaluate_par(
    state: game::State,
    actions: &[game::Action],
    opt: evaluate::Opt,
    tries: usize,
) -> Vec<Vec<f64>> {
    use rayon::prelude::*;
    use std::io::Write;
    use std::sync::Mutex;
    let solved = std::sync::Arc::<Mutex<usize>>::new(Mutex::new(0));
    let max = actions.len() * tries;
    print!("{} / {}", solved.lock().unwrap(), max);
    std::io::stdout().flush().unwrap();

    let res = actions
        .par_iter()
        .map(|act| {
            (0..tries)
                .into_par_iter()
                .map(|_| {
                    let mut st = state.clone();
                    st.act(*act);
                    let p = 1. - evaluate::Evaluate::new(opt, st.clone()).evaluate();
                    let mut solved = solved.lock().unwrap();
                    *solved += 1;
                    print!("\r{} / {}", solved, max);
                    std::io::stdout().flush().unwrap();
                    p
                })
                .collect()
        })
        .collect();
    print!("\r");
    res
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
