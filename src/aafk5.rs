use super::aacoordinatetransformation;
use super::aa3dcoordinate;

pub fn correction_in_longitude(longitude: f64, latitude: f64, jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let mut ldash = longitude - 1.397 * t - 0.00031 * t * t;

    //Convert to radians
    ldash = aacoordinatetransformation::degrees_to_radians(ldash);
    let latitude2 = aacoordinatetransformation::degrees_to_radians(latitude);

    let value = -0.09033 + 0.03916 * (f64::cos(ldash) + f64::sin(ldash)) * f64::tan(latitude2);
    aacoordinatetransformation::dms_to_degrees(0.0, 0.0, value, true)
}

pub fn correction_in_latitude(longitude: f64, jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let mut ldash = longitude - 1.397 * t - 0.00031 * t * t;

    //Convert to radians
    ldash = aacoordinatetransformation::degrees_to_radians(ldash);

    let value = 0.03916 * (f64::cos(ldash) - f64::sin(ldash));
    aacoordinatetransformation::dms_to_degrees(0.0, 0.0, value, true)
}

pub fn convert_vsop_to_fk5_j2000(value: &aa3dcoordinate::AA3DCoordinate) -> aa3dcoordinate::AA3DCoordinate
{
    aa3dcoordinate::AA3DCoordinate {
        x: value.x + 0.000000440360 * value.y - 0.000000190919 * value.z,
        y: -0.000000479966 * value.x + 0.917482137087 * value.y - 0.397776982902 * value.z,
        z: 0.397776982902 * value.y + 0.917482137087 * value.z,
    }
}

pub fn convert_vsop_to_fk5_b1950(value: &aa3dcoordinate::AA3DCoordinate) -> aa3dcoordinate::AA3DCoordinate
{
    aa3dcoordinate::AA3DCoordinate {
        x: 0.999925702634 * value.x + 0.012189716217 * value.y + 0.000011134016 * value.z,
        y: -0.011179418036 * value.x + 0.917413998946 * value.y - 0.397777041885 * value.z,
        z: -0.004859003787 * value.x + 0.397747363646 * value.y + 0.917482111428 * value.z,
    }
}

pub fn convert_vsop_to_fk5_any_equinox(value: &aa3dcoordinate::AA3DCoordinate, jdequinox: f64) -> aa3dcoordinate::AA3DCoordinate
{
    let t = (jdequinox - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    let mut sigma = 2306.2181 * t + 0.30188 * tsquared + 0.017988 * tcubed;
    sigma = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, sigma, true));

    let mut zeta = 2306.2181 * t + 1.09468 * tsquared + 0.018203 * tcubed;
    zeta = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, zeta, true));

    let mut phi = 2004.3109 * t - 0.42665 * tsquared - 0.041833 * tcubed;
    phi = aacoordinatetransformation::degrees_to_radians(aacoordinatetransformation::dms_to_degrees(0.0, 0.0, phi, true));

    let cossigma = f64::cos(sigma);
    let coszeta = f64::cos(zeta);
    let cosphi = f64::cos(phi);
    let sinsigma = f64::sin(sigma);
    let sinzeta = f64::sin(zeta);
    let sinphi = f64::sin(phi);

    let xx = cossigma * coszeta * cosphi - sinsigma * sinzeta;
    let xy = sinsigma * coszeta + cossigma * sinzeta * cosphi;
    let xz = cossigma * sinphi;
    let yx = -cossigma * sinzeta - sinsigma * coszeta * cosphi;
    let yy = cossigma * coszeta - sinsigma * sinzeta * cosphi;
    let yz = -sinsigma * sinphi;
    let zx = -coszeta * sinphi;
    let zy = -sinzeta * sinphi;
    let zz = cosphi;

    aa3dcoordinate::AA3DCoordinate {
        x: xx * value.x + yx * value.y + zx * value.z,
        y: xy * value.x + yy * value.y + zy * value.z,
        z: xz * value.x + yz * value.y + zz * value.z,
    }
}
