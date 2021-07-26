type Mod = u128;

const MOD: Mod = 1e9 as Mod + 7;

pub trait ModMath {
    fn madd(&self, other: Mod) -> Mod;
    fn msub(&self, other: Mod) -> Mod;
    fn mmul(&self, other: Mod) -> Mod;
    fn mdiv(&self, other: Mod) -> Mod;
    fn mpow(&self, power: Mod) -> Mod;
    fn minverse(&self) -> Mod;
}

impl ModMath for Mod {
    fn madd(&self, other: Mod) -> Mod {
        ((self % MOD) + (other % MOD)) % MOD
    }
    fn msub(&self, mut other: Mod) -> Mod {
        other %= MOD;
        if other > self % MOD {
            ((self % MOD) + MOD - (other % MOD)) % MOD
        } else {
            ((self % MOD) - (other % MOD)) % MOD
        }
    }
    fn mmul(&self, other: Mod) -> Mod {
        ((self % MOD) * (other % MOD)) % MOD
    }
    fn mdiv(&self, other: Mod) -> Mod {
        self.mmul(other.minverse())
    }
    fn mpow(&self, mut power: Mod) -> Mod {
        let mut base = *self;
        base %= MOD;
        let mut ans: Mod = 1;
        while power > 0 {
            if power & 1 != 0 {
                ans = ans * base % MOD;
            }
            base = base * base % MOD;
            power >>= 1;
        }
        ans
    }
    fn minverse(&self) -> Mod {
        // Uses Fermat's Little Theorem, hence only works when MOD is prime
        self.mpow(MOD - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(106889461, 51757006.mmul(2.mpow(293322).msub(1)));
    }
}
