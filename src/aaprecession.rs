use super::aa2dcoordinate;
use super::aacoordinatetransformation;

pub fn adjust_position_using_uniform_proper_motion(t: f64, alpha: f64, delta: f64, pmalpha: f64, pmdelta: f64) -> aa2dcoordinate::AA2DCoordinate
{
    aa2dcoordinate::AA2DCoordinate {
        x: aacoordinatetransformation::map_to_0to24_range(alpha + (pmalpha * t / 3600.0)),
        y: aacoordinatetransformation::map_to_minus90to90_range(delta + (pmdelta * t / 3600.0)),
    }
}

pub fn adjust_position_using_motion_in_space(r: f64, delta_r: f64, t: f64, alpha: f64, delta: f64, pmalpha: f64, pmdelta: f64) -> aa2dcoordinate::AA2DCoordinate
{
    //Convert delta_r from km/s to Parsecs / Year
    let delta_r2 = delta_r / 977792.0;

    //Convert from seconds of time to Radians / Year
    let pmalpha2 = pmalpha / 13751.0;

    //Convert from seconds of arc to Radians / Year
    let pmdelta2 = pmdelta / 206265.0;

    //Now convert to radians
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);

    let mut x = r * f64::cos(delta2) * f64::cos(alpha2);
    let mut y = r * f64::cos(delta2) * f64::sin(alpha2);
    let mut z = r * f64::sin(delta2);

    let delta_x = x / r * delta_r2 - z * pmdelta2 * f64::cos(alpha2) - y * pmalpha2;
    let delta_y = y / r * delta_r2 - z * pmdelta2 * f64::sin(alpha2) + x * pmalpha2;
    let delta_z = z / r * delta_r2 + r * pmdelta2 * f64::cos(delta2);

    x += t * delta_x;
    y += t * delta_y;
    z += t * delta_z;

    aa2dcoordinate::AA2DCoordinate {
        x: aacoordinatetransformation::map_to_0to24_range(aacoordinatetransformation::radians_to_hours(f64::atan2(y, x))),
        y: aacoordinatetransformation::radians_to_degrees(f64::atan2(z, f64::sqrt(x * x + y * y))),
    }
}

pub fn precess_equatorial(alpha: f64, delta: f64, jd0: f64, jd: f64) -> aa2dcoordinate::AA2DCoordinate
{
    let t0 = (jd0 - 2451545.0) / 36525.0;
    let t0squared = t0 * t0;
    let t = (jd - jd0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    //Now convert to radians
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);

    let sigma = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (2306.2181 + 1.39656 * t0 - 0.000139 * t0squared) * t + (0.30188 - 0.000344 * t0) * tsquared + 0.017998 * tcubed, true));
    let zeta = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (2306.2181 + 1.39656 * t0 - 0.000139 * t0squared) * t + (1.09468 + 0.000066 * t0) * tsquared + 0.018203 * tcubed, true));
    let phi = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (2004.3109 - 0.8533 * t0 - 0.000217 * t0squared) * t - (0.42665 + 0.000217 * t0) * tsquared - 0.041833 * tcubed, true));
    let a = f64::cos(delta2) * f64::sin(alpha2 + sigma);
    let b = f64::cos(phi) * f64::cos(delta2) * f64::cos(alpha2 + sigma) - f64::sin(phi) * f64::sin(delta2);
    let c = f64::sin(phi) * f64::cos(delta2) * f64::cos(alpha2 + sigma) + f64::cos(phi) * f64::sin(delta2);

    aa2dcoordinate::AA2DCoordinate {
        x: aacoordinatetransformation::map_to_0to24_range(aacoordinatetransformation::radians_to_hours(f64::atan2(a, b) + zeta)),
        y: aacoordinatetransformation::radians_to_degrees(f64::asin(c)),
    }
}

pub fn precess_equatorial_fk4(alpha: f64, delta: f64, jd0: f64, jd: f64) -> aa2dcoordinate::AA2DCoordinate
{
    let t0 = (jd0 - 2415020.3135) / 36524.2199;
    let t = (jd - jd0) / 36524.2199;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    //Now convert to radians
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);

    let sigma = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (2304.250 + 1.396 * t0) * t + 0.302 * tsquared + 0.018 * tcubed, true));
    let zeta = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 0.791 * tsquared + 0.001 * tcubed, true)) + sigma;
    let phi = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (2004.682 - 0.853 * t0) * t - 0.426 * tsquared - 0.042 * tcubed, true));
    let a = f64::cos(delta2) * f64::sin(alpha2 + sigma);
    let b = f64::cos(phi) * f64::cos(delta2) * f64::cos(alpha2 + sigma) - f64::sin(phi) * f64::sin(delta2);
    let c = f64::sin(phi) * f64::cos(delta2) * f64::cos(alpha2 + sigma) + f64::cos(phi) * f64::sin(delta2);

    let delta_alpha = aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 0.0775 + 0.0850 * t0, true);
    aa2dcoordinate::AA2DCoordinate {
        x: aacoordinatetransformation::map_to_0to24_range(aacoordinatetransformation::radians_to_hours(f64::atan2(a, b) + zeta) + delta_alpha),
        y: aacoordinatetransformation::radians_to_degrees(f64::asin(c)),
    }
}

pub fn precess_ecliptic(lambda: f64, beta: f64, jd0: f64, jd: f64) -> aa2dcoordinate::AA2DCoordinate
{
    let t0 = (jd0 - 2451545.0) / 36525.0;
    let t0squared = t0 * t0;
    let t = (jd - jd0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    //Now convert to radians
    let lambda2 = aacoordinatetransformation::degrees_to_radians(lambda);
    let beta2 = aacoordinatetransformation::degrees_to_radians(beta);

    let eta = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (47.0029 - 0.06603 * t0 + 0.000598 * t0squared) * t + (-0.03302 + 0.000598 * t0) * tsquared + 0.00006 * tcubed, true));
    let pi = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 174.876384 * 3600.0 + 3289.4789 * t0 + 0.60622 * t0squared - (869.8089 + 0.50491 * t0) * t + 0.03536 * tsquared, true));
    let p = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, (5029.0966 + 2.22226 * t0 - 0.000042 * t0squared) * t + (1.11113 - 0.000042 * t0) * tsquared - 0.000006 * tcubed, true));
    let a = f64::cos(eta) * f64::cos(beta2) * f64::sin(pi - lambda2) - f64::sin(eta) * f64::sin(beta2);
    let b = f64::cos(beta2) * f64::cos(pi - lambda2);
    let c = f64::cos(eta) * f64::sin(beta2) + f64::sin(eta) * f64::cos(beta2) * f64::sin(pi - lambda2);

    aa2dcoordinate::AA2DCoordinate {
        x: aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(p + pi - f64::atan2(a, b))),
        y: aacoordinatetransformation::radians_to_degrees(f64::asin(c)),
    }
}

pub fn equatorial_pmto_ecliptic(alpha: f64, delta: f64, beta: f64, pmalpha: f64, pmdelta: f64, epsilon: f64) -> aa2dcoordinate::AA2DCoordinate
{
    //Convert to radians
    let epsilon2 = aacoordinatetransformation::degrees_to_radians(epsilon);
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);
    let beta2 = aacoordinatetransformation::degrees_to_radians(beta);

    let cosb = f64::cos(beta2);
    let sin_epsilon = f64::sin(epsilon2);

    aa2dcoordinate::AA2DCoordinate {
        x: (pmdelta * sin_epsilon * f64::cos(alpha2) + pmalpha * f64::cos(delta2) * (f64::cos(epsilon2) * f64::cos(delta2) + sin_epsilon * f64::sin(delta2) * f64::sin(alpha2))) / (cosb * cosb),
        y: (pmdelta * (f64::cos(epsilon2) * f64::cos(delta2) + sin_epsilon * f64::sin(delta2) * f64::sin(alpha2)) - pmalpha * sin_epsilon * f64::cos(alpha2) * f64::cos(delta2)) / cosb,
    }
}
