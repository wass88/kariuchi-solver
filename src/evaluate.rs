use crate::{game::State, sticks::Sticks};

#[derive(Debug, Clone, Copy)]
pub struct Opt {
    pub ply: usize,
    pub rollout: usize,
}

#[derive(Debug, Clone)]
pub struct Evaluate {
    option: Opt,
    state: State,
}

impl Evaluate {
    pub fn new(option: Opt, state: State) -> Self {
        Evaluate { option, state }
    }
    pub fn evaluate(&self) -> f64 {
        self.expand(&self.state, self.option.ply)
    }
    fn leaf(state: &State, first: bool) -> Option<f64> {
        if state.is_end() {
            if state.is_win_first() == first {
                return Some(1.0);
            } else {
                return Some(0.0);
            }
        }
        None
    }
    fn expand(&self, state: &State, ply: usize) -> f64 {
        if let Some(p) = Self::leaf(state, state.is_first()) {
            return p;
        }
        if ply == 0 {
            return self.rollout(state);
        }
        let sticks = Sticks::all_sticks();
        let mut sum = 0.;
        for (p, s) in sticks {
            let o = self.solve(state, ply, s);
            sum += p * o;
        }
        sum
    }
    fn solve(&self, state: &State, ply: usize, sticks: Sticks) -> f64 {
        let actions = state.actions(sticks);
        let mut res = 0.;
        for act in actions {
            let mut s = state.clone();
            s.act(act);
            let p = 1. - self.expand(&s, ply - 1);
            if p > res {
                res = p;
            }
        }
        res
    }
    fn rollout(&self, state: &State) -> f64 {
        let mut sum = 0.;
        for i in 0..self.option.rollout {
            let s = state.clone();
            sum += self.rollout_once(s);
        }
        sum / self.option.rollout as f64
    }
    fn rollout_once(&self, mut state: State) -> f64 {
        let first = state.is_first();
        while !state.is_end() {
            let sticks = Sticks::throw();
            let actions = state.actions(sticks);
            let num = rand::random::<usize>() % actions.len();
            state.act(actions[num])
        }
        Self::leaf(&state, first).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{game::Action, position::Position};

    #[test]
    fn evaluate() {
        let opt = Opt { ply: 2, rollout: 2 };
        let mut state = State::new();
        println!("{}", state);
        let e = Evaluate::new(opt, state.clone());
        let p = e.evaluate();
        println!("{}", p);
        let actions = state.actions(Sticks::from(1));
        state.act(actions[0]);
        println!("{}", state);
        let e = Evaluate::new(opt, state);
        let p = e.evaluate();
        println!("{}", p);
    }
    #[test]
    fn almost_win() {
        let opt = Opt { ply: 2, rollout: 2 };
        let mut state = State::new();
        let goal = Position::goal();
        let back2 = Position::from(usize::from(goal) - 2);
        state.act(Action::new(Position::from(0), back2, 4));
        println!("{}", state);
        let e = Evaluate::new(opt, state.clone());
        let p = e.evaluate();
        println!("{}", p);

        let act = state.actions(Sticks::from(1))[0];
        state.act(act);
        println!("{}", state);
        let e = Evaluate::new(opt, state.clone());
        let p = e.evaluate();
        println!("{}", p);
    }
}
