use super::aavsop87;
use super::aacoordinatetransformation;
use super::aavsop87d_mer;

const G_L0MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 38] =
    [
        aavsop87::VSOP87Coefficient { a: 440250710.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 40989415.0, b: 1.48302034, c: 26087.90314157 },
        aavsop87::VSOP87Coefficient { a: 5046294.0, b: 4.4778549, c: 52175.8062831 },
        aavsop87::VSOP87Coefficient { a: 855347.0, b: 1.165203, c: 78263.709425 },
        aavsop87::VSOP87Coefficient { a: 165590.0, b: 4.119692, c: 104351.612566 },
        aavsop87::VSOP87Coefficient { a: 34562.0, b: 0.77931, c: 130439.51571 },
        aavsop87::VSOP87Coefficient { a: 7583.0, b: 3.7135, c: 156527.4188 },
        aavsop87::VSOP87Coefficient { a: 3560.0, b: 1.5120, c: 1109.3786 },
        aavsop87::VSOP87Coefficient { a: 1803.0, b: 4.1033, c: 5661.3320 },
        aavsop87::VSOP87Coefficient { a: 1726.0, b: 0.3583, c: 182615.3220 },
        aavsop87::VSOP87Coefficient { a: 1590.0, b: 2.9951, c: 25028.5212 },
        aavsop87::VSOP87Coefficient { a: 1365.0, b: 4.5992, c: 27197.2817 },
        aavsop87::VSOP87Coefficient { a: 1017.0, b: 0.8803, c: 31749.2352 },
        aavsop87::VSOP87Coefficient { a: 714.0, b: 1.541, c: 24978.525 },
        aavsop87::VSOP87Coefficient { a: 644.0, b: 5.303, c: 21535.950 },
        aavsop87::VSOP87Coefficient { a: 451.0, b: 6.050, c: 51116.424 },
        aavsop87::VSOP87Coefficient { a: 404.0, b: 3.282, c: 208703.225 },
        aavsop87::VSOP87Coefficient { a: 352.0, b: 5.242, c: 20426.571 },
        aavsop87::VSOP87Coefficient { a: 345.0, b: 2.792, c: 15874.618 },
        aavsop87::VSOP87Coefficient { a: 343.0, b: 5.765, c: 955.600 },
        aavsop87::VSOP87Coefficient { a: 339.0, b: 5.863, c: 25558.212 },
        aavsop87::VSOP87Coefficient { a: 325.0, b: 1.337, c: 53285.185 },
        aavsop87::VSOP87Coefficient { a: 273.0, b: 2.495, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 264.0, b: 3.917, c: 57837.138 },
        aavsop87::VSOP87Coefficient { a: 260.0, b: 0.987, c: 4551.953 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 0.113, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 235.0, b: 0.267, c: 11322.664 },
        aavsop87::VSOP87Coefficient { a: 217.0, b: 0.660, c: 13521.751 },
        aavsop87::VSOP87Coefficient { a: 209.0, b: 2.092, c: 47623.853 },
        aavsop87::VSOP87Coefficient { a: 183.0, b: 2.629, c: 27043.503 },
        aavsop87::VSOP87Coefficient { a: 182.0, b: 2.434, c: 25661.305 },
        aavsop87::VSOP87Coefficient { a: 176.0, b: 4.536, c: 51066.428 },
        aavsop87::VSOP87Coefficient { a: 173.0, b: 2.452, c: 24498.830 },
        aavsop87::VSOP87Coefficient { a: 142.0, b: 3.360, c: 37410.567 },
        aavsop87::VSOP87Coefficient { a: 138.0, b: 0.291, c: 10213.286 },
        aavsop87::VSOP87Coefficient { a: 125.0, b: 3.721, c: 39609.655 },
        aavsop87::VSOP87Coefficient { a: 118.0, b: 2.781, c: 77204.327 },
        aavsop87::VSOP87Coefficient { a: 106.0, b: 4.206, c: 19804.827 }
    ];

const G_L1MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 16] =
    [
        aavsop87::VSOP87Coefficient { a: 2608814706223.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1126008.0, b: 6.2170397, c: 26087.9031416 },
        aavsop87::VSOP87Coefficient { a: 303471.0, b: 3.055655, c: 52175.806283 },
        aavsop87::VSOP87Coefficient { a: 80538.0, b: 6.10455, c: 78263.70942 },
        aavsop87::VSOP87Coefficient { a: 21245.0, b: 2.83532, c: 104351.61257 },
        aavsop87::VSOP87Coefficient { a: 5592.0, b: 5.8268, c: 130439.5157 },
        aavsop87::VSOP87Coefficient { a: 1472.0, b: 2.5185, c: 156527.4188 },
        aavsop87::VSOP87Coefficient { a: 388.0, b: 5.480, c: 182615.322 },
        aavsop87::VSOP87Coefficient { a: 352.0, b: 3.052, c: 1109.379 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 2.149, c: 208703.225 },
        aavsop87::VSOP87Coefficient { a: 94.0, b: 6.12, c: 27197.28 },
        aavsop87::VSOP87Coefficient { a: 91.0, b: 0.00, c: 24978.52 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 5.62, c: 5661.33 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 4.57, c: 25028.52 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 3.04, c: 51066.43 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.09, c: 234791.13 }
    ];

const G_L2MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 10] =
    [
        aavsop87::VSOP87Coefficient { a: 53050.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 16904.0, b: 4.69072, c: 26087.90314 },
        aavsop87::VSOP87Coefficient { a: 7397.0, b: 1.3474, c: 52175.8063 },
        aavsop87::VSOP87Coefficient { a: 3018.0, b: 4.4564, c: 78263.7094 },
        aavsop87::VSOP87Coefficient { a: 1107.0, b: 1.2623, c: 104351.6126 },
        aavsop87::VSOP87Coefficient { a: 378.0, b: 4.320, c: 130439.516 },
        aavsop87::VSOP87Coefficient { a: 123.0, b: 1.069, c: 156527.419 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 4.08, c: 182615.32 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 4.63, c: 1109.38 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 0.79, c: 208703.23 }
    ];

const G_L3MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 8] =
    [
        aavsop87::VSOP87Coefficient { a: 188.0, b: 0.035, c: 52175.806 },
        aavsop87::VSOP87Coefficient { a: 142.0, b: 3.125, c: 26087.903 },
        aavsop87::VSOP87Coefficient { a: 97.0, b: 3.00, c: 78263.71 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 6.02, c: 104351.61 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 2.78, c: 130439.52 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 5.82, c: 156527.42 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.57, c: 182615.32 }
    ];

const G_L4MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 6] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.03, c: 26087.90 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 1.42, c: 78263.71 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.50, c: 52175.81 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.50, c: 104351.61 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.27, c: 130439.52 }
    ];

const G_L5MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_B0MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 14] =
    [
        aavsop87::VSOP87Coefficient { a: 11737529.0, b: 1.98357499, c: 26087.90314157 },
        aavsop87::VSOP87Coefficient { a: 2388077.0, b: 5.0373896, c: 52175.8062831 },
        aavsop87::VSOP87Coefficient { a: 1222840.0, b: 3.1415927, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 543252.0, b: 1.796444, c: 78263.709425 },
        aavsop87::VSOP87Coefficient { a: 129779.0, b: 4.832325, c: 104351.612566 },
        aavsop87::VSOP87Coefficient { a: 31867.0, b: 1.58088, c: 130439.51571 },
        aavsop87::VSOP87Coefficient { a: 7963.0, b: 4.6097, c: 156527.4188 },
        aavsop87::VSOP87Coefficient { a: 2014.0, b: 1.3532, c: 182615.3220 },
        aavsop87::VSOP87Coefficient { a: 514.0, b: 4.378, c: 208703.225 },
        aavsop87::VSOP87Coefficient { a: 209.0, b: 2.020, c: 24978.525 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 4.918, c: 27197.282 },
        aavsop87::VSOP87Coefficient { a: 132.0, b: 1.119, c: 234791.128 },
        aavsop87::VSOP87Coefficient { a: 121.0, b: 1.813, c: 53285.185 },
        aavsop87::VSOP87Coefficient { a: 100.0, b: 5.657, c: 20426.571 }
    ];

const G_B1MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 11] =
    [
        aavsop87::VSOP87Coefficient { a: 429151.0, b: 3.501698, c: 26087.903142 },
        aavsop87::VSOP87Coefficient { a: 146234.0, b: 3.141593, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 22675.0, b: 0.01515, c: 52175.80628 },
        aavsop87::VSOP87Coefficient { a: 10895.0, b: 0.48540, c: 78263.70942 },
        aavsop87::VSOP87Coefficient { a: 6353.0, b: 3.4294, c: 104351.6126 },
        aavsop87::VSOP87Coefficient { a: 2496.0, b: 0.1605, c: 130439.5157 },
        aavsop87::VSOP87Coefficient { a: 860.0, b: 3.185, c: 156527.419 },
        aavsop87::VSOP87Coefficient { a: 278.0, b: 6.210, c: 182615.322 },
        aavsop87::VSOP87Coefficient { a: 86.0, b: 2.95, c: 208703.23 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 0.29, c: 27197.28 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 5.98, c: 234791.13 }
    ];

const G_B2MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 9] =
    [
        aavsop87::VSOP87Coefficient { a: 11831.0, b: 4.79066, c: 26087.90314 },
        aavsop87::VSOP87Coefficient { a: 1914.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1045.0, b: 1.2122, c: 52175.8063 },
        aavsop87::VSOP87Coefficient { a: 266.0, b: 4.434, c: 78263.709 },
        aavsop87::VSOP87Coefficient { a: 170.0, b: 1.623, c: 104351.613 },
        aavsop87::VSOP87Coefficient { a: 96.0, b: 4.80, c: 130439.52 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 1.61, c: 156527.42 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 4.67, c: 182615.32 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 1.43, c: 208703.23 }
    ];

const G_B3MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 235.0, b: 0.354, c: 26087.903 },
        aavsop87::VSOP87Coefficient { a: 161.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 4.36, c: 52175.81 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.51, c: 78263.71 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 6.14, c: 104351.61 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.12, c: 130439.52 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 6.27, c: 156527.42 }
    ];

const G_B4MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 4.0, b: 1.75, c: 26087.90 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_R0MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 13] =
    [
        aavsop87::VSOP87Coefficient { a: 39528272.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 7834132.0, b: 6.1923372, c: 26087.9031416 },
        aavsop87::VSOP87Coefficient { a: 795526.0, b: 2.959897, c: 52175.806283 },
        aavsop87::VSOP87Coefficient { a: 121282.0, b: 6.010642, c: 78263.709425 },
        aavsop87::VSOP87Coefficient { a: 21922.0, b: 2.77820, c: 104351.61257 },
        aavsop87::VSOP87Coefficient { a: 4354.0, b: 5.8289, c: 130439.5157 },
        aavsop87::VSOP87Coefficient { a: 918.0, b: 2.597, c: 156527.419 },
        aavsop87::VSOP87Coefficient { a: 290.0, b: 1.424, c: 25028.521 },
        aavsop87::VSOP87Coefficient { a: 260.0, b: 3.028, c: 27197.282 },
        aavsop87::VSOP87Coefficient { a: 202.0, b: 5.647, c: 182615.322 },
        aavsop87::VSOP87Coefficient { a: 201.0, b: 5.592, c: 31749.235 },
        aavsop87::VSOP87Coefficient { a: 142.0, b: 6.253, c: 24978.525 },
        aavsop87::VSOP87Coefficient { a: 100.0, b: 3.734, c: 21535.950 }
    ];

const G_R1MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 8] =
    [
        aavsop87::VSOP87Coefficient { a: 217348.0, b: 4.656172, c: 26087.903142 },
        aavsop87::VSOP87Coefficient { a: 44142.0, b: 1.42386, c: 52175.80628 },
        aavsop87::VSOP87Coefficient { a: 10094.0, b: 4.47466, c: 78263.70942 },
        aavsop87::VSOP87Coefficient { a: 2433.0, b: 1.2423, c: 104351.6126 },
        aavsop87::VSOP87Coefficient { a: 1624.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 604.0, b: 4.293, c: 130439.516 },
        aavsop87::VSOP87Coefficient { a: 153.0, b: 1.061, c: 156527.419 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 4.11, c: 182615.32 }
    ];

const G_R2MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 3118.0, b: 3.0823, c: 26087.9031 },
        aavsop87::VSOP87Coefficient { a: 1245.0, b: 6.1518, c: 52175.8063 },
        aavsop87::VSOP87Coefficient { a: 425.0, b: 2.926, c: 78263.709 },
        aavsop87::VSOP87Coefficient { a: 136.0, b: 5.980, c: 104351.613 },
        aavsop87::VSOP87Coefficient { a: 42.0, b: 2.75, c: 130439.52 },
        aavsop87::VSOP87Coefficient { a: 22.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 5.80, c: 156527.42 }
    ];

const G_R3MERCURY_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 5] =
    [
        aavsop87::VSOP87Coefficient { a: 33.0, b: 1.68, c: 26087.90 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 4.63, c: 52175.81 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 1.39, c: 78263.71 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 4.44, c: 104351.61 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 1.21, c: 130439.52 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_mer::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate l0
    let n_l0coefficients = G_L0MERCURY_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0MERCURY_COEFFICIENTS[i].a * f64::cos(G_L0MERCURY_COEFFICIENTS[i].b + G_L0MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1MERCURY_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1MERCURY_COEFFICIENTS[i].a * f64::cos(G_L1MERCURY_COEFFICIENTS[i].b + G_L1MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2MERCURY_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2MERCURY_COEFFICIENTS[i].a * f64::cos(G_L2MERCURY_COEFFICIENTS[i].b + G_L2MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3MERCURY_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3MERCURY_COEFFICIENTS[i].a * f64::cos(G_L3MERCURY_COEFFICIENTS[i].b + G_L3MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4MERCURY_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4MERCURY_COEFFICIENTS[i].a * f64::cos(G_L4MERCURY_COEFFICIENTS[i].b + G_L4MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate l5
    let n_l5coefficients = G_L5MERCURY_COEFFICIENTS.len();
    let mut l5 = 0.0f64;
    for i in 0..n_l5coefficients {
        l5 += G_L5MERCURY_COEFFICIENTS[i].a * f64::cos(G_L5MERCURY_COEFFICIENTS[i].b + G_L5MERCURY_COEFFICIENTS[i].c * rho);
    }

    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_mer::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate b0
    let n_b0coefficients = G_B0MERCURY_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0MERCURY_COEFFICIENTS[i].a * f64::cos(G_B0MERCURY_COEFFICIENTS[i].b + G_B0MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1MERCURY_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1MERCURY_COEFFICIENTS[i].a * f64::cos(G_B1MERCURY_COEFFICIENTS[i].b + G_B1MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2MERCURY_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2MERCURY_COEFFICIENTS[i].a * f64::cos(G_B2MERCURY_COEFFICIENTS[i].b + G_B2MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3MERCURY_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3MERCURY_COEFFICIENTS[i].a * f64::cos(G_B3MERCURY_COEFFICIENTS[i].b + G_B3MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4MERCURY_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4MERCURY_COEFFICIENTS[i].a * f64::cos(G_B4MERCURY_COEFFICIENTS[i].b + G_B4MERCURY_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_mer::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;

    //Calculate r0
    let n_r0coefficients = G_R0MERCURY_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0MERCURY_COEFFICIENTS[i].a * f64::cos(G_R0MERCURY_COEFFICIENTS[i].b + G_R0MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1MERCURY_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1MERCURY_COEFFICIENTS[i].a * f64::cos(G_R1MERCURY_COEFFICIENTS[i].b + G_R1MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2MERCURY_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2MERCURY_COEFFICIENTS[i].a * f64::cos(G_R2MERCURY_COEFFICIENTS[i].b + G_R2MERCURY_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3MERCURY_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3MERCURY_COEFFICIENTS[i].a * f64::cos(G_R3MERCURY_COEFFICIENTS[i].b + G_R3MERCURY_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed) / 100000000.0
}
