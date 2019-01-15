use super::aakepler;
use super::aacoordinatetransformation;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AABinaryStarDetails
{
    pub r: f64,
    pub theta: f64,
    pub rho: f64,
}

pub fn calculate(t: f64, p: f64, t2: f64, e: f64, a: f64, i: f64, omega: f64, w: f64) -> AABinaryStarDetails
{
    let n = 360.0 / p;
    let m = aacoordinatetransformation::map_to_0to360_range(n * (t - t2));
    let mut e_local = aakepler::calculate(m, e, 53);
    e_local = aacoordinatetransformation::degrees_to_radians(e_local);
    let i = aacoordinatetransformation::degrees_to_radians(i);
    let w = aacoordinatetransformation::degrees_to_radians(w);
    let omega = aacoordinatetransformation::degrees_to_radians(omega);

    let mut details: AABinaryStarDetails = Default::default();

    details.r = a * (1.0 - e * f64::cos(e_local));

    let v = f64::atan(f64::sqrt((1.0 + e) / (1.0 - e)) * f64::tan(e_local / 2.0)) * 2.0;
    details.theta = f64::atan2(f64::sin(v + w) * f64::cos(i), f64::cos(v + w)) + omega;
    details.theta = aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(details.theta));

    let sinvw = f64::sin(v + w);
    let cosvw = f64::cos(v + w);
    let cosi = f64::cos(i);
    details.rho = details.r * f64::sqrt((sinvw * sinvw * cosi * cosi) + (cosvw * cosvw));

    details
}

pub fn apparent_eccentricity(e: f64, i: f64, w: f64) -> f64
{
    let i = aacoordinatetransformation::degrees_to_radians(i);
    let w = aacoordinatetransformation::degrees_to_radians(w);

    let cosi = f64::cos(i);
    let cosw = f64::cos(w);
    let sinw = f64::sin(w);
    let esquared = e * e;
    let a = (1.0 - esquared * cosw * cosw) * cosi * cosi;
    let b = esquared * sinw * cosw * cosi;
    let c = 1.0 - esquared * sinw * sinw;
    let d = (a - c) * (a - c) + 4.0 * b * b;

    let sqrt_d = f64::sqrt(d);
    f64::sqrt(2.0 * sqrt_d / (a + c + sqrt_d))
}
