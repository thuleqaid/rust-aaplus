use super::aa2dcoordinate;
use super::aa3dcoordinate;
use super::aacoordinatetransformation;
use super::aasun;
use super::aavsop87a_ear;
use super::aafk5;

pub struct AberrationCoefficient {
    pub l2: i64,
    pub l3: i64,
    pub l4: i64,
    pub l5: i64,
    pub l6: i64,
    pub l7: i64,
    pub l8: i64,
    pub ldash: i64,
    pub d: i64,
    pub mdash: i64,
    pub f: i64,
    pub xsin: i64,
    pub xsint: i64,
    pub xcos: i64,
    pub xcost: i64,
    pub ysin: i64,
    pub ysint: i64,
    pub ycos: i64,
    pub ycost: i64,
    pub zsin: i64,
    pub zsint: i64,
    pub zcos: i64,
    pub zcost: i64,
}

const G_ABERRATION_COEFFICIENTS: [AberrationCoefficient; 36] =
    [
        AberrationCoefficient { l2: 0, l3: 1, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -1719914, xsint: -2, xcos: -25, xcost: 0, ysin: 25, ysint: -13, ycos: 1578089, ycost: 156, zsin: 10, zsint: 32, zcos: 684185, zcost: -358 },
        AberrationCoefficient { l2: 0, l3: 2, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 6434, xsint: 141, xcos: 28007, xcost: -107, ysin: 25697, ysint: -95, ycos: -5904, ycost: -130, zsin: 11141, zsint: -48, zcos: -2559, zcost: -55 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 1, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 715, xsint: 0, xcos: 0, xcost: 0, ysin: 6, ysint: 0, ycos: -657, ycost: 0, zsin: -15, zsint: 0, zcos: -282, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 1, d: 0, mdash: 0, f: 0, xsin: 715, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -656, ycost: 0, zsin: 0, zsint: 0, zcos: -285, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 3, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 486, xsint: -5, xcos: -236, xcost: -4, ysin: -216, ysint: -4, ycos: -446, ycost: 5, zsin: -94, zsint: 0, zcos: -193, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 1, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 159, xsint: 0, xcos: 0, xcost: 0, ysin: 2, ysint: 0, ycos: -147, ycost: 0, zsin: -6, zsint: 0, zcos: -61, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 1, xsin: 0, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: 26, ycost: 0, zsin: 0, zsint: 0, zcos: -59, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 1, d: 0, mdash: 1, f: 0, xsin: 39, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -36, ycost: 0, zsin: 0, zsint: 0, zcos: -16, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 2, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 33, xsint: 0, xcos: -10, xcost: 0, ysin: -9, ysint: 0, ycos: -30, ycost: 0, zsin: -5, zsint: 0, zcos: -13, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 2, l4: 0, l5: -1, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 31, xsint: 0, xcos: 1, xcost: 0, ysin: 1, ysint: 0, ycos: -28, ycost: 0, zsin: 0, zsint: 0, zcos: -12, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 3, l4: -8, l5: 3, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 8, xsint: 0, xcos: -28, xcost: 0, ysin: 25, ysint: 0, ycos: 8, ycost: 0, zsin: 11, zsint: 0, zcos: 3, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 5, l4: -8, l5: 3, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 8, xsint: 0, xcos: -28, xcost: 0, ysin: -25, ysint: 0, ycos: -8, ycost: 0, zsin: -11, zsint: 0, zcos: -3, zcost: 0 },
        AberrationCoefficient { l2: 2, l3: -1, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 21, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -19, ycost: 0, zsin: 0, zsint: 0, zcos: -8, zcost: 0 },
        AberrationCoefficient { l2: 1, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -19, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: 17, ycost: 0, zsin: 0, zsint: 0, zcos: 8, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 1, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 17, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -16, ycost: 0, zsin: 0, zsint: 0, zcos: -7, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 1, l4: 0, l5: -2, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 16, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: 15, ycost: 0, zsin: 1, zsint: 0, zcos: 7, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 1, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 16, xsint: 0, xcos: 0, xcost: 0, ysin: 1, ysint: 0, ycos: -15, ycost: 0, zsin: -3, zsint: 0, zcos: -6, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 1, l4: 0, l5: 1, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 11, xsint: 0, xcos: -1, xcost: 0, ysin: -1, ysint: 0, ycos: -10, ycost: 0, zsin: -1, zsint: 0, zcos: -5, zcost: 0 },
        AberrationCoefficient { l2: 2, l3: -2, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 0, xsint: 0, xcos: -11, xcost: 0, ysin: -10, ysint: 0, ycos: 0, ycost: 0, zsin: -4, zsint: 0, zcos: 0, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 1, l4: 0, l5: -1, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -11, xsint: 0, xcos: -2, xcost: 0, ysin: -2, ysint: 0, ycos: 9, ycost: 0, zsin: -1, zsint: 0, zcos: 4, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 4, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -7, xsint: 0, xcos: -8, xcost: 0, ysin: -8, ysint: 0, ycos: 6, ycost: 0, zsin: -3, zsint: 0, zcos: 3, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 3, l4: 0, l5: -2, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -10, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: 9, ycost: 0, zsin: 0, zsint: 0, zcos: 4, zcost: 0 },
        AberrationCoefficient { l2: 1, l3: -2, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -9, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -9, ycost: 0, zsin: 0, zsint: 0, zcos: -4, zcost: 0 },
        AberrationCoefficient { l2: 2, l3: -3, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -9, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -8, ycost: 0, zsin: 0, zsint: 0, zcos: -4, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 2, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 0, xsint: 0, xcos: -9, xcost: 0, ysin: -8, ysint: 0, ycos: 0, ycost: 0, zsin: -3, zsint: 0, zcos: 0, zcost: 0 },
        AberrationCoefficient { l2: 2, l3: -4, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 0, xsint: 0, xcos: -9, xcost: 0, ysin: 8, ysint: 0, ycos: 0, ycost: 0, zsin: 3, zsint: 0, zcos: 0, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 3, l4: -2, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 8, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -8, ycost: 0, zsin: 0, zsint: 0, zcos: -3, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 1, d: 2, mdash: -1, f: 0, xsin: 8, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -7, ycost: 0, zsin: 0, zsint: 0, zcos: -3, zcost: 0 },
        AberrationCoefficient { l2: 8, l3: -12, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -4, xsint: 0, xcos: -7, xcost: 0, ysin: -6, ysint: 0, ycos: 4, ycost: 0, zsin: -3, zsint: 0, zcos: 2, zcost: 0 },
        AberrationCoefficient { l2: 8, l3: -14, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -4, xsint: 0, xcos: -7, xcost: 0, ysin: 6, ysint: 0, ycos: -4, ycost: 0, zsin: 3, zsint: 0, zcos: -2, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 2, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -6, xsint: 0, xcos: -5, xcost: 0, ysin: -4, ysint: 0, ycos: 5, ycost: 0, zsin: -2, zsint: 0, zcos: 2, zcost: 0 },
        AberrationCoefficient { l2: 3, l3: -4, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: -1, xsint: 0, xcos: -1, xcost: 0, ysin: -2, ysint: 0, ycos: -7, ycost: 0, zsin: 1, zsint: 0, zcos: -4, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 2, l4: 0, l5: -2, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 4, xsint: 0, xcos: -6, xcost: 0, ysin: -5, ysint: 0, ycos: -4, ycost: 0, zsin: -2, zsint: 0, zcos: -2, zcost: 0 },
        AberrationCoefficient { l2: 3, l3: -3, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 0, xsint: 0, xcos: -7, xcost: 0, ysin: -6, ysint: 0, ycos: 0, ycost: 0, zsin: -3, zsint: 0, zcos: 0, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 2, l4: -2, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 0, d: 0, mdash: 0, f: 0, xsin: 5, xsint: 0, xcos: -5, xcost: 0, ysin: -4, ysint: 0, ycos: -5, ycost: 0, zsin: -2, zsint: 0, zcos: -2, zcost: 0 },
        AberrationCoefficient { l2: 0, l3: 0, l4: 0, l5: 0, l6: 0, l7: 0, l8: 0, ldash: 1, d: -2, mdash: 0, f: 0, xsin: 5, xsint: 0, xcos: 0, xcost: 0, ysin: 0, ysint: 0, ycos: -5, ycost: 0, zsin: 0, zsint: 0, zcos: -2, zcost: 0 },
    ];

pub fn earth_velocity(jd: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let mut velocity: aa3dcoordinate::AA3DCoordinate = Default::default();

    if b_high_precision {
        velocity.x = aavsop87a_ear::x_dash(jd);
        velocity.y = aavsop87a_ear::y_dash(jd);
        velocity.z = aavsop87a_ear::z_dash(jd);
        velocity = aafk5::convert_vsop_to_fk5_j2000(&velocity);
        velocity.x *= 100000000.0;
        velocity.y *= 100000000.0;
        velocity.z *= 100000000.0;
    } else {
        let t = (jd - 2451545.0) / 36525.0;
        let l2 = 3.1761467 + 1021.3285546 * t;
        let l3 = 1.7534703 + 628.3075849 * t;
        let l4 = 6.2034809 + 334.0612431 * t;
        let l5 = 0.5995465 + 52.9690965 * t;
        let l6 = 0.8740168 + 21.3299095 * t;
        let l7 = 5.4812939 + 7.4781599 * t;
        let l8 = 5.3118863 + 3.8133036 * t;
        let ldash = 3.8103444 + 8399.6847337 * t;
        let d = 5.1984667 + 7771.3771486 * t;
        let mdash = 2.3555559 + 8328.6914289 * t;
        let f = 1.6279052 + 8433.4661601 * t;

        let n_aberration_coefficients = G_ABERRATION_COEFFICIENTS.len();
        for i in 0..n_aberration_coefficients {
            let argument = G_ABERRATION_COEFFICIENTS[i].l2 as f64 * l2 + G_ABERRATION_COEFFICIENTS[i].l3 as f64 * l3 +
                G_ABERRATION_COEFFICIENTS[i].l4 as f64 * l4 + G_ABERRATION_COEFFICIENTS[i].l5 as f64 * l5 +
                G_ABERRATION_COEFFICIENTS[i].l6 as f64 * l6 + G_ABERRATION_COEFFICIENTS[i].l7 as f64 * l7 +
                G_ABERRATION_COEFFICIENTS[i].l8 as f64 * l8 + G_ABERRATION_COEFFICIENTS[i].ldash as f64 * ldash +
                G_ABERRATION_COEFFICIENTS[i].d as f64 * d + G_ABERRATION_COEFFICIENTS[i].mdash as f64 * mdash +
                G_ABERRATION_COEFFICIENTS[i].f as f64 * f;
            velocity.x += (G_ABERRATION_COEFFICIENTS[i].xsin as f64 + G_ABERRATION_COEFFICIENTS[i].xsint as f64 * t) * f64::sin(argument);
            velocity.x += (G_ABERRATION_COEFFICIENTS[i].xcos as f64 + G_ABERRATION_COEFFICIENTS[i].xcost as f64 * t) * f64::cos(argument);

            velocity.y += (G_ABERRATION_COEFFICIENTS[i].ysin as f64 + G_ABERRATION_COEFFICIENTS[i].ysint as f64 * t) * f64::sin(argument);
            velocity.y += (G_ABERRATION_COEFFICIENTS[i].ycos as f64 + G_ABERRATION_COEFFICIENTS[i].ycost as f64 * t) * f64::cos(argument);

            velocity.z += (G_ABERRATION_COEFFICIENTS[i].zsin as f64 + G_ABERRATION_COEFFICIENTS[i].zsint as f64 * t) * f64::sin(argument);
            velocity.z += (G_ABERRATION_COEFFICIENTS[i].zcos as f64 + G_ABERRATION_COEFFICIENTS[i].zcost as f64 * t) * f64::cos(argument);
        }
    }
    velocity
}

pub fn equatorial_aberration(alpha: f64, delta: f64, jd: f64, b_high_precision: bool) -> aa2dcoordinate::AA2DCoordinate
{
    //Convert to radians
    let alpha2 = aacoordinatetransformation::degrees_to_radians(alpha * 15.0);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);

    let cos_alpha = f64::cos(alpha2);
    let sin_alpha = f64::sin(alpha2);
    let cos_delta = f64::cos(delta2);
    let sin_delta = f64::sin(delta2);

    let velocity = earth_velocity(jd, b_high_precision);

    //What is the return value
    let mut aberration: aa2dcoordinate::AA2DCoordinate = Default::default();

    aberration.x = aacoordinatetransformation::radians_to_hours((velocity.y * cos_alpha - velocity.x * sin_alpha) / (17314463350.0 * cos_delta));
    aberration.y = aacoordinatetransformation::radians_to_degrees(-(((velocity.x * cos_alpha + velocity.y * sin_alpha) * sin_delta - velocity.z * cos_delta) / 17314463350.0));

    aberration
}

pub fn ecliptic_aberration(lambda: f64, beta: f64, jd: f64, b_high_precision: bool) -> aa2dcoordinate::AA2DCoordinate
{
    //What is the return value
    let mut aberration: aa2dcoordinate::AA2DCoordinate = Default::default();

    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let e = 0.016708634 - 0.000042037 * t - 0.0000001267 * tsquared;
    let mut pi = 102.93735 + 1.71946 * t + 0.00046 * tsquared;
    let k = 20.49552;
    let mut sun_longitude = aasun::geometric_ecliptic_longitude(jd, b_high_precision);

    //Convert to radians
    pi = aacoordinatetransformation::degrees_to_radians(pi);
    let lambda2 = aacoordinatetransformation::degrees_to_radians(lambda);
    let beta2 = aacoordinatetransformation::degrees_to_radians(beta);
    sun_longitude = aacoordinatetransformation::degrees_to_radians(sun_longitude);

    aberration.x = (-k * f64::cos(sun_longitude - lambda2) + e * k * f64::cos(pi - lambda2)) / f64::cos(beta2) / 3600.0;
    aberration.y = -k * f64::sin(beta2) * (f64::sin(sun_longitude - lambda2) - e * f64::sin(pi - lambda2)) / 3600.0;

    aberration
}
