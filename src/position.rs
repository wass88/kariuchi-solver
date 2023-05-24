use std::fmt::{self, Display, Formatter};

const S_ON_START: usize = 0;
const ON_START: usize = 1;
const S_ON_ROUTES: usize = S_ON_START + ON_START;
const ON_CURVE: usize = 5;
const NUM_CURVE: usize = 6;
const ON_ROUTES: usize = ON_CURVE * NUM_CURVE;
const S_ON_GOAL: usize = S_ON_ROUTES + ON_ROUTES;
const ON_GOAL: usize = 1;
const S_ON_SHORTCUTS: usize = S_ON_GOAL + ON_GOAL;
const NUM_SHORTCUTS: usize = 6;
pub const OPP_SHORTCUTS: usize = NUM_SHORTCUTS / 2;
const ON_SHORTCUT: usize = 3;
const S_ON_CENTER: usize = S_ON_SHORTCUTS + NUM_SHORTCUTS * ON_SHORTCUT;
const ON_CENTER: usize = 1;
pub const POSITIONS: usize = S_ON_CENTER + ON_CENTER;

/*
0 | 1 2 3 4 5  |6 7 8 9 10| 11 12....
            a1          b1
            a2          b2
            a3          b3
31 f3 f2 f1 C
            d1
            d2
            d3
            20
*/

/*
** a = 0, b = 1 ... f = 5 **
 5 -> a -> C -> d -> 20
10 -> b -> C -> e -> 25
15 -> c -> C -> f -> 30
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HumanPosition {
    START,
    ROUTE(usize),
    GOAL,
    CENTER,
    SHORTCUT(usize, usize),
}

impl Position {
    pub fn start() -> Self {
        Position(S_ON_START)
    }
    fn route(x: usize) -> Self {
        assert!(S_ON_START <= x && x < S_ON_GOAL);
        Position(x)
    }
    pub fn goal() -> Self {
        Position(S_ON_GOAL)
    }
    fn center() -> Self {
        Position(S_ON_CENTER)
    }
    fn shortcut(path: usize, x: usize) -> Self {
        assert!(path < NUM_SHORTCUTS);
        assert!(x < ON_SHORTCUT);
        Position(S_ON_SHORTCUTS + path * ON_SHORTCUT + x)
    }
    fn on_route(x: usize) -> Self {
        if x >= S_ON_GOAL {
            Position::goal()
        } else {
            Position::route(x)
        }
    }
    fn can_shortcut(self) -> Option<usize> {
        let x = self.0;
        if x == S_ON_ROUTES + ON_CURVE - 1 {
            Some(0)
        } else if x == S_ON_ROUTES + 2 * ON_CURVE - 1 {
            Some(1)
        } else if x == S_ON_ROUTES + 3 * ON_CURVE - 1 {
            Some(2)
        } else {
            None
        }
    }
    fn to_shortcut(path: usize, x: usize) -> Self {
        if path < OPP_SHORTCUTS {
            if x < ON_SHORTCUT {
                Position::shortcut(path, x)
            } else if x == ON_SHORTCUT {
                Position::center()
            } else {
                Position::to_shortcut(path + OPP_SHORTCUTS, x - ON_SHORTCUT - ON_CENTER)
            }
        } else {
            if x < ON_SHORTCUT {
                Position::shortcut(path, x)
            } else {
                Position::on_route(x - ON_SHORTCUT + (path + 1) * ON_CURVE)
            }
        }
    }
    pub fn advance(self, d: usize) -> Vec<Position> {
        match self.into() {
            HumanPosition::START => {
                vec![Position::route(d)]
            }
            HumanPosition::ROUTE(x) => {
                let simple = Position::on_route(x + d);
                let x = Position::route(x);
                if let Some(p) = x.can_shortcut() {
                    vec![simple, Position::to_shortcut(p, d - 1)]
                } else {
                    vec![simple]
                }
            }
            HumanPosition::GOAL => {
                vec![]
            }
            HumanPosition::SHORTCUT(path, x) => {
                vec![Position::to_shortcut(path, x + d)]
            }
            HumanPosition::CENTER => {
                let mut v = vec![];
                for i in OPP_SHORTCUTS..(OPP_SHORTCUTS * 2) {
                    v.push(Position::to_shortcut(i, d));
                }
                v
            }
        }
    }
}
impl From<Position> for usize {
    fn from(x: Position) -> Self {
        x.0
    }
}
impl Into<HumanPosition> for Position {
    fn into(self) -> HumanPosition {
        let x = self.0;
        if x == S_ON_START {
            HumanPosition::START
        } else if x == S_ON_GOAL {
            HumanPosition::GOAL
        } else if x == S_ON_CENTER {
            HumanPosition::CENTER
        } else if x < S_ON_SHORTCUTS {
            HumanPosition::ROUTE(x)
        } else {
            let path = (x - S_ON_SHORTCUTS) / ON_SHORTCUT;
            let pos = (x - S_ON_SHORTCUTS) % ON_SHORTCUT;
            HumanPosition::SHORTCUT(path, pos)
        }
    }
}
impl From<usize> for Position {
    fn from(x: usize) -> Self {
        assert! {x < POSITIONS}
        Position(x)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (*self).into() {
            HumanPosition::START => write!(f, "S"),
            HumanPosition::ROUTE(x) => write!(f, "R{}", x),
            HumanPosition::GOAL => write!(f, "G"),
            HumanPosition::CENTER => write!(f, "C"),
            HumanPosition::SHORTCUT(path, x) => write!(f, "K{}_{}", path, x),
        }
    }
}

pub fn alignment() -> (usize, usize, Vec<(usize, usize)>) {
    let mut a = vec![(0, 0); POSITIONS];
    let cw = 3;
    let h = ON_CURVE * 2 + 3;
    let w = 80;
    a[S_ON_START] = (2, 2);
    a[S_ON_GOAL] = (2, 2);
    a[S_ON_CENTER] = (ON_CURVE + 1, OPP_SHORTCUTS * ON_CURVE / 2 * cw);
    for p in 0..NUM_SHORTCUTS {
        for i in 0..ON_CURVE {
            let path = p * ON_CURVE + i;
            if p < OPP_SHORTCUTS {
                a[S_ON_ROUTES + path] = (0, path * cw)
            } else {
                a[S_ON_ROUTES + path] = (ON_CURVE * 2 + 2, (ON_ROUTES - path - 1) * cw)
            }
        }
    }
    for p in 0..NUM_SHORTCUTS {
        for i in 0..ON_SHORTCUT {
            let path = p * ON_SHORTCUT + i;
            let skip = ON_CURVE * cw;
            if p < OPP_SHORTCUTS {
                a[S_ON_SHORTCUTS + path] = (i + 2, (p + 1) * skip - cw);
            } else {
                a[S_ON_SHORTCUTS + path] = (i + NUM_CURVE + 2, (NUM_SHORTCUTS - p - 1) * skip);
            }
        }
    }
    (h, w, a)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_positions() {
        for i in 0..POSITIONS {
            println!("{}: {}", i, Position::from(i));
        }
    }
    #[test]
    fn advance_one() {
        for i in 0..POSITIONS {
            let p = Position::from(i);
            print!("{} ->", p);
            let v = p.advance(1);
            for x in v {
                print!(" {}", x);
            }
            println!();
        }
    }
    #[test]
    fn advance_five() {
        for i in 0..POSITIONS {
            let p = Position::from(i);
            print!("{} ->", p);
            let v = p.advance(5);
            for x in v {
                print!(" {}", x);
            }
            println!();
        }
    }
}
