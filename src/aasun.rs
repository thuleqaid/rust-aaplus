use super::aacoordinatetransformation;
use super::aa3dcoordinate;
use super::aaearth;
use super::aafk5;
use super::aanutation;

pub fn geometric_ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    aacoordinatetransformation::map_to_0to360_range(aaearth::ecliptic_longitude(jd, b_high_precision) + 180.0)
}

pub fn geometric_ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    -aaearth::ecliptic_latitude(jd, b_high_precision)
}

pub fn geometric_ecliptic_longitude_j2000(jd: f64, b_high_precision: bool) -> f64
{
    aacoordinatetransformation::map_to_0to360_range(aaearth::ecliptic_longitude_j2000(jd, b_high_precision) + 180.0)
}

pub fn geometric_ecliptic_latitude_j2000(jd: f64, b_high_precision: bool) -> f64
{
    -aaearth::ecliptic_latitude_j2000(jd, b_high_precision)
}

pub fn geometric_fk5ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    //Convert to the FK5 system
    let longitude = geometric_ecliptic_longitude(jd, b_high_precision);
    let latitude = geometric_ecliptic_latitude(jd, b_high_precision);
    longitude + aafk5::correction_in_longitude(longitude, latitude, jd)
}

pub fn geometric_fk5ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    //Convert to the FK5 system
    let longitude = geometric_ecliptic_longitude(jd, b_high_precision);
    let latitude = geometric_ecliptic_latitude(jd, b_high_precision);
    let sun_lat_correction = aafk5::correction_in_latitude(longitude, jd);
    latitude + sun_lat_correction
}

pub fn apparent_ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    let mut longitude = geometric_fk5ecliptic_longitude(jd, b_high_precision);

    //Apply the correction in longitude due to nutation
    longitude += aacoordinatetransformation::dms_to_degrees(0.0, 0.0, aanutation::nutation_in_longitude(jd), true);

    //Apply the correction in longitude due to aberration
    let r = aaearth::radius_vector(jd, b_high_precision);
    if b_high_precision {
        longitude -= 0.005775518 * r * aacoordinatetransformation::dms_to_degrees(0.0, 0.0, variation_geometric_ecliptic_longitude(jd), true);
    } else {
        longitude -= aacoordinatetransformation::dms_to_degrees(0.0, 0.0, 20.4898 / r, true);
    }

    longitude
}

pub fn apparent_ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    geometric_fk5ecliptic_latitude(jd, b_high_precision)
}

pub fn equatorial_rectangular_coordinates_mean_equinox(jd: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let longitude = aacoordinatetransformation::degrees_to_radians(geometric_fk5ecliptic_longitude(jd, b_high_precision));
    let latitude = aacoordinatetransformation::degrees_to_radians(geometric_fk5ecliptic_latitude(jd, b_high_precision));
    let r = aaearth::radius_vector(jd, b_high_precision);
    let epsilon = aacoordinatetransformation::degrees_to_radians(aanutation::mean_obliquity_of_ecliptic(jd));

    aa3dcoordinate::AA3DCoordinate {
        x: r * f64::cos(latitude) * f64::cos(longitude),
        y: r * (f64::cos(latitude) * f64::sin(longitude) * f64::cos(epsilon) - f64::sin(latitude) * f64::sin(epsilon)),
        z: r * (f64::cos(latitude) * f64::sin(longitude) * f64::sin(epsilon) + f64::sin(latitude) * f64::cos(epsilon)),
    }
}

pub fn ecliptic_rectangular_coordinates_j2000(jd: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let mut longitude = geometric_ecliptic_longitude_j2000(jd, b_high_precision);
    longitude = aacoordinatetransformation::degrees_to_radians(longitude);
    let mut latitude = geometric_ecliptic_latitude_j2000(jd, b_high_precision);
    latitude = aacoordinatetransformation::degrees_to_radians(latitude);
    let r = aaearth::radius_vector(jd, b_high_precision);

    let coslatitude = f64::cos(latitude);
    aa3dcoordinate::AA3DCoordinate {
        x: r * coslatitude * f64::cos(longitude),
        y: r * coslatitude * f64::sin(longitude),
        z: r * f64::sin(latitude),
    }
}

pub fn equatorial_rectangular_coordinates_j2000(jd: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let value = ecliptic_rectangular_coordinates_j2000(jd, b_high_precision);
    aafk5::convert_vsop_to_fk5_j2000(&value)
}

pub fn equatorial_rectangular_coordinates_b1950(jd: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let value = ecliptic_rectangular_coordinates_j2000(jd, b_high_precision);
    aafk5::convert_vsop_to_fk5_b1950(&value)
}

pub fn equatorial_rectangular_coordinates_any_equinox(jd: f64, jdequinox: f64, b_high_precision: bool) -> aa3dcoordinate::AA3DCoordinate
{
    let value = equatorial_rectangular_coordinates_j2000(jd, b_high_precision);
    aafk5::convert_vsop_to_fk5_any_equinox(&value, jdequinox)
}

pub fn variation_geometric_ecliptic_longitude(jd: f64) -> f64
{
    //d is the number of days since the epoch
    let d = jd - 2451545.00;
    let tau = d / 365250.0;
    let tau2 = tau * tau;
    let tau3 = tau2 * tau;

    let delta_lambda: f64 = 3548.193
        + 118.568 * f64::sin(aacoordinatetransformation::degrees_to_radians(87.5287 + 359993.7286 * tau))
        + 2.476 * f64::sin(aacoordinatetransformation::degrees_to_radians(85.0561 + 719987.4571 * tau))
        + 1.376 * f64::sin(aacoordinatetransformation::degrees_to_radians(27.8502 + 4452671.1152 * tau))
        + 0.119 * f64::sin(aacoordinatetransformation::degrees_to_radians(73.1375 + 450368.8564 * tau))
        + 0.114 * f64::sin(aacoordinatetransformation::degrees_to_radians(337.2264 + 329644.6718 * tau))
        + 0.086 * f64::sin(aacoordinatetransformation::degrees_to_radians(222.5400 + 659289.3436 * tau))
        + 0.078 * f64::sin(aacoordinatetransformation::degrees_to_radians(162.8136 + 9224659.7915 * tau))
        + 0.054 * f64::sin(aacoordinatetransformation::degrees_to_radians(82.5823 + 1079981.1857 * tau))
        + 0.052 * f64::sin(aacoordinatetransformation::degrees_to_radians(171.5189 + 225184.4282 * tau))
        + 0.034 * f64::sin(aacoordinatetransformation::degrees_to_radians(30.3214 + 4092677.3866 * tau))
        + 0.033 * f64::sin(aacoordinatetransformation::degrees_to_radians(119.8105 + 337181.4711 * tau))
        + 0.023 * f64::sin(aacoordinatetransformation::degrees_to_radians(247.5418 + 299295.6151 * tau))
        + 0.023 * f64::sin(aacoordinatetransformation::degrees_to_radians(325.1526 + 315559.5560 * tau))
        + 0.021 * f64::sin(aacoordinatetransformation::degrees_to_radians(155.1241 + 675553.2846 * tau))
        + 7.311 * tau * f64::sin(aacoordinatetransformation::degrees_to_radians(333.4515 + 359993.7286 * tau))
        + 0.305 * tau * f64::sin(aacoordinatetransformation::degrees_to_radians(330.9814 + 719987.4571 * tau))
        + 0.010 * tau * f64::sin(aacoordinatetransformation::degrees_to_radians(328.5170 + 1079981.1857 * tau))
        + 0.309 * tau2 * f64::sin(aacoordinatetransformation::degrees_to_radians(241.4518 + 359993.7286 * tau))
        + 0.021 * tau2 * f64::sin(aacoordinatetransformation::degrees_to_radians(205.0482 + 719987.4571 * tau))
        + 0.004 * tau2 * f64::sin(aacoordinatetransformation::degrees_to_radians(297.8610 + 4452671.1152 * tau))
        + 0.010 * tau3 * f64::sin(aacoordinatetransformation::degrees_to_radians(154.7066 + 359993.7286 * tau));
    delta_lambda
}
