use super::aacoordinatetransformation;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AAEclipticalElementDetails
{
    pub i: f64,
    pub w: f64,
    pub omega: f64,
}

pub fn calculate(i0: f64, w0: f64, omega0: f64, jd0: f64, jd: f64) -> AAEclipticalElementDetails
{
    let t0 = (jd0 - 2451545.0) / 36525.0;
    let t0squared = t0 * t0;
    let t = (jd - jd0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    //Now convert to radians
    let i0rad = aacoordinatetransformation::degrees_to_radians(i0);
    let omega0rad = aacoordinatetransformation::degrees_to_radians(omega0);

    let mut eta = (47.0029 - 0.06603 * t0 + 0.000598 * t0squared) * t + (-0.03302 + 0.000598 * t0) * tsquared + 0.00006 * tcubed;
    eta = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, eta, true));

    let mut pi = 174.876384 * 3600.0 + 3289.4789 * t0 + 0.60622 * t0squared - (869.8089 + 0.50491 * t0) * t + 0.03536 * tsquared;
    pi = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, pi, true));

    let mut p = (5029.0966 + 2.22226 * t0 - 0.000042 * t0squared) * t + (1.11113 - 0.000042 * t0) * tsquared - 0.000006 * tcubed;
    p = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, p, true));

    let sini0rad = f64::sin(i0rad);
    let cosi0rad = f64::cos(i0rad);
    let sinomega0rad_pi = f64::sin(omega0rad - pi);
    let cosomega0rad_pi = f64::cos(omega0rad - pi);
    let sineta = f64::sin(eta);
    let coseta = f64::cos(eta);
    let mut a = sini0rad * sinomega0rad_pi;
    let mut b = -sineta * cosi0rad + coseta * sini0rad * cosomega0rad_pi;
    let irad = f64::asin(f64::sqrt(a * a + b * b));

    let mut details: AAEclipticalElementDetails = Default::default();

    details.i = aacoordinatetransformation::radians_to_degrees(irad);
    let cosi = cosi0rad * coseta + sini0rad * sineta * cosomega0rad_pi;
    if cosi < 0.0 {
        details.i = 180.0 - details.i;
    }

    let phi = pi + p;
    details.omega = aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(f64::atan2(a, b) + phi));

    a = -sineta * sinomega0rad_pi;
    b = sini0rad * coseta - cosi0rad * sineta * cosomega0rad_pi;
    let deltaw = aacoordinatetransformation::radians_to_degrees(f64::atan2(a, b));
    details.w = aacoordinatetransformation::map_to_0to360_range(w0 + deltaw);

    details
}

pub fn fk4b1950_to_fk5j2000(i0: f64, w0: f64, omega0: f64) -> AAEclipticalElementDetails
{
    //convert to radians
    let l = aacoordinatetransformation::degrees_to_radians(5.19856209);
    let j = aacoordinatetransformation::degrees_to_radians(0.00651966);
    let i0rad = aacoordinatetransformation::degrees_to_radians(i0);
    let omega0rad = aacoordinatetransformation::degrees_to_radians(omega0);
    let sini0rad = f64::sin(i0rad);
    let cosi0rad = f64::cos(i0rad);

    //Calculate some values used later
    let cos_j = f64::cos(j);
    let sin_j = f64::sin(j);
    let w = l + omega0rad;
    let cos_w = f64::cos(w);
    let sin_w = f64::sin(w);
    let a = sin_j * sin_w;
    let b = sini0rad * cos_j + cosi0rad * sin_j * cos_w;

    //Calculate the values
    let mut details: AAEclipticalElementDetails = Default::default();
    details.i = aacoordinatetransformation::radians_to_degrees(f64::asin(f64::sqrt(a * a + b * b)));
    let cosi = cosi0rad * cos_j - sini0rad * sin_j * cos_w;
    if cosi < 0.0 {
        details.i = 180.0 - details.i;
    }

    details.w = aacoordinatetransformation::map_to_0to360_range(w0 + aacoordinatetransformation::radians_to_degrees(f64::atan2(a, b)));
    details.omega = aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(f64::atan2(sini0rad * sin_w, cosi0rad * sin_j + sini0rad * cos_j * cos_w)) - 4.50001688);

    details
}

