extern crate aaplus;

fn sun_position(year:i64, index:i64) -> f64
{
    aaplus::aadynamicaltime::tt2utc(aaplus::aaequinoxesandsolstices::equinox(year, index, false)) + 8./24.
}
fn moon_position(utc:f64) -> f64
{
    let jd = aaplus::aadate::AADate::from_jd(utc, true);
    let year = jd.year() as f64 + jd.day_of_year() / jd.days_in_year() as f64;
    let k = aaplus::aamoonphases::k(year);
    aaplus::aamoonphases::true_phase(f64::ceil(k))
}
fn moon_first_day(year:i64) -> (f64, i64)
{
    let mut zq = [0.0_f64;13];
    let mut hs = [0.0_f64;15];
    let mut utc:f64;
    let mut y = year - 1;
    utc = sun_position(y, 22);
    zq[0] = f64::floor(utc + 8./24. + 0.5) - 0.5;
    for i in 0..15 {
        hs[i] = f64::floor(moon_position(utc) + 8./24. + 0.5) - 0.5;
        utc = hs[i] + 15.0;
    }
    let mut idx = 24_i64;
    for i in 1..13 {
        zq[i] = f64::floor(sun_position(y, idx) + 8./24. + 0.5) - 0.5;
        idx += 2;
        if idx > 24 {
            idx -= 24;
            y += 1;
        }
    }
    idx = 4;
    while (idx < 13) && (zq[idx as usize] < hs[idx as usize]) {
        idx += 1;
    }
    if idx >= 13 {
        idx = 0;
    } else {
        idx -= 2;
    }
    for i in 0..13 {
        if hs[i+2] - hs[i+1] > 29.5 {
            idx = idx | (1 << (i + 8));
        }
    }
    println!("{:08X}", idx);
    (hs[1], idx)
}
fn main()
{
    let jd = sun_position(2018, 4);
    let date = aaplus::aadate::AADate::from_jd(jd, true);
    let mut year = 0_i64;
    let mut month = 0_i64;
    let mut day = 0_i64;
    let mut hour = 0_i64;
    let mut minute = 0_i64;
    let mut second = 0.0_f64;
    date.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
    println!("{}/{:02}/{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, f64::round(second));
    let jdinfo = moon_first_day(2018);
    let date = aaplus::aadate::AADate::from_jd(jdinfo.0, true);
    date.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
    println!("{}/{:02}/{:02}", year, month, day);
}
