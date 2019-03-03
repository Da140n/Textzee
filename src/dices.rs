use std::{
    char::from_u32_unchecked,
    fmt::{self, Display},
    ops::{Deref},
};

use rand::Rng;

pub struct Dices {
    n: usize,
    dices: Vec<Dice>,
}

impl Dices {
    pub fn gen_range(n: usize) -> Self {
        let mut dices: Vec<_> = std::iter::repeat_with(|| Dice::new()).take(n).collect();
        dices.sort_unstable();
        Self {
            n,
            dices,
        }
    }

    pub fn sum(&self) -> i16 {
        self.dices.iter().map(|n| n.deref()).sum()
    }
}

impl Display for Dices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (n, v) in self
            .dices
            .iter()
            .enumerate()
        {
            write!(f, "{}{}", v, if n != self.n - 1 { " " } else { "" })?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Dice {
    val: i16,
}

impl Dice {
    fn new() -> Self {
        Dice {
            val: rand::thread_rng().gen_range(1, 7),
        }
    }
}

impl Deref for Dice {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_ucode: u32 = 0x267F;
        let dice_code = match self.val {
            n if n >= 1 && n <= 6 => n as u32 + base_ucode,
            _ => unreachable!(),
        };
        //this is always safe as we know that the only outcome is the dices.
        //unreachable will panic in case it is not.
        write!(f, "{}", unsafe{from_u32_unchecked(dice_code)})
    }
}
