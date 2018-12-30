use super::aacoordinatetransformation;

struct NutationCoefficient
{
    d: i64,
    m: i64,
    mprime: i64,
    f: i64,
    omega: i64,
    sincoeff1: i64,
    sincoeff2: f64,
    coscoeff1: i64,
    coscoeff2: f64,
}

const G_NUTATION_COEFFICIENTS: [NutationCoefficient; 63] =
    [
        NutationCoefficient { d: 0, m: 0, mprime: 0, f: 0, omega: 1, sincoeff1: -171996, sincoeff2: -174.2, coscoeff1: 92025, coscoeff2: 8.9 },
        NutationCoefficient { d: -2, m: 0, mprime: 0, f: 2, omega: 2, sincoeff1: -13187, sincoeff2: -1.6, coscoeff1: 5736, coscoeff2: -3.1 },
        NutationCoefficient { d: 0, m: 0, mprime: 0, f: 2, omega: 2, sincoeff1: -2274, sincoeff2: -0.2, coscoeff1: 977, coscoeff2: -0.5 },
        NutationCoefficient { d: 0, m: 0, mprime: 0, f: 0, omega: 2, sincoeff1: 2062, sincoeff2: 0.2, coscoeff1: -895, coscoeff2: 0.5 },
        NutationCoefficient { d: 0, m: 1, mprime: 0, f: 0, omega: 0, sincoeff1: 1426, sincoeff2: -3.4, coscoeff1: 54, coscoeff2: -0.1 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: 0, omega: 0, sincoeff1: 712, sincoeff2: 0.1, coscoeff1: -7, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 1, mprime: 0, f: 2, omega: 2, sincoeff1: -517, sincoeff2: 1.2, coscoeff1: 224, coscoeff2: -0.6 },
        NutationCoefficient { d: 0, m: 0, mprime: 0, f: 2, omega: 1, sincoeff1: -386, sincoeff2: -0.4, coscoeff1: 200, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: 2, omega: 2, sincoeff1: -301, sincoeff2: 0.0, coscoeff1: 129, coscoeff2: -0.1 },
        NutationCoefficient { d: -2, m: -1, mprime: 0, f: 2, omega: 2, sincoeff1: 217, sincoeff2: -0.5, coscoeff1: -95, coscoeff2: 0.3 },
        NutationCoefficient { d: -2, m: 0, mprime: 1, f: 0, omega: 0, sincoeff1: -158, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 0, f: 2, omega: 1, sincoeff1: 129, sincoeff2: 0.1, coscoeff1: -70, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: -1, f: 2, omega: 2, sincoeff1: 123, sincoeff2: 0.0, coscoeff1: -53, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 0, f: 0, omega: 0, sincoeff1: 63, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: 0, omega: 1, sincoeff1: 63, sincoeff2: 0.1, coscoeff1: -33, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: -1, f: 2, omega: 2, sincoeff1: -59, sincoeff2: 0.0, coscoeff1: 26, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: -1, f: 0, omega: 1, sincoeff1: -58, sincoeff2: -0.1, coscoeff1: 32, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: 2, omega: 1, sincoeff1: -51, sincoeff2: 0.0, coscoeff1: 27, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 2, f: 0, omega: 0, sincoeff1: 48, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: -2, f: 2, omega: 1, sincoeff1: 46, sincoeff2: 0.0, coscoeff1: -24, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 0, f: 2, omega: 2, sincoeff1: -38, sincoeff2: 0.0, coscoeff1: 16, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 2, f: 2, omega: 2, sincoeff1: -31, sincoeff2: 0.0, coscoeff1: 13, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 2, f: 0, omega: 0, sincoeff1: 29, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 1, f: 2, omega: 2, sincoeff1: 29, sincoeff2: 0.0, coscoeff1: -12, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 0, f: 2, omega: 0, sincoeff1: 26, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 0, f: 2, omega: 0, sincoeff1: -22, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: -1, f: 2, omega: 1, sincoeff1: 21, sincoeff2: 0.0, coscoeff1: -10, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 2, mprime: 0, f: 0, omega: 0, sincoeff1: 17, sincoeff2: -0.1, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: -1, f: 0, omega: 1, sincoeff1: 16, sincoeff2: 0.0, coscoeff1: -8, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 2, mprime: 0, f: 2, omega: 2, sincoeff1: -16, sincoeff2: 0.1, coscoeff1: 7, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 1, mprime: 0, f: 0, omega: 1, sincoeff1: -15, sincoeff2: 0.0, coscoeff1: 9, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 1, f: 0, omega: 1, sincoeff1: -13, sincoeff2: 0.0, coscoeff1: 7, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: -1, mprime: 0, f: 0, omega: 1, sincoeff1: -12, sincoeff2: 0.0, coscoeff1: 6, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 2, f: -2, omega: 0, sincoeff1: 11, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: -1, f: 2, omega: 1, sincoeff1: -10, sincoeff2: 0.0, coscoeff1: 5, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 1, f: 2, omega: 2, sincoeff1: -8, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 1, mprime: 0, f: 2, omega: 2, sincoeff1: 7, sincoeff2: 0.0, coscoeff1: -3, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 1, mprime: 1, f: 0, omega: 0, sincoeff1: -7, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: -1, mprime: 0, f: 2, omega: 2, sincoeff1: -7, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 0, f: 2, omega: 1, sincoeff1: -7, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 1, f: 0, omega: 0, sincoeff1: 6, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 2, f: 2, omega: 2, sincoeff1: 6, sincoeff2: 0.0, coscoeff1: -3, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 1, f: 2, omega: 1, sincoeff1: 6, sincoeff2: 0.0, coscoeff1: -3, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: -2, f: 0, omega: 1, sincoeff1: -6, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: 0, mprime: 0, f: 0, omega: 1, sincoeff1: -6, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: -1, mprime: 1, f: 0, omega: 0, sincoeff1: 5, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: -1, mprime: 0, f: 2, omega: 1, sincoeff1: -5, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 0, f: 0, omega: 1, sincoeff1: -5, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 2, f: 2, omega: 1, sincoeff1: -5, sincoeff2: 0.0, coscoeff1: 3, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 0, mprime: 2, f: 0, omega: 1, sincoeff1: 4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 1, mprime: 0, f: 2, omega: 1, sincoeff1: 4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: -2, omega: 0, sincoeff1: 4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -1, m: 0, mprime: 1, f: 0, omega: 0, sincoeff1: -4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -2, m: 1, mprime: 0, f: 0, omega: 0, sincoeff1: -4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 1, m: 0, mprime: 0, f: 0, omega: 0, sincoeff1: -4, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 1, f: 2, omega: 0, sincoeff1: 3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: -2, f: 2, omega: 2, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: -1, m: -1, mprime: 1, f: 0, omega: 0, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 1, mprime: 1, f: 0, omega: 0, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: -1, mprime: 1, f: 2, omega: 2, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: -1, mprime: -1, f: 2, omega: 2, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 0, m: 0, mprime: 3, f: 2, omega: 2, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
        NutationCoefficient { d: 2, m: -1, mprime: 0, f: 2, omega: 2, sincoeff1: -3, sincoeff2: 0.0, coscoeff1: 0, coscoeff2: 0.0 },
    ];

pub fn nutation_in_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    let mut d = 297.85036 + 445267.111480 * t - 0.0019142 * tsquared + tcubed / 189474.0;
    d = aacoordinatetransformation::map_to_0to360_range(d);

    let mut m = 357.52772 + 35999.050340 * t - 0.0001603 * tsquared - tcubed / 300000.0;
    m = aacoordinatetransformation::map_to_0to360_range(m);

    let mut mprime = 134.96298 + 477198.867398 * t + 0.0086972 * tsquared + tcubed / 56250.0;
    mprime = aacoordinatetransformation::map_to_0to360_range(mprime);

    let mut f = 93.27191 + 483202.017538 * t - 0.0036825 * tsquared + tcubed / 327270.0;
    f = aacoordinatetransformation::map_to_0to360_range(f);

    let mut omega = 125.04452 - 1934.136261 * t + 0.0020708 * tsquared + tcubed / 450000.0;
    omega = aacoordinatetransformation::map_to_0to360_range(omega);

    let n_coefficients = G_NUTATION_COEFFICIENTS.len();
    let mut value = 0.0;
    for i in 0..n_coefficients
        {
            let argument = G_NUTATION_COEFFICIENTS[i].d as f64 * d + G_NUTATION_COEFFICIENTS[i].m as f64 * m +
                G_NUTATION_COEFFICIENTS[i].mprime as f64 * mprime + G_NUTATION_COEFFICIENTS[i].f as f64 * f +
                G_NUTATION_COEFFICIENTS[i].omega as f64 * omega;
            let radargument = aacoordinatetransformation::degrees_to_radians(argument);
            value += (G_NUTATION_COEFFICIENTS[i].sincoeff1 as f64 + G_NUTATION_COEFFICIENTS[i].sincoeff2 * t) * f64::sin(radargument) * 0.0001;
        }

    value
}

pub fn nutation_in_obliquity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    let mut d = 297.85036 + 445267.111480 * t - 0.0019142 * tsquared + tcubed / 189474.0;
    d = aacoordinatetransformation::map_to_0to360_range(d);

    let mut m = 357.52772 + 35999.050340 * t - 0.0001603 * tsquared - tcubed / 300000.0;
    m = aacoordinatetransformation::map_to_0to360_range(m);

    let mut mprime = 134.96298 + 477198.867398 * t + 0.0086972 * tsquared + tcubed / 56250.0;
    mprime = aacoordinatetransformation::map_to_0to360_range(mprime);

    let mut f = 93.27191 + 483202.017538 * t - 0.0036825 * tsquared + tcubed / 327270.0;
    f = aacoordinatetransformation::map_to_0to360_range(f);

    let mut omega = 125.04452 - 1934.136261 * t + 0.0020708 * tsquared + tcubed / 450000.0;
    omega = aacoordinatetransformation::map_to_0to360_range(omega);

    let n_coefficients = G_NUTATION_COEFFICIENTS.len();
    let mut value = 0.0;
    for i in 0..n_coefficients
        {
            let argument = G_NUTATION_COEFFICIENTS[i].d as f64 * d + G_NUTATION_COEFFICIENTS[i].m as f64 * m +
                G_NUTATION_COEFFICIENTS[i].mprime as f64 * mprime + G_NUTATION_COEFFICIENTS[i].f as f64 * f +
                G_NUTATION_COEFFICIENTS[i].omega as f64 * omega;
            let radargument = aacoordinatetransformation::degrees_to_radians(argument);
            value += (G_NUTATION_COEFFICIENTS[i].coscoeff1 as f64 + G_NUTATION_COEFFICIENTS[i].coscoeff2 * t) * f64::cos(radargument) * 0.0001;
        }

    value
}

pub fn mean_obliquity_of_ecliptic(jd: f64) -> f64
{
    let u = (jd - 2451545.0) / 3652500.0;
    let usquared = u * u;
    let ucubed = usquared * u;
    let u_4 = ucubed * u;
    let u_5 = u_4 * u;
    let u_6 = u_5 * u;
    let u_7 = u_6 * u;
    let u_8 = u_7 * u;
    let u_9 = u_8 * u;
    let u_10 = u_9 * u;

    aacoordinatetransformation::dms_to_degrees(23.0, 26.0, 21.448, true) - aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 4680.93, true) * u
        - aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 1.55, true) * usquared
        + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 1999.25, true) * ucubed
        - aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 51.38, true) * u_4
        - aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 249.67, true) * u_5
        - aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 39.05, true) * u_6
        + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 7.12, true) * u_7
        + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 27.87, true) * u_8
        + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 5.79, true) * u_9
        + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 2.45, true) * u_10
}

pub fn true_obliquity_of_ecliptic(jd: f64) -> f64
{
    mean_obliquity_of_ecliptic(jd) + aacoordinatetransformation::dms_to_degrees(0.0, 0.0, nutation_in_obliquity(jd), true)
}

pub fn nutation_in_right_ascension(alpha: f64, delta: f64, obliquity: f64, nutation_in_longitude: f64, nutation_in_obliquity: f64) -> f64
{
    //Convert to radians
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let delta2 = aacoordinatetransformation::degrees_to_radians(delta);
    let obliquity2 = aacoordinatetransformation::degrees_to_radians(obliquity);

    (f64::cos(obliquity2) + f64::sin(obliquity2) * f64::sin(alpha2) * f64::tan(delta2)) * nutation_in_longitude - f64::cos(alpha2) * f64::tan(delta2) * nutation_in_obliquity
}

pub fn nutation_in_declination(alpha: f64, obliquity: f64, nutation_in_longitude: f64, nutation_in_obliquity: f64) -> f64
{
    //Convert to radians
    let alpha2 = aacoordinatetransformation::hours_to_radians(alpha);
    let obliquity2 = aacoordinatetransformation::degrees_to_radians(obliquity);

    f64::sin(obliquity2) * f64::cos(alpha2) * nutation_in_longitude + f64::sin(alpha2) * nutation_in_obliquity
}
