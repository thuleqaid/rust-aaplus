use super::aacoordinatetransformation;

pub fn calculate(m: f64, e: f64, n_iterations: i64) -> f64
{
    //Convert from degrees to radians
    let mut m = aacoordinatetransformation::degrees_to_radians(m);
    let pi = aacoordinatetransformation::pi();

    let mut f = 1;
    if m < 0.0 {
        f = -1;
    }
    m = f64::abs(m) / (2.0 * pi);
    m = (m - f64::trunc(m)) * 2.0 * pi * f as f64;
    if m < 0.0 {
        m += 2.0 * pi;
    }
    f = 1;
    if m > pi {
        f = -1;
    }
    if m > pi {
        m = 2.0 * pi - m;
    }

    let mut e2 = pi / 2.0;
    let mut scale = pi / 4.0;
    for _i in 0..n_iterations {
        let r = e2 - e * f64::sin(e2);
        if m > r {
            e2 += scale;
        } else {
            e2 -= scale;
        }
        scale /= 2.0;
    }

    //Convert the result back to degrees
    aacoordinatetransformation::radians_to_degrees(e2) * f as f64
}
