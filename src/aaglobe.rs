use super::aacoordinatetransformation;

pub fn rho_sin_theta_prime(geographical_latitude: f64, height: f64) -> f64
{
    //Convert from degress to radians
    let geographical_latitude = aacoordinatetransformation::degrees_to_radians(geographical_latitude);

    let u = f64::atan(0.99664719 * f64::tan(geographical_latitude));
    0.99664719 * f64::sin(u) + (height / 6378140.0 * f64::sin(geographical_latitude))
}

pub fn rho_cos_theta_prime(geographical_latitude: f64, height: f64) -> f64
{
    //Convert from degress to radians
    let geographical_latitude = aacoordinatetransformation::degrees_to_radians(geographical_latitude);

    let u = f64::atan(0.99664719 * f64::tan(geographical_latitude));
    f64::cos(u) + (height / 6378140.0 * f64::cos(geographical_latitude))
}

pub fn radius_of_parallel_of_latitude(geographical_latitude: f64) -> f64
{
    //Convert from degress to radians
    let geographical_latitude = aacoordinatetransformation::degrees_to_radians(geographical_latitude);

    let sin_geo = f64::sin(geographical_latitude);
    (6378.14 * f64::cos(geographical_latitude)) / (f64::sqrt(1.0 - 0.0066943847614084 * sin_geo * sin_geo))
}

pub fn radius_of_curvature(geographical_latitude: f64) -> f64
{
    //Convert from degress to radians
    let geographical_latitude = aacoordinatetransformation::degrees_to_radians(geographical_latitude);

    let sin_geo = f64::sin(geographical_latitude);
    (6378.14 * (1.0 - 0.0066943847614084)) / f64::powf(1.0 - 0.0066943847614084 * sin_geo * sin_geo, 1.5)
}

pub fn distance_between_points(geographical_latitude1: f64, geographical_longitude1: f64, geographical_latitude2: f64, geographical_longitude2: f64) -> f64
{
    //Convert from degress to radians
    let geographical_latitude1 = aacoordinatetransformation::degrees_to_radians(geographical_latitude1);
    let geographical_latitude2 = aacoordinatetransformation::degrees_to_radians(geographical_latitude2);
    let geographical_longitude1 = aacoordinatetransformation::degrees_to_radians(geographical_longitude1);
    let geographical_longitude2 = aacoordinatetransformation::degrees_to_radians(geographical_longitude2);

    let f = (geographical_latitude1 + geographical_latitude2) / 2.0;
    let g = (geographical_latitude1 - geographical_latitude2) / 2.0;
    let lambda = (geographical_longitude1 - geographical_longitude2) / 2.0;
    let sin_g = f64::sin(g);
    let cos_g = f64::cos(g);
    let cos_f = f64::cos(f);
    let sin_f = f64::sin(f);
    let sin_lambda = f64::sin(lambda);
    let cos_lambda = f64::cos(lambda);
    let s = (sin_g * sin_g * cos_lambda * cos_lambda) + (cos_f * cos_f * sin_lambda * sin_lambda);
    let c = (cos_g * cos_g * cos_lambda * cos_lambda) + (sin_f * sin_f * sin_lambda * sin_lambda);
    let w = f64::atan(f64::sqrt(s / c));
    let r = f64::sqrt(s * c) / w;
    let d = 2.0 * w * 6378.14;
    let hprime = (3.0 * r - 1.0) / (2.0 * c);
    let hprime2 = (3.0 * r + 1.0) / (2.0 * s);
    let f2 = 0.0033528131778969144060323814696721;

    d * (1.0 + (f2 * hprime * sin_f * sin_f * cos_g * cos_g) - (f2 * hprime2 * cos_f * cos_f * sin_g * sin_g))
}
