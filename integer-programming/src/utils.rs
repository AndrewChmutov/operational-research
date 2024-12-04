const EPSILON: f64 = 0.0000000001;

pub fn is_float(value: f64) -> bool {
    (value - value.floor()).abs() > EPSILON
}

pub fn is_zero(value: f64) -> bool {
    value.abs() < EPSILON
}
