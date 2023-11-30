#[derive(Debug, Copy, Clone)]
pub struct Interval {
    // range of t
    pub tmin: f64,
    pub tmax: f64,
}

impl Interval {
    pub fn new(tmin: f64, tmax: f64) -> Interval {
        Interval { tmin, tmax }
    }

    pub fn merge (op1: &Self, op2: &Self) -> Self {
        Interval {
            tmin: f64::min(op1.tmin, op2.tmin),
            tmax: f64::max(op1.tmax, op2.tmax),
        }
    }

    pub fn in_between_closed(&self, t: f64) -> bool {
        self.tmax >= t && self.tmin <= t
    }

    pub fn in_between_open(&self, t: f64) -> bool {
        self.tmax > t && self.tmin < t
    }

    pub fn set_tmax(&mut self, t: f64) {
        self.tmax = t;
    }

    pub fn set_tmin(&mut self, t: f64) {
        self.tmin = t;
    }
}
