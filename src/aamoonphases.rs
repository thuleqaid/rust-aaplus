use super::aacoordinatetransformation;

pub fn k(year: f64) -> f64
{
    12.3685 * (year - 2000.0)
}

pub fn mean_phase(k: f64) -> f64
{
    //convert from K to T
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    2451550.09766 + 29.530588861 * k + 0.00015437 * t2 - 0.000000150 * t3 + 0.00000000073 * t4
}

pub fn true_phase(k: f64) -> f64
{
    //What will be the return value
    let mut jd = mean_phase(k);

    //convert from K to T
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;

    let e = 1.0 - 0.002516 * t - 0.0000074 * t2;
    let e2 = e * e;

    let mut m = aacoordinatetransformation::map_to_0to360_range(2.5534 + 29.10535670 * k - 0.0000014 * t2 - 0.00000011 * t3);
    m = aacoordinatetransformation::degrees_to_radians(m);
    let mut mdash = aacoordinatetransformation::map_to_0to360_range(201.5643 + 385.81693528 * k + 0.0107582 * t2 + 0.00001238 * t3 - 0.000000058 * t4);
    mdash = aacoordinatetransformation::degrees_to_radians(mdash);
    let mut f = aacoordinatetransformation::map_to_0to360_range(160.7108 + 390.67050284 * k - 0.0016118 * t2 - 0.00000227 * t3 + 0.000000011 * t4);
    f = aacoordinatetransformation::degrees_to_radians(f);
    let mut omega = aacoordinatetransformation::map_to_0to360_range(124.7746 - 1.56375588 * k + 0.0020672 * t2 + 0.00000215 * t3);
    omega = aacoordinatetransformation::degrees_to_radians(omega);
    let mut a1 = aacoordinatetransformation::map_to_0to360_range(299.77 + 0.107408 * k - 0.009173 * t2);
    a1 = aacoordinatetransformation::degrees_to_radians(a1);
    let mut a2 = aacoordinatetransformation::map_to_0to360_range(251.88 + 0.016321 * k);
    a2 = aacoordinatetransformation::degrees_to_radians(a2);
    let mut a3 = aacoordinatetransformation::map_to_0to360_range(251.83 + 26.651886 * k);
    a3 = aacoordinatetransformation::degrees_to_radians(a3);
    let mut a4 = aacoordinatetransformation::map_to_0to360_range(349.42 + 36.412478 * k);
    a4 = aacoordinatetransformation::degrees_to_radians(a4);
    let mut a5 = aacoordinatetransformation::map_to_0to360_range(84.66 + 18.206239 * k);
    a5 = aacoordinatetransformation::degrees_to_radians(a5);
    let mut a6 = aacoordinatetransformation::map_to_0to360_range(141.74 + 53.303771 * k);
    a6 = aacoordinatetransformation::degrees_to_radians(a6);
    let mut a7 = aacoordinatetransformation::map_to_0to360_range(207.14 + 2.453732 * k);
    a7 = aacoordinatetransformation::degrees_to_radians(a7);
    let mut a8 = aacoordinatetransformation::map_to_0to360_range(154.84 + 7.306860 * k);
    a8 = aacoordinatetransformation::degrees_to_radians(a8);
    let mut a9 = aacoordinatetransformation::map_to_0to360_range(34.52 + 27.261239 * k);
    a9 = aacoordinatetransformation::degrees_to_radians(a9);
    let mut a10 = aacoordinatetransformation::map_to_0to360_range(207.19 + 0.121824 * k);
    a10 = aacoordinatetransformation::degrees_to_radians(a10);
    let mut a11 = aacoordinatetransformation::map_to_0to360_range(291.34 + 1.844379 * k);
    a11 = aacoordinatetransformation::degrees_to_radians(a11);
    let mut a12 = aacoordinatetransformation::map_to_0to360_range(161.72 + 24.198154 * k);
    a12 = aacoordinatetransformation::degrees_to_radians(a12);
    let mut a13 = aacoordinatetransformation::map_to_0to360_range(239.56 + 25.513099 * k);
    a13 = aacoordinatetransformation::degrees_to_radians(a13);
    let mut a14 = aacoordinatetransformation::map_to_0to360_range(331.55 + 3.592518 * k);
    a14 = aacoordinatetransformation::degrees_to_radians(a14);

    //convert to radians
    let mut kfrac = f64::fract(k);
    if kfrac < 0.0 {
        kfrac = 1.0 + kfrac;
    }
    if kfrac == 0.0 { //New Moon
        let delta_jd = -0.40720 * f64::sin(mdash) +
            0.17241 * e * f64::sin(m) +
            0.01608 * f64::sin(2.0 * mdash) +
            0.01039 * f64::sin(2.0 * f) +
            0.00739 * e * f64::sin(mdash - m) +
            -0.00514 * e * f64::sin(mdash + m) +
            0.00208 * e2 * f64::sin(2.0 * m) +
            -0.00111 * f64::sin(mdash - 2.0 * f) +
            -0.00057 * f64::sin(mdash + 2.0 * f) +
            0.00056 * e * f64::sin(2.0 * mdash + m) +
            -0.00042 * f64::sin(3.0 * mdash) +
            0.00042 * e * f64::sin(m + 2.0 * f) +
            0.00038 * e * f64::sin(m - 2.0 * f) +
            -0.00024 * e * f64::sin(2.0 * mdash - m) +
            -0.00017 * f64::sin(omega) +
            -0.00007 * f64::sin(mdash + 2.0 * m) +
            0.00004 * f64::sin(2.0 * mdash - 2.0 * f) +
            0.00004 * f64::sin(3.0 * m) +
            0.00003 * f64::sin(mdash + m - 2.0 * f) +
            0.00003 * f64::sin(2.0 * mdash + 2.0 * f) +
            -0.00003 * f64::sin(mdash + m + 2.0 * f) +
            0.00003 * f64::sin(mdash - m + 2.0 * f) +
            -0.00002 * f64::sin(mdash - m - 2.0 * f) +
            -0.00002 * f64::sin(3.0 * mdash + m) +
            0.00002 * f64::sin(4.0 * mdash);
        jd += delta_jd;
    } else if (kfrac == 0.25) || (kfrac == 0.75) { //First Quarter or Last Quarter
        let delta_jd = -0.62801 * f64::sin(mdash) +
            0.17172 * e * f64::sin(m) +
            -0.01183 * e * f64::sin(mdash + m) +
            0.00862 * f64::sin(2.0 * mdash) +
            0.00804 * f64::sin(2.0 * f) +
            0.00454 * e * f64::sin(mdash - m) +
            0.00204 * e2 * f64::sin(2.0 * m) +
            -0.00180 * f64::sin(mdash - 2.0 * f) +
            -0.00070 * f64::sin(mdash + 2.0 * f) +
            -0.00040 * f64::sin(3.0 * mdash) +
            -0.00034 * e * f64::sin(2.0 * mdash - m) +
            0.00032 * e * f64::sin(m + 2.0 * f) +
            0.00032 * e * f64::sin(m - 2.0 * f) +
            -0.00028 * e2 * f64::sin(mdash + 2.0 * m) +
            0.00027 * e * f64::sin(2.0 * mdash + m) +
            -0.00017 * f64::sin(omega) +
            -0.00005 * f64::sin(mdash - m - 2.0 * f) +
            0.00004 * f64::sin(2.0 * mdash + 2.0 * f) +
            -0.00004 * f64::sin(mdash + m + 2.0 * f) +
            0.00004 * f64::sin(mdash - 2.0 * m) +
            0.00003 * f64::sin(mdash + m - 2.0 * f) +
            0.00003 * f64::sin(3.0 * m) +
            0.00002 * f64::sin(2.0 * mdash - 2.0 * f) +
            0.00002 * f64::sin(mdash - m + 2.0 * f) +
            -0.00002 * f64::sin(3.0 * mdash + m);
        jd += delta_jd;

        let w = 0.00306 - 0.00038 * e * f64::cos(m) + 0.00026 * f64::cos(mdash) - 0.00002 * f64::cos(mdash - m) + 0.00002 * f64::cos(mdash + m) + 0.00002 * f64::cos(2.0 * f);
        if kfrac == 0.25 { //First quarter
            jd += w;
        } else {
            jd -= w;
        }
    } else if kfrac == 0.5 { //Full Moon
        let delta_jd = -0.40614 * f64::sin(mdash) +
            0.17302 * e * f64::sin(m) +
            0.01614 * f64::sin(2.0 * mdash) +
            0.01043 * f64::sin(2.0 * f) +
            0.00734 * e * f64::sin(mdash - m) +
            -0.00514 * e * f64::sin(mdash + m) +
            0.00209 * e2 * f64::sin(2.0 * m) +
            -0.00111 * f64::sin(mdash - 2.0 * f) +
            -0.00057 * f64::sin(mdash + 2.0 * f) +
            0.00056 * e * f64::sin(2.0 * mdash + m) +
            -0.00042 * f64::sin(3.0 * mdash) +
            0.00042 * e * f64::sin(m + 2.0 * f) +
            0.00038 * e * f64::sin(m - 2.0 * f) +
            -0.00024 * e * f64::sin(2.0 * mdash - m) +
            -0.00017 * f64::sin(omega) +
            -0.00007 * f64::sin(mdash + 2.0 * m) +
            0.00004 * f64::sin(2.0 * mdash - 2.0 * f) +
            0.00004 * f64::sin(3.0 * m) +
            0.00003 * f64::sin(mdash + m - 2.0 * f) +
            0.00003 * f64::sin(2.0 * mdash + 2.0 * f) +
            -0.00003 * f64::sin(mdash + m + 2.0 * f) +
            0.00003 * f64::sin(mdash - m + 2.0 * f) +
            -0.00002 * f64::sin(mdash - m - 2.0 * f) +
            -0.00002 * f64::sin(3.0 * mdash + m) +
            0.00002 * f64::sin(4.0 * mdash);
        jd += delta_jd;
    } else {
        assert!(false);
    }

    //Additional corrections for all phases
    let delta_jd2 = 0.000325 * f64::sin(a1) +
        0.000165 * f64::sin(a2) +
        0.000164 * f64::sin(a3) +
        0.000126 * f64::sin(a4) +
        0.000110 * f64::sin(a5) +
        0.000062 * f64::sin(a6) +
        0.000060 * f64::sin(a7) +
        0.000056 * f64::sin(a8) +
        0.000047 * f64::sin(a9) +
        0.000042 * f64::sin(a10) +
        0.000040 * f64::sin(a11) +
        0.000037 * f64::sin(a12) +
        0.000035 * f64::sin(a13) +
        0.000023 * f64::sin(a14);
    jd + delta_jd2
}

