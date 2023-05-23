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
