use super::aa2dcoordinate::AA2DCoordinate;

pub fn degrees_to_radians(degrees: f64) -> f64
{
    degrees * 0.017453292519943295769236907684886
}

pub fn radians_to_degrees(radians: f64) -> f64
{
    radians * 57.295779513082320876798154814105
}

pub fn radians_to_hours(radians: f64) -> f64
{
    radians * 3.8197186342054880584532103209403
}

pub fn hours_to_radians(hours: f64) -> f64
{
    hours * 0.26179938779914943653855361527329
}

pub fn hours_to_degrees(hours: f64) -> f64
{
    hours * 15.0
}

pub fn degrees_to_hours(degrees: f64) -> f64
{
    degrees / 15.0
}

pub fn pi() -> f64
{
    3.1415926535897932384626433832795
}

pub fn map_to_0to360_range(degrees: f64) -> f64
{
    let mut f_result = degrees % 360.0;
    if f_result < 0.0 {
        f_result += 360.0;
    }
    f_result
}

pub fn map_to_minus90to90_range(degrees: f64) -> f64
{
    let mut f_result = map_to_0to360_range(degrees);
    if f_result > 270.0 {
        f_result = f_result - 360.0;
    } else if f_result > 180.0 {
        f_result = 180.0 - f_result;
    } else if f_result > 90.0 {
        f_result = 180.0 - f_result;
    }
    f_result
}

pub fn map_to_0to24_range(hour_angle: f64) -> f64
{
    let mut f_result = hour_angle % 24.0;
    if f_result < 0.0 {
        f_result += 24.0;
    }
    f_result
}

pub fn map_to_0to2pi_range(angle: f64) -> f64
{
    let mut f_result = angle % (2.0 * pi());
    if f_result < 0.0 {
        f_result += 2.0 * pi();
    }
    f_result
}

pub fn equatorial_to_ecliptic(alpha: f64, delta: f64, epsilon: f64) -> AA2DCoordinate
{
    let alpha2 = hours_to_radians(alpha);
    let delta2 = degrees_to_radians(delta);
    let epsilon2 = degrees_to_radians(epsilon);

    let mut ecliptic: AA2DCoordinate = Default::default();
    ecliptic.x = radians_to_degrees(f64::atan2(f64::sin(alpha2) * f64::cos(epsilon2) + f64::tan(delta2) * f64::sin(epsilon2), f64::cos(alpha2)));
    if ecliptic.x < 0.0 {
        ecliptic.x += 360.0;
    }
    ecliptic.y = radians_to_degrees(f64::asin(f64::sin(delta2) * f64::cos(epsilon2) - f64::cos(delta2) * f64::sin(epsilon2) * f64::sin(alpha2)));
    ecliptic
}

pub fn ecliptic_to_equatorial(lambda: f64, beta: f64, epsilon: f64) -> AA2DCoordinate
{
    let lambda2 = degrees_to_radians(lambda);
    let beta2 = degrees_to_radians(beta);
    let epsilon2 = degrees_to_radians(epsilon);

    let mut equatorial: AA2DCoordinate = Default::default();
    equatorial.x = radians_to_hours(f64::atan2(f64::sin(lambda2) * f64::cos(epsilon2) - f64::tan(beta2) * f64::sin(epsilon2), f64::cos(lambda2)));
    if equatorial.x < 0.0 {
        equatorial.x += 24.0;
    }
    equatorial.y = radians_to_degrees(f64::asin(f64::sin(beta2) * f64::cos(epsilon2) + f64::cos(beta2) * f64::sin(epsilon2) * f64::sin(lambda2)));
    equatorial
}

pub fn equatorial_to_horizontal(local_hour_angle: f64, delta: f64, latitude: f64) -> AA2DCoordinate
{
    let local_hour_angle2 = hours_to_radians(local_hour_angle);
    let delta2 = degrees_to_radians(delta);
    let latitude2 = degrees_to_radians(latitude);

    let mut horizontal: AA2DCoordinate = Default::default();
    horizontal.x = radians_to_degrees(f64::atan2(f64::sin(local_hour_angle2), f64::cos(local_hour_angle2) * f64::sin(latitude2) - f64::tan(delta2) * f64::cos(latitude2)));
    if horizontal.x < 0.0 {
        horizontal.x += 360.0;
    }
    horizontal.y = radians_to_degrees(f64::asin(f64::sin(latitude2) * f64::sin(delta2) + f64::cos(latitude2) * f64::cos(delta2) * f64::cos(local_hour_angle2)));
    horizontal
}

pub fn horizontal_to_equatorial(azimuth: f64, altitude: f64, latitude: f64) -> AA2DCoordinate
{
    //Convert from degress to radians
    let azimuth2 = degrees_to_radians(azimuth);
    let altitude2 = degrees_to_radians(altitude);
    let latitude2 = degrees_to_radians(latitude);

    let mut equatorial: AA2DCoordinate = Default::default();
    equatorial.x = radians_to_hours(f64::atan2(f64::sin(azimuth2), f64::cos(azimuth2) * f64::sin(latitude2) + f64::tan(altitude2) * f64::cos(latitude2)));
    if equatorial.x < 0.0 {
        equatorial.x += 24.0;
    }
    equatorial.y = radians_to_degrees(f64::asin(f64::sin(latitude2) * f64::sin(altitude2) - f64::cos(latitude2) * f64::cos(altitude2) * f64::cos(azimuth2)));
    equatorial
}

pub fn equatorial_to_galactic(alpha: f64, delta: f64) -> AA2DCoordinate
{
    let mut alpha2 = 192.25 - hours_to_degrees(alpha);
    alpha2 = degrees_to_radians(alpha2);
    let delta2 = degrees_to_radians(delta);

    let mut galactic: AA2DCoordinate = Default::default();
    galactic.x = radians_to_degrees(f64::atan2(f64::sin(alpha2), f64::cos(alpha2) * f64::sin(degrees_to_radians(27.4)) - f64::tan(delta2) * f64::cos(degrees_to_radians(27.4))));
    galactic.x = 303.0 - galactic.x;
    if galactic.x >= 360.0 {
        galactic.x -= 360.0;
    }
    galactic.y = radians_to_degrees(f64::asin(f64::sin(delta2) * f64::sin(degrees_to_radians(27.4)) + f64::cos(delta2) * f64::cos(degrees_to_radians(27.4)) * f64::cos(alpha2)));
    galactic
}

pub fn galactic_to_equatorial(l: f64, b: f64) -> AA2DCoordinate
{
    let mut l2 = l - 123.0;
    l2 = degrees_to_radians(l2);
    let b2 = degrees_to_radians(b);

    let mut equatorial: AA2DCoordinate = Default::default();
    equatorial.x = radians_to_degrees(f64::atan2(f64::sin(l2), f64::cos(l2) * f64::sin(degrees_to_radians(27.4)) - f64::tan(b2) * f64::cos(degrees_to_radians(27.4))));
    equatorial.x += 12.25;
    if equatorial.x < 0.0 {
        equatorial.x += 360.0;
    }
    equatorial.x = degrees_to_hours(equatorial.x);
    equatorial.y = radians_to_degrees(f64::asin(f64::sin(b2) * f64::sin(degrees_to_radians(27.4)) + f64::cos(b2) * f64::cos(degrees_to_radians(27.4)) * f64::cos(l2)));
    equatorial
}

pub fn dms_to_degrees(degrees: f64, minutes: f64, seconds: f64, b_positive: bool) -> f64
{
    //validate our parameters
    if !b_positive
        {
            assert!(degrees >= 0.0);  //All parameters should be non negative if the "bPositive" parameter is false
            assert!(minutes >= 0.0);
            assert!(seconds >= 0.0);
        }

    if b_positive {
        degrees + minutes / 60.0 + seconds / 3600.0
    } else {
        -degrees - minutes / 60.0 - seconds / 3600.0
    }
}

