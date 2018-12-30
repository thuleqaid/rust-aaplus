use super::aacoordinatetransformation;
use super::aasun;

pub fn equinox(year: i64, index: i64, b_high_precision: bool) -> f64
{
    let csk: [[f64; 5]; 8] = [
        [1721139.29189, 365242.13740, 0.06134, 0.00111, -0.00071],
        [2451623.80984, 365242.37404, 0.05169, -0.00411, -0.00057],
        [1721233.25401, 365241.72562, -0.05323, 0.00907, 0.00025],
        [2451716.56767, 365241.62603, 0.00325, 0.00888, -0.00030],
        [1721325.70455, 365242.49558, -0.11677, -0.00297, 0.00074],
        [2451810.21715, 365242.01767, -0.11575, 0.00337, 0.00078],
        [1721414.39987, 365242.88257, -0.00769, -0.00933, -0.00006],
        [2451900.05952, 365242.74049, -0.06223, -0.00823, 0.00032],
    ];
    //calculate the approximate date
    let index2 = (((index - 1) % 24 + 24) % 24 + 1) as usize;
    let mut cskidx =
        if index2 <= 6 {
            0
        } else if index2 <= 12 {
            2
        } else if index2 <= 18 {
            4
        } else {
            6
        };
    let mut jde;
    if year <= 1000 {
        let y = year as f64 / 1000.0;
        let ysquared = y * y;
        let ycubed = ysquared * y;
        let y4 = ycubed * y;
        jde = csk[cskidx][0] + csk[cskidx][1] * y + csk[cskidx][2] * ysquared + csk[cskidx][3] * ycubed + csk[cskidx][4] * y4;
    } else {
        let y = (year as f64 - 2000.0) / 1000.0;
        let ysquared = y * y;
        let ycubed = ysquared * y;
        let y4 = ycubed * y;
        cskidx += 1;
        jde = csk[cskidx][0] + csk[cskidx][1] * y + csk[cskidx][2] * ysquared + csk[cskidx][3] * ycubed + csk[cskidx][4] * y4;
    }

    let mut correction = 1.0;
    //Corresponds to an error of 0.86 of a second
    while f64::abs(correction) > 0.00001 {
        let sun_longitude = aasun::apparent_ecliptic_longitude(jde, b_high_precision);
        correction = 58.0 * f64::sin(aacoordinatetransformation::degrees_to_radians(((index2 as i64 - 4) * 15) as f64 - sun_longitude));
        jde += correction;
    }

    jde
}

pub fn northward_equinox(year: i64, b_high_precision: bool) -> f64
{
    equinox(year, 4, b_high_precision)
}

pub fn northern_solstice(year: i64, b_high_precision: bool) -> f64
{
    equinox(year, 10, b_high_precision)
}

pub fn southward_equinox(year: i64, b_high_precision: bool) -> f64
{
    equinox(year, 16, b_high_precision)
}

pub fn southern_solstice(year: i64, b_high_precision: bool) -> f64
{
    equinox(year, 22, b_high_precision)
}

pub fn length_of_spring(year: i64, b_northern_hemisphere: bool, b_high_precision: bool) -> f64
{
    if b_northern_hemisphere {
        northern_solstice(year, b_high_precision) - northward_equinox(year, b_high_precision)
    } else {
        southern_solstice(year, b_high_precision) - southward_equinox(year, b_high_precision)
    }
}

pub fn length_of_summer(year: i64, b_northern_hemisphere: bool, b_high_precision: bool) -> f64
{
    if b_northern_hemisphere {
        southward_equinox(year, b_high_precision) - northern_solstice(year, b_high_precision)
    } else {
        //The Summer season wraps around into the following year for the southern hemisphere
        northward_equinox(year + 1, b_high_precision) - southern_solstice(year, b_high_precision)
    }
}

pub fn length_of_autumn(year: i64, b_northern_hemisphere: bool, b_high_precision: bool) -> f64
{
    if b_northern_hemisphere {
        southern_solstice(year, b_high_precision) - southward_equinox(year, b_high_precision)
    } else {
        northern_solstice(year, b_high_precision) - northward_equinox(year, b_high_precision)
    }
}

pub fn length_of_winter(year: i64, b_northern_hemisphere: bool, b_high_precision: bool) -> f64
{
    if b_northern_hemisphere {
        //The Winter season wraps around into the following year for the nothern hemisphere
        northward_equinox(year + 1, b_high_precision) - southern_solstice(year, b_high_precision)
    } else {
        southward_equinox(year, b_high_precision) - northern_solstice(year, b_high_precision)
    }
}
