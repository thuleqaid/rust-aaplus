use super::aavsop87;
use super::aacoordinatetransformation;
use super::aavsop87d_nep;

const G_L0NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 38] =
    [
        aavsop87::VSOP87Coefficient { a: 531188633.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1798476.0, b: 2.9010127, c: 38.1330356 },
        aavsop87::VSOP87Coefficient { a: 1019728.0, b: 0.4858092, c: 1.4844727 },
        aavsop87::VSOP87Coefficient { a: 124532.0, b: 4.830081, c: 36.648563 },
        aavsop87::VSOP87Coefficient { a: 42064.0, b: 5.41055, c: 2.96895 },
        aavsop87::VSOP87Coefficient { a: 37715.0, b: 6.09222, c: 35.16409 },
        aavsop87::VSOP87Coefficient { a: 33785.0, b: 1.24489, c: 76.26607 },
        aavsop87::VSOP87Coefficient { a: 16483.0, b: 0.00008, c: 491.55793 },
        aavsop87::VSOP87Coefficient { a: 9199.0, b: 4.9375, c: 39.6175 },
        aavsop87::VSOP87Coefficient { a: 8994.0, b: 0.2746, c: 175.1661 },
        aavsop87::VSOP87Coefficient { a: 4216.0, b: 1.9871, c: 73.2971 },
        aavsop87::VSOP87Coefficient { a: 3365.0, b: 1.0359, c: 33.6796 },
        aavsop87::VSOP87Coefficient { a: 2285.0, b: 4.2061, c: 4.4534 },
        aavsop87::VSOP87Coefficient { a: 1434.0, b: 2.7834, c: 74.7816 },
        aavsop87::VSOP87Coefficient { a: 900.0, b: 2.076, c: 109.946 },
        aavsop87::VSOP87Coefficient { a: 745.0, b: 3.190, c: 71.813 },
        aavsop87::VSOP87Coefficient { a: 506.0, b: 5.748, c: 114.399 },
        aavsop87::VSOP87Coefficient { a: 400.0, b: 0.350, c: 1021.249 },
        aavsop87::VSOP87Coefficient { a: 345.0, b: 3.462, c: 41.102 },
        aavsop87::VSOP87Coefficient { a: 340.0, b: 3.304, c: 77.751 },
        aavsop87::VSOP87Coefficient { a: 323.0, b: 2.248, c: 32.195 },
        aavsop87::VSOP87Coefficient { a: 306.0, b: 0.497, c: 0.521 },
        aavsop87::VSOP87Coefficient { a: 287.0, b: 4.505, c: 0.048 },
        aavsop87::VSOP87Coefficient { a: 282.0, b: 2.246, c: 146.594 },
        aavsop87::VSOP87Coefficient { a: 267.0, b: 4.889, c: 0.963 },
        aavsop87::VSOP87Coefficient { a: 252.0, b: 5.782, c: 388.465 },
        aavsop87::VSOP87Coefficient { a: 245.0, b: 1.247, c: 9.561 },
        aavsop87::VSOP87Coefficient { a: 233.0, b: 2.505, c: 137.033 },
        aavsop87::VSOP87Coefficient { a: 227.0, b: 1.797, c: 453.425 },
        aavsop87::VSOP87Coefficient { a: 170.0, b: 3.324, c: 108.461 },
        aavsop87::VSOP87Coefficient { a: 151.0, b: 2.192, c: 33.940 },
        aavsop87::VSOP87Coefficient { a: 150.0, b: 2.997, c: 5.938 },
        aavsop87::VSOP87Coefficient { a: 148.0, b: 0.859, c: 111.430 },
        aavsop87::VSOP87Coefficient { a: 119.0, b: 3.677, c: 2.448 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 2.416, c: 183.243 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 0.041, c: 0.261 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 4.404, c: 70.328 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 5.705, c: 0.112 }
    ];

const G_L1NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 18] =
    [
        aavsop87::VSOP87Coefficient { a: 3837687717.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 16604.0, b: 4.86319, c: 1.48447 },
        aavsop87::VSOP87Coefficient { a: 15807.0, b: 2.27923, c: 38.13304 },
        aavsop87::VSOP87Coefficient { a: 3335.0, b: 3.6820, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 1306.0, b: 3.6732, c: 2.9689 },
        aavsop87::VSOP87Coefficient { a: 605.0, b: 1.505, c: 35.164 },
        aavsop87::VSOP87Coefficient { a: 179.0, b: 3.453, c: 39.618 },
        aavsop87::VSOP87Coefficient { a: 107.0, b: 2.451, c: 4.453 },
        aavsop87::VSOP87Coefficient { a: 106.0, b: 2.755, c: 33.680 },
        aavsop87::VSOP87Coefficient { a: 73.0, b: 5.49, c: 36.65 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 1.86, c: 114.40 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 5.22, c: 0.52 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 4.52, c: 74.78 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 5.90, c: 77.75 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 3.67, c: 388.47 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 5.17, c: 9.56 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 5.17, c: 2.45 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 5.25, c: 168.05 }
    ];

const G_L2NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 53893.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 296.0, b: 1.855, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 281.0, b: 1.191, c: 38.133 },
        aavsop87::VSOP87Coefficient { a: 270.0, b: 5.721, c: 76.266 },
        aavsop87::VSOP87Coefficient { a: 23.0, b: 1.21, c: 2.97 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 4.43, c: 35.16 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 0.54, c: 2.45 }
    ];

const G_L3NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 31.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 1.35, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 6.04, c: 1.48 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 6.11, c: 38.13 }
    ];

const G_L4NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.142, c: 0.0 }
    ];

const G_B0NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 17] =
    [
        aavsop87::VSOP87Coefficient { a: 3088623.0, b: 1.4410437, c: 38.1330356 },
        aavsop87::VSOP87Coefficient { a: 27780.0, b: 5.91272, c: 76.26607 },
        aavsop87::VSOP87Coefficient { a: 27624.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 15448.0, b: 3.50877, c: 39.61751 },
        aavsop87::VSOP87Coefficient { a: 15355.0, b: 2.52124, c: 36.64856 },
        aavsop87::VSOP87Coefficient { a: 2000.0, b: 1.5100, c: 74.7816 },
        aavsop87::VSOP87Coefficient { a: 1968.0, b: 4.3778, c: 1.4845 },
        aavsop87::VSOP87Coefficient { a: 1015.0, b: 3.2156, c: 35.1641 },
        aavsop87::VSOP87Coefficient { a: 606.0, b: 2.802, c: 73.297 },
        aavsop87::VSOP87Coefficient { a: 595.0, b: 2.129, c: 41.102 },
        aavsop87::VSOP87Coefficient { a: 589.0, b: 3.187, c: 2.969 },
        aavsop87::VSOP87Coefficient { a: 402.0, b: 4.169, c: 114.399 },
        aavsop87::VSOP87Coefficient { a: 280.0, b: 1.682, c: 77.751 },
        aavsop87::VSOP87Coefficient { a: 262.0, b: 3.767, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 254.0, b: 3.271, c: 453.425 },
        aavsop87::VSOP87Coefficient { a: 206.0, b: 4.257, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 140.0, b: 3.530, c: 137.033 }
    ];

const G_B1NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 13] =
    [
        aavsop87::VSOP87Coefficient { a: 227279.0, b: 3.807931, c: 38.133036 },
        aavsop87::VSOP87Coefficient { a: 1803.0, b: 1.9758, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 1433.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1386.0, b: 4.8256, c: 36.6486 },
        aavsop87::VSOP87Coefficient { a: 1073.0, b: 6.0805, c: 39.6175 },
        aavsop87::VSOP87Coefficient { a: 148.0, b: 3.858, c: 74.782 },
        aavsop87::VSOP87Coefficient { a: 136.0, b: 0.478, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 70.0, b: 6.19, c: 35.16 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 5.05, c: 73.30 },
        aavsop87::VSOP87Coefficient { a: 43.0, b: 0.31, c: 114.40 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 4.89, c: 41.10 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 5.76, c: 2.97 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 5.22, c: 213.30 }
    ];

const G_B2NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 6] =
    [
        aavsop87::VSOP87Coefficient { a: 9691.0, b: 5.5712, c: 38.1330 },
        aavsop87::VSOP87Coefficient { a: 79.0, b: 3.63, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 0.45, c: 36.65 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 1.61, c: 39.62 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.61, c: 74.78 }
    ];

const G_B3NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 273.0, b: 1.017, c: 38.133 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.37, c: 36.65 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.33, c: 76.27 }
    ];

const G_B4NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.67, c: 38.13 }
    ];

const G_R0NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 32] =
    [
        aavsop87::VSOP87Coefficient { a: 3007013206.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 27062259.0, b: 1.32999459, c: 38.13303564 },
        aavsop87::VSOP87Coefficient { a: 1691764.0, b: 3.2518614, c: 36.6485629 },
        aavsop87::VSOP87Coefficient { a: 807831.0, b: 5.185928, c: 1.484473 },
        aavsop87::VSOP87Coefficient { a: 537761.0, b: 4.521139, c: 35.164090 },
        aavsop87::VSOP87Coefficient { a: 495726.0, b: 1.571057, c: 491.557929 },
        aavsop87::VSOP87Coefficient { a: 274572.0, b: 1.845523, c: 175.166060 },
        aavsop87::VSOP87Coefficient { a: 135134.0, b: 3.372206, c: 39.617508 },
        aavsop87::VSOP87Coefficient { a: 121802.0, b: 5.797544, c: 76.266071 },
        aavsop87::VSOP87Coefficient { a: 100895.0, b: 0.377027, c: 73.297126 },
        aavsop87::VSOP87Coefficient { a: 69792.0, b: 3.79617, c: 2.96895 },
        aavsop87::VSOP87Coefficient { a: 46688.0, b: 5.74938, c: 33.67962 },
        aavsop87::VSOP87Coefficient { a: 24594.0, b: 0.50802, c: 109.94569 },
        aavsop87::VSOP87Coefficient { a: 16939.0, b: 1.59422, c: 71.81265 },
        aavsop87::VSOP87Coefficient { a: 14230.0, b: 1.07786, c: 74.78160 },
        aavsop87::VSOP87Coefficient { a: 12012.0, b: 1.92062, c: 1021.24889 },
        aavsop87::VSOP87Coefficient { a: 8395.0, b: 0.6782, c: 146.5943 },
        aavsop87::VSOP87Coefficient { a: 7572.0, b: 1.0715, c: 388.4652 },
        aavsop87::VSOP87Coefficient { a: 5721.0, b: 2.5906, c: 4.4534 },
        aavsop87::VSOP87Coefficient { a: 4840.0, b: 1.9069, c: 41.1020 },
        aavsop87::VSOP87Coefficient { a: 4483.0, b: 2.9057, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 4421.0, b: 1.7499, c: 108.4612 },
        aavsop87::VSOP87Coefficient { a: 4354.0, b: 0.6799, c: 32.1951 },
        aavsop87::VSOP87Coefficient { a: 4270.0, b: 3.4134, c: 453.4249 },
        aavsop87::VSOP87Coefficient { a: 3381.0, b: 0.8481, c: 183.2428 },
        aavsop87::VSOP87Coefficient { a: 2881.0, b: 1.9860, c: 137.0330 },
        aavsop87::VSOP87Coefficient { a: 2879.0, b: 3.6742, c: 350.3321 },
        aavsop87::VSOP87Coefficient { a: 2636.0, b: 3.0976, c: 213.2991 },
        aavsop87::VSOP87Coefficient { a: 2530.0, b: 5.7984, c: 490.0735 },
        aavsop87::VSOP87Coefficient { a: 2523.0, b: 0.4863, c: 493.0424 },
        aavsop87::VSOP87Coefficient { a: 2306.0, b: 2.8096, c: 70.3282 },
        aavsop87::VSOP87Coefficient { a: 2087.0, b: 0.6186, c: 33.9402 }
    ];

const G_R1NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 15] =
    [
        aavsop87::VSOP87Coefficient { a: 236339.0, b: 0.704980, c: 38.133036 },
        aavsop87::VSOP87Coefficient { a: 13220.0, b: 3.32015, c: 1.48447 },
        aavsop87::VSOP87Coefficient { a: 8622.0, b: 6.2163, c: 35.1641 },
        aavsop87::VSOP87Coefficient { a: 2702.0, b: 1.8814, c: 39.6175 },
        aavsop87::VSOP87Coefficient { a: 2155.0, b: 2.0943, c: 2.9689 },
        aavsop87::VSOP87Coefficient { a: 2153.0, b: 5.1687, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 1603.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1464.0, b: 1.1842, c: 33.6796 },
        aavsop87::VSOP87Coefficient { a: 1136.0, b: 3.9189, c: 36.6486 },
        aavsop87::VSOP87Coefficient { a: 898.0, b: 5.241, c: 388.465 },
        aavsop87::VSOP87Coefficient { a: 790.0, b: 0.533, c: 168.053 },
        aavsop87::VSOP87Coefficient { a: 760.0, b: 0.021, c: 182.280 },
        aavsop87::VSOP87Coefficient { a: 607.0, b: 1.077, c: 1021.249 },
        aavsop87::VSOP87Coefficient { a: 572.0, b: 3.401, c: 484.444 },
        aavsop87::VSOP87Coefficient { a: 561.0, b: 2.887, c: 498.671 }
    ];

const G_R2NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 5] =
    [
        aavsop87::VSOP87Coefficient { a: 4247.0, b: 5.8991, c: 38.1330 },
        aavsop87::VSOP87Coefficient { a: 218.0, b: 0.346, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 163.0, b: 2.239, c: 168.053 },
        aavsop87::VSOP87Coefficient { a: 156.0, b: 4.594, c: 182.280 },
        aavsop87::VSOP87Coefficient { a: 127.0, b: 2.848, c: 35.164 }
    ];

const G_R3NEPTUNE_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 166.0, b: 4.552, c: 38.133 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_nep::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate l0
    let n_l0coefficients = G_L0NEPTUNE_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_L0NEPTUNE_COEFFICIENTS[i].b + G_L0NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1NEPTUNE_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_L1NEPTUNE_COEFFICIENTS[i].b + G_L1NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2NEPTUNE_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_L2NEPTUNE_COEFFICIENTS[i].b + G_L2NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3NEPTUNE_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_L3NEPTUNE_COEFFICIENTS[i].b + G_L3NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4NEPTUNE_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_L4NEPTUNE_COEFFICIENTS[i].b + G_L4NEPTUNE_COEFFICIENTS[i].c * rho);
    }


    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_nep::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate b0
    let n_b0coefficients = G_B0NEPTUNE_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_B0NEPTUNE_COEFFICIENTS[i].b + G_B0NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1NEPTUNE_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_B1NEPTUNE_COEFFICIENTS[i].b + G_B1NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2NEPTUNE_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_B2NEPTUNE_COEFFICIENTS[i].b + G_B2NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3NEPTUNE_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_B3NEPTUNE_COEFFICIENTS[i].b + G_B3NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4NEPTUNE_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_B4NEPTUNE_COEFFICIENTS[i].b + G_B4NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_nep::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;

    //Calculate r0
    let n_r0coefficients = G_R0NEPTUNE_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_R0NEPTUNE_COEFFICIENTS[i].b + G_R0NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1NEPTUNE_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_R1NEPTUNE_COEFFICIENTS[i].b + G_R1NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2NEPTUNE_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_R2NEPTUNE_COEFFICIENTS[i].b + G_R2NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3NEPTUNE_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3NEPTUNE_COEFFICIENTS[i].a * f64::cos(G_R3NEPTUNE_COEFFICIENTS[i].b + G_R3NEPTUNE_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed) / 100000000.0
}
