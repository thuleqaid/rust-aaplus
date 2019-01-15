use super::aacoordinatetransformation;

pub fn separation(alpha1: f64, delta1: f64, alpha2: f64, delta2: f64) -> f64
{
    let delta1 = aacoordinatetransformation::degrees_to_radians(delta1);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta2);

    let alpha1 = aacoordinatetransformation::hours_to_radians(alpha1);
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha2);

    let x = f64::cos(delta1) * f64::sin(delta2) - f64::sin(delta1) * f64::cos(delta2) * f64::cos(alpha2 - alpha1);
    let y = f64::cos(delta2) * f64::sin(alpha2 - alpha1);
    let z = f64::sin(delta1) * f64::sin(delta2) + f64::cos(delta1) * f64::cos(delta2) * f64::cos(alpha2 - alpha1);

    let mut value = f64::atan2(f64::sqrt(x * x + y * y), z);
    value = aacoordinatetransformation::radians_to_degrees(value);
    if value < 0.0 {
        value += 180.0;
    }

    value
}

pub fn position_angle(alpha1: f64, delta1: f64, alpha2: f64, delta2: f64) -> f64
{
    let delta1 = aacoordinatetransformation::degrees_to_radians(delta1);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta2);

    let alpha1 = aacoordinatetransformation::hours_to_radians(alpha1);
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha2);

    let delta_alpha = alpha1 - alpha2;
    let mut value = f64::atan2(f64::sin(delta_alpha), f64::cos(delta2) * f64::tan(delta1) - f64::sin(delta2) * f64::cos(delta_alpha));
    value = aacoordinatetransformation::radians_to_degrees(value);
    if value < 0.0 {
        value += 180.0;
    }

    value
}

pub fn distance_from_great_arc(alpha1: f64, delta1: f64, alpha2: f64, delta2: f64, alpha3: f64, delta3: f64) -> f64
{
    let delta1 = aacoordinatetransformation::degrees_to_radians(delta1);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta2);
    let delta3 = aacoordinatetransformation::degrees_to_radians(delta3);

    let alpha1 = aacoordinatetransformation::hours_to_radians(alpha1);
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha2);
    let alpha3 = aacoordinatetransformation::hours_to_radians(alpha3);

    let x1 = f64::cos(delta1) * f64::cos(alpha1);
    let x2 = f64::cos(delta2) * f64::cos(alpha2);

    let y1 = f64::cos(delta1) * f64::sin(alpha1);
    let y2 = f64::cos(delta2) * f64::sin(alpha2);

    let z1 = f64::sin(delta1);
    let z2 = f64::sin(delta2);

    let a = y1 * z2 - z1 * y2;
    let b = z1 * x2 - x1 * z2;
    let c = x1 * y2 - y1 * x2;

    let m = f64::tan(alpha3);
    let n = f64::tan(delta3) / f64::cos(alpha3);

    let mut value = f64::asin((a + b * m + c * n) / (f64::sqrt(a * a + b * b + c * c) * f64::sqrt(1.0 + m * m + n * n)));
    value = aacoordinatetransformation::radians_to_degrees(value);
    if value < 0.0 {
        value = f64::abs(value);
    }

    value
}

pub fn smallest_circle(alpha1: f64, delta1: f64, alpha2: f64, delta2: f64, alpha3: f64, delta3: f64, b_type1: &mut bool) -> f64
{
    let d1 = separation(alpha1, delta1, alpha2, delta2);
    let d2 = separation(alpha1, delta1, alpha3, delta3);
    let d3 = separation(alpha2, delta2, alpha3, delta3);

    let mut a = d1;
    let mut b = d2;
    let mut c = d3;
    if b > a {
        a = d2;
        b = d1;
        c = d3;
    }
    if c > a {
        a = d3;
        b = d1;
        c = d2;
    }

    let value:f64;
    if a > f64::sqrt(b * b + c * c) {
        *b_type1 = true;
        value = a;
    } else {
        *b_type1 = false;
        value = 2.0 * a * b * c / (f64::sqrt((a + b + c) * (a + b - c) * (b + c - a) * (a + c - b)));
    }

    value
}
