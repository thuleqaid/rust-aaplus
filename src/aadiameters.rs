use super::aacoordinatetransformation;
use super::aaglobe;

pub fn sun_semidiameter_a(delta: f64) -> f64
{
    959.63 / delta
}

pub fn mercury_semidiameter_a(delta: f64) -> f64
{
    3.34 / delta
}

pub fn venus_semidiameter_a(delta: f64) -> f64
{
    8.41 / delta
}

pub fn mars_semidiameter_a(delta: f64) -> f64
{
    4.68 / delta
}

pub fn jupiter_equatorial_semidiameter_a(delta: f64) -> f64
{
    98.47 / delta
}

pub fn jupiter_polar_semidiameter_a(delta: f64) -> f64
{
    91.91 / delta
}

pub fn saturn_equatorial_semidiameter_a(delta: f64) -> f64
{
    83.33 / delta
}

pub fn saturn_polar_semidiameter_a(delta: f64) -> f64
{
    74.57 / delta
}

pub fn uranus_semidiameter_a(delta: f64) -> f64
{
    34.28 / delta
}

pub fn neptune_semidiameter_a(delta: f64) -> f64
{
    36.56 / delta
}

pub fn mercury_semidiameter_b(delta: f64) -> f64
{
    3.36 / delta
}

pub fn venus_semidiameter_b(delta: f64) -> f64
{
    8.34 / delta
}

pub fn mars_semidiameter_b(delta: f64) -> f64
{
    4.68 / delta
}

pub fn jupiter_equatorial_semidiameter_b(delta: f64) -> f64
{
    98.44 / delta
}

pub fn jupiter_polar_semidiameter_b(delta: f64) -> f64
{
    92.06 / delta
}

pub fn saturn_equatorial_semidiameter_b(delta: f64) -> f64
{
    82.73 / delta
}

pub fn saturn_polar_semidiameter_b(delta: f64) -> f64
{
    73.82 / delta
}

pub fn uranus_semidiameter_b(delta: f64) -> f64
{
    35.02 / delta
}

pub fn neptune_semidiameter_b(delta: f64) -> f64
{
    33.50 / delta
}

pub fn pluto_semidiameter_b(delta: f64) -> f64
{
    2.07 / delta
}

pub fn geocentric_moon_semidiameter(delta: f64) -> f64
{
    aacoordinatetransformation::radians_to_degrees(0.272481 * 6378.14 / delta) * 3600.0
}

pub fn apparent_asteroid_diameter(delta: f64, d: f64) -> f64
{
    0.0013788 * d / delta
}

pub fn apparent_saturn_polar_semidiameter_a(delta: f64, b: f64) -> f64
{
    let cos_b = f64::cos(aacoordinatetransformation::degrees_to_radians(b));
    saturn_polar_semidiameter_a(delta) * f64::sqrt(1.0 - 0.199197 * cos_b * cos_b)
}

pub fn apparent_saturn_polar_semidiameter_b(delta: f64, b: f64) -> f64
{
    let cos_b = f64::cos(aacoordinatetransformation::degrees_to_radians(b));
    saturn_polar_semidiameter_b(delta) * f64::sqrt(1.0 - 0.203800 * cos_b * cos_b)
}

pub fn topocentric_moon_semidiameter(distance_delta: f64, delta: f64, h: f64, latitude: f64, height: f64) -> f64
{
    //Convert to radians
    let h = aacoordinatetransformation::hours_to_radians(h);
    let delta = aacoordinatetransformation::degrees_to_radians(delta);

    let pi = f64::asin(6378.14 / distance_delta);
    let a = f64::cos(delta) * f64::sin(h);
    let b = f64::cos(delta) * f64::cos(h) - aaglobe::rho_cos_theta_prime(latitude, height) * f64::sin(pi);
    let c = f64::sin(delta) - aaglobe::rho_sin_theta_prime(latitude, height) * f64::sin(pi);
    let q = f64::sqrt(a * a + b * b + c * c);

    let s = aacoordinatetransformation::degrees_to_radians(geocentric_moon_semidiameter(distance_delta) / 3600.0);
    aacoordinatetransformation::radians_to_degrees(f64::asin(f64::sin(s) / q)) * 3600.0
}

pub fn asteroid_diameter(h: f64, a: f64) -> f64
{
    let x = 3.12 - h / 5.0 - 0.217147 * f64::ln(a);
    f64::powf(10.0, x)
}
