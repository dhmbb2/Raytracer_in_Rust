
pub struct ROT {
    // range of t
    tmax: f64,
    tmin: f64,
}

impl ROT {
    pub fn new(tmax: f64, tmin: f64) -> ROT {
        ROT {
            tmax,
            tmin,
        }
    }

    pub fn in_between_closed(&self, t: f64) -> bool {
        self.tmax >= t && self.tmin <= t
    }

    pub fn in_between_open(&self, t: f64) -> bool {
        self.tmax > t && self.tmin < t
    }

    pub fn set_tmax (&mut self, t: f64) {
        self.tmax = t;
    }

    pub fn set_tmin (&mut self, t: f64) {
        self.tmin = t;
    }
}