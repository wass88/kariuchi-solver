const NUM_STICKS: usize = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sticks(usize);

impl Sticks {
    pub fn throw() -> Self {
        let mut num = 0;
        for i in 0..NUM_STICKS {
            num += rand::random::<usize>() % 2;
        }
        if num == 0 {
            Sticks(NUM_STICKS + 1)
        } else {
            Sticks(num)
        }
    }
    pub fn all_sticks() -> Vec<(f64, Self)> {
        let mut vec = Vec::new();
        let mut over = 1.;
        let mut under = 1.;
        for i in 0..=NUM_STICKS {
            let p = over / under / (2f64.powf(NUM_STICKS as f64) as f64);
            let c = if i == 0 { 5 } else { i };
            under *= (i + 1) as f64;
            over *= (NUM_STICKS - i) as f64;
            vec.push((p, Sticks(c)));
        }
        vec
    }
}
impl From<usize> for Sticks {
    fn from(x: usize) -> Self {
        Sticks(x)
    }
}
impl From<Sticks> for usize {
    fn from(s: Sticks) -> Self {
        s.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn all_sticks() {
        let vec = Sticks::all_sticks();
        for (p, s) in vec {
            println!("{} {}", usize::from(s), p);
        }
    }
}
