type Data = i64;

const MOD: Data = 1e9 as Data + 7;

pub trait Modulo {
    fn add(&mut self, other: Data);

    fn sub(&mut self, other: Data);
}

impl Modulo for Data {
    fn add(&mut self, other: Data) {
        *self += other;
        if *self >= MOD {
            *self -= MOD;
        }
    }

    fn sub(&mut self, other: Data) {
        *self -= other;
        if *self < 0 {
            *self += MOD;
        }
    }
}
