use super::aavsop87;
use super::aacoordinatetransformation;
use super::aavsop87d_ven;

const G_L0VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 24] =
    [
        aavsop87::VSOP87Coefficient { a: 317614667.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1353968.0, b: 5.5931332, c: 10213.2855462 },
        aavsop87::VSOP87Coefficient { a: 89892.0, b: 5.30650, c: 20426.57109 },
        aavsop87::VSOP87Coefficient { a: 5477.0, b: 4.4163, c: 7860.4194 },
        aavsop87::VSOP87Coefficient { a: 3456.0, b: 2.6996, c: 11790.6291 },
        aavsop87::VSOP87Coefficient { a: 2372.0, b: 2.9938, c: 3930.2097 },
        aavsop87::VSOP87Coefficient { a: 1664.0, b: 4.2502, c: 1577.3435 },
        aavsop87::VSOP87Coefficient { a: 1438.0, b: 4.1575, c: 9683.5946 },
        aavsop87::VSOP87Coefficient { a: 1317.0, b: 5.1867, c: 26.2983 },
        aavsop87::VSOP87Coefficient { a: 1201.0, b: 6.1536, c: 30639.8566 },
        aavsop87::VSOP87Coefficient { a: 769.0, b: 0.816, c: 9437.763 },
        aavsop87::VSOP87Coefficient { a: 761.0, b: 1.950, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 708.0, b: 1.065, c: 775.523 },
        aavsop87::VSOP87Coefficient { a: 585.0, b: 3.998, c: 191.448 },
        aavsop87::VSOP87Coefficient { a: 500.0, b: 4.123, c: 15720.839 },
        aavsop87::VSOP87Coefficient { a: 429.0, b: 3.586, c: 19367.189 },
        aavsop87::VSOP87Coefficient { a: 327.0, b: 5.677, c: 5507.553 },
        aavsop87::VSOP87Coefficient { a: 326.0, b: 4.591, c: 10404.734 },
        aavsop87::VSOP87Coefficient { a: 232.0, b: 3.163, c: 9153.904 },
        aavsop87::VSOP87Coefficient { a: 180.0, b: 4.653, c: 1109.379 },
        aavsop87::VSOP87Coefficient { a: 155.0, b: 5.570, c: 19651.048 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 4.226, c: 20.775 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 0.962, c: 5661.332 },
        aavsop87::VSOP87Coefficient { a: 106.0, b: 1.537, c: 801.821 }
    ];

const G_L1VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 12] =
    [
        aavsop87::VSOP87Coefficient { a: 1021352943053.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 95708.0, b: 2.46424, c: 10213.28555 },
        aavsop87::VSOP87Coefficient { a: 14445.0, b: 0.51625, c: 20426.57109 },
        aavsop87::VSOP87Coefficient { a: 213.0, b: 1.795, c: 30639.857 },
        aavsop87::VSOP87Coefficient { a: 174.0, b: 2.655, c: 26.298 },
        aavsop87::VSOP87Coefficient { a: 152.0, b: 6.106, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 82.0, b: 5.70, c: 191.45 },
        aavsop87::VSOP87Coefficient { a: 70.0, b: 2.68, c: 9437.76 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 3.60, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 1.03, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 1.25, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 6.11, c: 10404.73 }
    ];

const G_L2VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 8] =
    [
        aavsop87::VSOP87Coefficient { a: 54127.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 3891.0, b: 0.3451, c: 10213.2855 },
        aavsop87::VSOP87Coefficient { a: 1338.0, b: 2.0201, c: 20426.5711 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 2.05, c: 26.30 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 3.54, c: 30639.86 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 3.97, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 1.52, c: 1577.34 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.00, c: 191.45 }
    ];

const G_L3VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 136.0, b: 4.804, c: 10213.286 },
        aavsop87::VSOP87Coefficient { a: 78.0, b: 3.67, c: 20426.57 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 0.0, c: 0.0 }
    ];

const G_L4VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.21, c: 20426.57 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.51, c: 10213.29 }
    ];

const G_L5VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_B0VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 9] =
    [
        aavsop87::VSOP87Coefficient { a: 5923638.0, b: 0.2670278, c: 10213.2855462 },
        aavsop87::VSOP87Coefficient { a: 40108.0, b: 1.14737, c: 20426.57109 },
        aavsop87::VSOP87Coefficient { a: 32815.0, b: 3.14159, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1011.0, b: 1.0895, c: 30639.8566 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 6.254, c: 18073.705 },
        aavsop87::VSOP87Coefficient { a: 138.0, b: 0.860, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 130.0, b: 3.672, c: 9437.763 },
        aavsop87::VSOP87Coefficient { a: 120.0, b: 3.705, c: 2352.866 },
        aavsop87::VSOP87Coefficient { a: 108.0, b: 4.539, c: 22003.915 }
    ];

const G_B1VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 513348.0, b: 1.803643, c: 10213.285546 },
        aavsop87::VSOP87Coefficient { a: 4380.0, b: 3.3862, c: 20426.5711 },
        aavsop87::VSOP87Coefficient { a: 199.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 197.0, b: 2.530, c: 30639.857 }
    ];

const G_B2VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 22378.0, b: 3.38509, c: 10213.28555 },
        aavsop87::VSOP87Coefficient { a: 282.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 173.0, b: 5.256, c: 20426.571 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 3.87, c: 30639.86 }
    ];

const G_B3VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 647.0, b: 4.992, c: 10213.286 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 0.77, c: 20426.57 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.44, c: 30639.86 }
    ];

const G_B4VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 14.0, b: 0.32, c: 10213.29 }
    ];

const G_R0VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 12] =
    [
        aavsop87::VSOP87Coefficient { a: 72334821.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 489824.0, b: 4.021518, c: 10213.285546 },
        aavsop87::VSOP87Coefficient { a: 1658.0, b: 4.9021, c: 20426.5711 },
        aavsop87::VSOP87Coefficient { a: 1632.0, b: 2.8455, c: 7860.4194 },
        aavsop87::VSOP87Coefficient { a: 1378.0, b: 1.1285, c: 11790.6291 },
        aavsop87::VSOP87Coefficient { a: 498.0, b: 2.587, c: 9683.595 },
        aavsop87::VSOP87Coefficient { a: 374.0, b: 1.423, c: 3930.210 },
        aavsop87::VSOP87Coefficient { a: 264.0, b: 5.529, c: 9437.763 },
        aavsop87::VSOP87Coefficient { a: 237.0, b: 2.551, c: 15720.839 },
        aavsop87::VSOP87Coefficient { a: 222.0, b: 2.013, c: 19367.189 },
        aavsop87::VSOP87Coefficient { a: 126.0, b: 2.728, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 119.0, b: 3.020, c: 10404.734 }
    ];

const G_R1VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 34551.0, b: 0.89199, c: 10213.28555 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 1.772, c: 20426.571 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 3.142, c: 0.0 }
    ];

const G_R2VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 1407.0, b: 5.0637, c: 10213.2855 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 5.47, c: 20426.57 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 0.0, c: 0.0 }
    ];

const G_R3VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 50.0, b: 3.22, c: 10213.29 }
    ];

const G_R4VENUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.92, c: 10213.29 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ven::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate l0
    let n_l0coefficients = G_L0VENUS_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0VENUS_COEFFICIENTS[i].a * f64::cos(G_L0VENUS_COEFFICIENTS[i].b + G_L0VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1VENUS_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1VENUS_COEFFICIENTS[i].a * f64::cos(G_L1VENUS_COEFFICIENTS[i].b + G_L1VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2VENUS_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2VENUS_COEFFICIENTS[i].a * f64::cos(G_L2VENUS_COEFFICIENTS[i].b + G_L2VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3VENUS_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3VENUS_COEFFICIENTS[i].a * f64::cos(G_L3VENUS_COEFFICIENTS[i].b + G_L3VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4VENUS_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4VENUS_COEFFICIENTS[i].a * f64::cos(G_L4VENUS_COEFFICIENTS[i].b + G_L4VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l5
    let n_l5coefficients = G_L5VENUS_COEFFICIENTS.len();
    let mut l5 = 0.0f64;
    for i in 0..n_l5coefficients {
        l5 += G_L5VENUS_COEFFICIENTS[i].a * f64::cos(G_L5VENUS_COEFFICIENTS[i].b + G_L5VENUS_COEFFICIENTS[i].c * rho);
    }

    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ven::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate b0
    let n_b0coefficients = G_B0VENUS_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0VENUS_COEFFICIENTS[i].a * f64::cos(G_B0VENUS_COEFFICIENTS[i].b + G_B0VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1VENUS_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1VENUS_COEFFICIENTS[i].a * f64::cos(G_B1VENUS_COEFFICIENTS[i].b + G_B1VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2VENUS_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2VENUS_COEFFICIENTS[i].a * f64::cos(G_B2VENUS_COEFFICIENTS[i].b + G_B2VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3VENUS_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3VENUS_COEFFICIENTS[i].a * f64::cos(G_B3VENUS_COEFFICIENTS[i].b + G_B3VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4VENUS_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4VENUS_COEFFICIENTS[i].a * f64::cos(G_B4VENUS_COEFFICIENTS[i].b + G_B4VENUS_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_ven::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate r0
    let n_r0coefficients = G_R0VENUS_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0VENUS_COEFFICIENTS[i].a * f64::cos(G_R0VENUS_COEFFICIENTS[i].b + G_R0VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1VENUS_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1VENUS_COEFFICIENTS[i].a * f64::cos(G_R1VENUS_COEFFICIENTS[i].b + G_R1VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2VENUS_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2VENUS_COEFFICIENTS[i].a * f64::cos(G_R2VENUS_COEFFICIENTS[i].b + G_R2VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3VENUS_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3VENUS_COEFFICIENTS[i].a * f64::cos(G_R3VENUS_COEFFICIENTS[i].b + G_R3VENUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r4
    let n_r4coefficients = G_R4VENUS_COEFFICIENTS.len();
    let mut r4 = 0.0f64;
    for i in 0..n_r4coefficients {
        r4 += G_R4VENUS_COEFFICIENTS[i].a * f64::cos(G_R4VENUS_COEFFICIENTS[i].b + G_R4VENUS_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4) / 100000000.0
}
