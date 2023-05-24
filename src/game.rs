use std::fmt::{Display, Formatter};

use crate::{
    position::{self, Position, POSITIONS},
    sticks::Sticks,
};
const NUM_PIECES: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    first_pos: [Position; NUM_PIECES],
    second_pos: [Position; NUM_PIECES],
    first_turn: bool,
    winner: isize,
}

impl State {
    pub fn new() -> Self {
        State {
            first_pos: [Position::start(); NUM_PIECES],
            second_pos: [Position::start(); NUM_PIECES],
            first_turn: true,
            winner: 0,
        }
    }
    pub fn act(&mut self, act: Action) {
        let pos = self.get_piece(act.at);
        assert!(!pos.is_empty());
        assert!(act.num <= pos.len());
        for i in 0..act.num {
            self.current_pieces_mut()[pos[i]] = act.to;
        }
        if act.to != Position::goal() {
            if self.first_turn {
                for i in 0..NUM_PIECES {
                    if self.second_pos[i] == act.to {
                        self.second_pos[i] = Position::start();
                    }
                }
            } else {
                for i in 0..NUM_PIECES {
                    if self.first_pos[i] == act.to {
                        self.first_pos[i] = Position::start();
                    }
                }
            }
        }
        self.check_end();
        self.first_turn = !self.first_turn;
    }
    pub fn actions(&self, sticks: Sticks) -> Vec<Action> {
        let my_pieces = self.my_pieces();
        let mut actions = vec![];
        for i in 0..POSITIONS {
            if Position::from(i) == Position::goal() {
                continue;
            }
            let my_piece = my_pieces[i];
            if my_piece == 0 {
                continue;
            }
            let to = Position::from(i).advance(usize::from(sticks));
            for way in 0..to.len() {
                let movable = if i == 0 { 1 } else { my_piece };
                for m in 1..=movable {
                    let act = Action::new(Position::from(i), to[way], m);
                    actions.push(act);
                }
            }
        }
        actions
    }
    pub fn is_end(&self) -> bool {
        self.winner != 0
    }
    pub fn is_win_first(&self) -> bool {
        self.winner == 1
    }
    pub fn is_first(&self) -> bool {
        self.first_turn
    }
    fn check_end(&mut self) {
        let mut first = true;
        let mut second = true;
        for i in 0..NUM_PIECES {
            if self.first_pos[i] != Position::goal() {
                first = false;
            }
            if self.second_pos[i] != Position::goal() {
                second = false;
            }
        }
        if first {
            self.winner = 1;
        } else if second {
            self.winner = -1;
        } else {
            self.winner = 0;
        }
    }
    fn current_pieces_mut(&mut self) -> &mut [Position; NUM_PIECES] {
        if self.first_turn {
            &mut self.first_pos
        } else {
            &mut self.second_pos
        }
    }
    fn current_pieces(&self) -> &[Position; NUM_PIECES] {
        if self.first_turn {
            &self.first_pos
        } else {
            &self.second_pos
        }
    }
    fn get_piece(&self, at: Position) -> Vec<usize> {
        let pieces = self.current_pieces();
        let mut v = Vec::new();
        for i in 0..NUM_PIECES {
            if pieces[i] == at {
                v.push(i);
            }
        }
        v
    }
    fn my_pieces(&self) -> Vec<usize> {
        let mut board = vec![0; POSITIONS];
        for i in 0..NUM_PIECES {
            let pos = self.current_pieces()[i];
            board[usize::from(pos)] += 1
        }
        board
    }
    pub fn visual(&self) -> String {
        let mut board = vec![0; POSITIONS];
        for i in 0..NUM_PIECES {
            let pos = self.first_pos[i];
            board[usize::from(pos)] += 1;
            let pos = self.second_pos[i];
            board[usize::from(pos)] -= 1;
        }
        let (h, w, alignment) = position::alignment();
        let mut s = vec![" ".repeat(w); h];
        for i in 0..POSITIONS {
            let (y, x) = alignment[i];
            let str = if board[i] == 0 {
                format!("..")
            } else {
                let mark = if board[i] > 0 { "O" } else { "X" };
                let num = if board[i] > 0 { board[i] } else { -board[i] };
                format!("{}{}", mark, num)
            };
            s[y].replace_range(x..x + 2, &str);
        }
        s.join("\n")
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_end() {
            writeln!(
                f,
                "end: winner {}",
                if self.winner == 1 { "First" } else { "Second" }
            )?;
        } else {
            writeln!(
                f,
                "turn: {}",
                if self.first_turn { "first" } else { "second" }
            )?;
        }
        for i in 0..NUM_PIECES {
            write!(f, "{} ", self.first_pos[i])?;
        }
        write!(f, "/ ")?;
        for i in 0..NUM_PIECES {
            write!(f, "{} ", self.second_pos[i])?;
        }
        writeln!(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    at: Position,
    to: Position,
    num: usize,
}
impl Action {
    pub fn new(at: Position, to: Position, num: usize) -> Self {
        assert!(num > 0);
        Action { at, to, num }
    }
}
impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {}/{}({})", self.at, self.to, self.num)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_state() {
        let s = State::new();
        println!("{}", s);
    }
    #[test]
    fn actions() {
        let s = State::new();
        let actions = s.actions(Sticks::from(1));
        assert_eq!(actions.len(), 1);
        for act in actions {
            println!("{}", act);
        }
    }
    #[test]
    fn act() {
        let mut s = State::new();
        let actions = s.actions(Sticks::from(1));
        assert_eq!(actions.len(), 1);
        println!("{}", actions[0]);
        s.act(actions[0]);
        println!("{}", s);
        assert_eq!(s.first_pos[0], Position::from(1));
        let actions = s.actions(Sticks::from(2));
        println!("{}", actions[0]);
        s.act(actions[0]);
        println!("{}", s);
        assert_eq!(s.second_pos[0], Position::from(2));
    }
    #[test]
    fn hit() {
        let mut s = State::new();
        let actions = s.actions(Sticks::from(3));
        assert_eq!(actions.len(), 1);
        println!("{}", actions[0]);
        s.act(actions[0]);
        println!("{}", s);
        assert_eq!(s.first_pos[0], Position::from(3));
        let actions = s.actions(Sticks::from(3));
        println!("{}", actions[0]);
        s.act(actions[0]);
        println!("{}", s);
        assert_eq!(s.second_pos[0], Position::from(3));
        assert_eq!(s.first_pos[0], Position::from(0));
    }
    #[test]
    fn split_move() {
        let mut s = State::new();
        s.act(s.actions(Sticks::from(2))[0]);
        s.act(s.actions(Sticks::from(1))[0]);
        s.act(s.actions(Sticks::from(2))[0]);
        s.act(s.actions(Sticks::from(1))[0]);

        println!("{}", s);
        let actions = s.actions(Sticks::from(2));
        for act in &actions {
            println!("{}", act);
        }
        assert_eq!(actions.len(), 3);
        s.act(actions[1]);
        println!("{}", s);
        assert_eq!(s.first_pos[0], Position::from(4));
        assert_eq!(s.first_pos[1], Position::from(2));

        let actions = s.actions(Sticks::from(3));
        for act in &actions {
            println!("{}", act);
        }
        s.act(actions[2]);
        println!("{}", s);
        assert_eq!(s.second_pos[0], Position::from(4));
        assert_eq!(s.second_pos[1], Position::from(4));
        assert_eq!(s.first_pos[0], Position::from(0));
    }
    #[test]
    fn print_visual() {
        let s = State::new();
        println!("{}", s.visual());
    }
}
