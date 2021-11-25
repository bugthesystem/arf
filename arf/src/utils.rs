// const PI: f64 = 3.14159265358979323846;
use std::f64::consts::PI;

const DEG2RAD: f64 = PI / 180.0;
const RAD2DEG: f64 = 180.0 / PI;

/// `clamp` float value
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    let mut result = if value < min { min } else { value };

    if result > max {
        result = max;
    }

    return result;
}

/// `lerp` calculates linear interpolation between two floats
pub fn lerp(start: f64, end: f64, amount: f64) -> f64 {
    let result = start + amount * (end - start);

    return result;
}

/// `normalize` input value within input range
pub fn normalize(value: f64, start: f64, end: f64) -> f64 {
    let result = (value - start) / (end - start);

    return result;
}

/// `remap` input value within input range to output range
pub fn remap(value: f64, input_start: f64, input_end: f64, output_start: f64, output_end: f64) -> f64 {
    let result = (value - input_start) / (input_end - input_start)
        * (output_end - output_start) + output_start;

    return result;
}