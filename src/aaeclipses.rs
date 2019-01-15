use super::aamoonphases;
use super::aacoordinatetransformation;

pub const TOTAL_ECLIPSE: u64 = 0x01;
pub const ANNULAR_ECLIPSE: u64 = 0x02;
pub const ANNULAR_TOTAL_ECLIPSE: u64 = 0x04;
pub const CENTRAL_ECLIPSE: u64 = 0x08;
pub const PARTIAL_ECLIPSE: u64 = 0x10;
pub const NON_CENTRAL_ECLIPSE: u64 = 0x20;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AASolarEclipseDetails {
    pub flags: u64,
    pub time_of_maximum_eclipse: f64,
    pub f: f64,
    pub u: f64,
    pub gamma: f64,
    pub greatest_magnitude: f64,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AALunarEclipseDetails {
    pub b_eclipse: bool,
    pub time_of_maximum_eclipse: f64,
    pub f: f64,
    pub u: f64,
    pub gamma: f64,
    pub penumbral_radii: f64,
    pub umbral_radii: f64,
    pub penumbral_magnitude: f64,
    pub umbral_magnitude: f64,
    pub partial_phase_semi_duration: f64,
    pub total_phase_semi_duration: f64,
    pub partial_phase_penumbra_semi_duration: f64,
}

fn calculate(k: f64, mdash: &mut f64) -> AASolarEclipseDetails
{
    //Are we looking for a solar or lunar eclipse
    let b_solar_eclipse = f64::fract(k) == 0.0;

    //What will be the return value
    let mut details: AASolarEclipseDetails = Default::default();

    //convert from K to t
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;

    let e = 1.0 - 0.002516 * t - 0.0000074 * t2;

    let mut m = aacoordinatetransformation::map_to_0to360_range(2.5534 + 29.10535670 * k - 0.0000014 * t2 - 0.00000011 * t3);
    m = aacoordinatetransformation::degrees_to_radians(m);

    *mdash = aacoordinatetransformation::map_to_0to360_range(201.5643 + 385.81693528 * k + 0.0107582 * t2 + 0.00001238 * t3 - 0.000000058 * t4);
    *mdash = aacoordinatetransformation::degrees_to_radians(*mdash);

    let mut omega = aacoordinatetransformation::map_to_0to360_range(124.7746 - 1.56375588 * k + 0.0020672 * t2 + 0.00000215 * t3);
    omega = aacoordinatetransformation::degrees_to_radians(omega);

    let mut f = aacoordinatetransformation::map_to_0to360_range(160.7108 + 390.67050284 * k - 0.0016118 * t2 - 0.00000227 * t3 + 0.00000001 * t4);
    details.f = f;
    let mut fdash = f - 0.02665 * f64::sin(omega);

    f = aacoordinatetransformation::degrees_to_radians(f);
    fdash = aacoordinatetransformation::degrees_to_radians(fdash);

    //Do the first check to see if we have an eclipse
    if f64::abs(f64::sin(f)) > 0.36 {
        return details;
    }

    let mut a1 = aacoordinatetransformation::map_to_0to360_range(299.77 + 0.107408 * k - 0.009173 * t2);
    a1 = aacoordinatetransformation::degrees_to_radians(a1);

    details.time_of_maximum_eclipse = aamoonphases::mean_phase(k);

    let mut delta_jd = 0.0f64;
    if b_solar_eclipse {
        delta_jd += -0.4075 * f64::sin(*mdash) +
            0.1721 * e * f64::sin(m);
    } else {
        delta_jd += -0.4065 * f64::sin(*mdash) +
            0.1727 * e * f64::sin(m);
    }
    delta_jd += 0.0161 * f64::sin(2.0 * *mdash) +
        -0.0097 * f64::sin(2.0 * fdash) +
        0.0073 * e * f64::sin(*mdash - m) +
        -0.0050 * e * f64::sin(*mdash + m) +
        -0.0023 * f64::sin(*mdash - 2.0 * fdash) +
        0.0021 * e * f64::sin(2.0 * m) +
        0.0012 * f64::sin(*mdash + 2.0 * fdash) +
        0.0006 * e * f64::sin(2.0 * *mdash + m) +
        -0.0004 * f64::sin(3.0 * *mdash) +
        -0.0003 * e * f64::sin(m + 2.0 * fdash) +
        0.0003 * f64::sin(a1) +
        -0.0002 * e * f64::sin(m - 2.0 * fdash) +
        -0.0002 * e * f64::sin(2.0 * *mdash - m) +
        -0.0002 * f64::sin(omega);

    details.time_of_maximum_eclipse += delta_jd;

    let p = 0.2070 * e * f64::sin(m) +
        0.0024 * e * f64::sin(2.0 * m) +
        -0.0392 * f64::sin(*mdash) +
        0.0116 * f64::sin(2.0 * *mdash) +
        -0.0073 * e * f64::sin(*mdash + m) +
        0.0067 * e * f64::sin(*mdash - m) +
        0.0118 * f64::sin(2.0 * fdash);

    let q = 5.2207 +
        -0.0048 * e * f64::cos(m) +
        0.0020 * e * f64::cos(2.0 * m) +
        -0.3299 * f64::cos(*mdash) +
        -0.0060 * e * f64::cos(*mdash + m) +
        0.0041 * e * f64::cos(*mdash - m);

    let w = f64::abs(f64::cos(fdash));

    details.gamma = (p * f64::cos(fdash) + q * f64::sin(fdash)) * (1.0 - 0.0048 * w);

    details.u = 0.0059 +
        0.0046 * e * f64::cos(m) +
        -0.0182 * f64::cos(*mdash) +
        0.0004 * f64::cos(2.0 * *mdash) +
        -0.0005 * f64::cos(m + *mdash);

    //Check to see if the eclipse is visible from the Earth's surface
    let fgamma = f64::abs(details.gamma);
    if fgamma > (1.5433 + details.u) {
        return details;
    }

    //We have an eclipse at this time, fill in the details
    if fgamma < 0.9972 {
        if details.u < 0.0 {
            details.flags = TOTAL_ECLIPSE;
        } else if details.u > 0.0047 {
            details.flags = ANNULAR_ECLIPSE;
        } else if (details.u >= 0.0) && (details.u <= 0.0047) {
            let w = 0.00464 * f64::sqrt(1.0 - (details.gamma * details.gamma));
            if details.u < w {
                details.flags = ANNULAR_TOTAL_ECLIPSE;
            } else {
                details.flags = ANNULAR_ECLIPSE;
            }
        }

        details.flags |= CENTRAL_ECLIPSE;
    } else if (fgamma > 0.9972) && (fgamma < (1.5433 + details.u)) {
        if (fgamma > 0.9972) && (fgamma < (0.9972 + f64::abs(details.u))) {
            if details.u < 0.0 {
                details.flags = TOTAL_ECLIPSE;
            } else if details.u > 0.0047 {
                details.flags = ANNULAR_ECLIPSE;
            } else if details.u >= 0.0 && details.u <= 0.0047 {
                let w = 0.00464 * f64::sqrt(1.0 - (details.gamma * details.gamma));
                if details.u < w {
                    details.flags = ANNULAR_TOTAL_ECLIPSE;
                } else {
                    details.flags = ANNULAR_ECLIPSE;
                }
            }
        } else {
            details.flags = PARTIAL_ECLIPSE;
            details.greatest_magnitude = (1.5433 + details.u - fgamma) / (0.5461 + (2.0 * details.u));
        }

        details.flags |= NON_CENTRAL_ECLIPSE;
    }

    details
}

pub fn calculate_solar(k: f64) -> AASolarEclipseDetails
{
    debug_assert!(f64::fract(k) == 0.0);

    let mut mdash = 0.0f64;
    calculate(k, &mut mdash)
}

pub fn calculate_lunar(k: f64) -> AALunarEclipseDetails
{
    debug_assert!(f64::fract(k) == 0.0);

    let mut mdash = 0.0f64;
    let solar_details = calculate(k, &mut mdash);

    //What will be the return value
    let mut details: AALunarEclipseDetails = Default::default();
    details.b_eclipse = solar_details.flags != 0;
    details.f = solar_details.f;
    details.gamma = solar_details.gamma;
    details.time_of_maximum_eclipse = solar_details.time_of_maximum_eclipse;
    details.u = solar_details.u;

    if details.b_eclipse {
        details.penumbral_radii = 1.2848 + details.u;
        details.umbral_radii = 0.7403 - details.u;
        let fgamma = f64::abs(details.gamma);
        details.penumbral_magnitude = (1.5573 + details.u - fgamma) / 0.5450;
        details.umbral_magnitude = (1.0128 - details.u - fgamma) / 0.5450;

        let p = 1.0128 - details.u;
        let t = 0.4678 - details.u;
        let n = 0.5458 + 0.0400 * f64::cos(mdash);

        let gamma2 = details.gamma * details.gamma;
        let p2 = p * p;
        if p2 >= gamma2 {
            details.partial_phase_semi_duration = 60.0 / n * f64::sqrt(p2 - gamma2);
        }

        let t2 = t * t;
        if t2 >= gamma2 {
            details.total_phase_semi_duration = 60.0 / n * f64::sqrt(t2 - gamma2);
        }

        let h = 1.5573 + details.u;
        let h2 = h * h;
        if h2 >= gamma2 {
            details.partial_phase_penumbra_semi_duration = 60.0 / n * f64::sqrt(h2 - gamma2);
        }
    }

    details
}

