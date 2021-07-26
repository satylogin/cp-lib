type Data = u128;

const MOD: Data = 1e9 as Data + 7;

pub trait ModMath {
    fn madd(&mut self, other: Data) -> Data;
    fn msub(&mut self, other: Data) -> Data;
    fn mmul(&self, other: Data) -> Data;
    fn mdiv(&self, other: Data) -> Data;
    fn mpow(&self, power: Data) -> Data;
    fn minverse(&self) -> Data;
}

impl ModMath for Data {
    fn madd(&mut self, other: Data) -> Data {
        *self += other;
        if *self >= MOD {
            *self -= MOD;
        }
        *self
    }
    fn msub(&mut self, other: Data) -> Data {
        if other > *self {
            *self += MOD;
        }
        *self -= other;
        *self
    }
    fn mmul(&self, other: Data) -> Data {
        ((self % MOD) * (other % MOD)) % MOD
    }
    fn mdiv(&self, other: Data) -> Data {
        self.mmul(other.minverse())
    }

    /// Uses Euler's totient theorem, hence only works when `self` and `MOD` are coprime
    fn mpow(&self, mut power: Data) -> Data {
        power %= MOD - 1;
        let mut base = *self;
        base %= MOD;
        let mut ans: Data = 1;
        while power > 0 {
            if power & 1 != 0 {
                ans = ans * base % MOD;
            }
            base = base * base % MOD;
            power >>= 1;
        }
        ans
    }

    /// Uses Fermat's Little Theorem, hence only works when `MOD` is prime
    fn minverse(&self) -> Data {
        self.mpow(MOD - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mexp() {
        assert_eq!(568493343, 2.mpow(293322));
    }
}
