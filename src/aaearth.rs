use super::aacoordinatetransformation;
use super::aavsop87;
use super::aavsop87b_ear;
use super::aavsop87d_ear;

const G_L0_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 64] =
    [
        aavsop87::VSOP87Coefficient { a: 175347046.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 3341656.0, b: 4.6692568, c: 6283.0758500 },
        aavsop87::VSOP87Coefficient { a: 34894.0, b: 4.62610, c: 12566.15170 },
        aavsop87::VSOP87Coefficient { a: 3497.0, b: 2.7441, c: 5753.3849 },
        aavsop87::VSOP87Coefficient { a: 3418.0, b: 2.8289, c: 3.5231 },
        aavsop87::VSOP87Coefficient { a: 3136.0, b: 3.6277, c: 77713.7715 },
        aavsop87::VSOP87Coefficient { a: 2676.0, b: 4.4181, c: 7860.4194 },
        aavsop87::VSOP87Coefficient { a: 2343.0, b: 6.1352, c: 3930.2097 },
        aavsop87::VSOP87Coefficient { a: 1324.0, b: 0.7425, c: 11506.7698 },
        aavsop87::VSOP87Coefficient { a: 1273.0, b: 2.0371, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 1199.0, b: 1.1096, c: 1577.3435 },
        aavsop87::VSOP87Coefficient { a: 990.0, b: 5.233, c: 5884.927 },
        aavsop87::VSOP87Coefficient { a: 902.0, b: 2.045, c: 26.298 },
        aavsop87::VSOP87Coefficient { a: 857.0, b: 3.508, c: 398.149 },
        aavsop87::VSOP87Coefficient { a: 780.0, b: 1.179, c: 5223.694 },
        aavsop87::VSOP87Coefficient { a: 753.0, b: 2.533, c: 5507.553 },
        aavsop87::VSOP87Coefficient { a: 505.0, b: 4.583, c: 18849.228 },
        aavsop87::VSOP87Coefficient { a: 492.0, b: 4.205, c: 775.523 },
        aavsop87::VSOP87Coefficient { a: 357.0, b: 2.920, c: 0.067 },
        aavsop87::VSOP87Coefficient { a: 317.0, b: 5.849, c: 11790.629 },
        aavsop87::VSOP87Coefficient { a: 284.0, b: 1.899, c: 796.298 },
        aavsop87::VSOP87Coefficient { a: 271.0, b: 0.315, c: 10977.079 },
        aavsop87::VSOP87Coefficient { a: 243.0, b: 0.345, c: 5486.778 },
        aavsop87::VSOP87Coefficient { a: 206.0, b: 4.806, c: 2544.314 },
        aavsop87::VSOP87Coefficient { a: 205.0, b: 1.869, c: 5573.143 },
        aavsop87::VSOP87Coefficient { a: 202.0, b: 2.458, c: 6069.777 },
        aavsop87::VSOP87Coefficient { a: 156.0, b: 0.833, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 132.0, b: 3.411, c: 2942.463 },
        aavsop87::VSOP87Coefficient { a: 126.0, b: 1.083, c: 20.775 },
        aavsop87::VSOP87Coefficient { a: 115.0, b: 0.645, c: 0.980 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 0.636, c: 4694.003 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 0.976, c: 15720.839 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 4.267, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 99.0, b: 6.21, c: 2146.17 },
        aavsop87::VSOP87Coefficient { a: 98.0, b: 0.68, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 86.0, b: 5.98, c: 161000.69 },
        aavsop87::VSOP87Coefficient { a: 85.0, b: 1.30, c: 6275.96 },
        aavsop87::VSOP87Coefficient { a: 85.0, b: 3.67, c: 71430.70 },
        aavsop87::VSOP87Coefficient { a: 80.0, b: 1.81, c: 17260.15 },
        aavsop87::VSOP87Coefficient { a: 79.0, b: 3.04, c: 12036.46 },
        aavsop87::VSOP87Coefficient { a: 75.0, b: 1.76, c: 5088.63 },
        aavsop87::VSOP87Coefficient { a: 74.0, b: 3.50, c: 3154.69 },
        aavsop87::VSOP87Coefficient { a: 74.0, b: 4.68, c: 801.82 },
        aavsop87::VSOP87Coefficient { a: 70.0, b: 0.83, c: 9437.76 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 3.98, c: 8827.39 },
        aavsop87::VSOP87Coefficient { a: 61.0, b: 1.82, c: 7084.90 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 2.78, c: 6286.60 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 4.39, c: 14143.50 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 3.47, c: 6279.55 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 0.19, c: 12139.55 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 1.33, c: 1748.02 },
        aavsop87::VSOP87Coefficient { a: 51.0, b: 0.28, c: 5856.48 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 0.49, c: 1194.45 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 5.37, c: 8429.24 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 2.40, c: 19651.05 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 6.17, c: 10447.39 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 6.04, c: 10213.29 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 2.57, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 1.71, c: 2352.87 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 1.78, c: 6812.77 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 0.59, c: 17789.85 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 0.44, c: 83996.85 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 2.74, c: 1349.87 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 3.16, c: 4690.48 }
    ];

const G_L1_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 34] =
    [
        aavsop87::VSOP87Coefficient { a: 628331966747.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 206059.0, b: 2.678235, c: 6283.075850 },
        aavsop87::VSOP87Coefficient { a: 4303.0, b: 2.6351, c: 12566.1517 },
        aavsop87::VSOP87Coefficient { a: 425.0, b: 1.590, c: 3.523 },
        aavsop87::VSOP87Coefficient { a: 119.0, b: 5.796, c: 26.298 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 2.966, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 93.0, b: 2.59, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 1.14, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 68.0, b: 1.87, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 4.41, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 2.89, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 2.17, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 0.40, c: 796.30 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 0.47, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 2.65, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 5.34, c: 0.98 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 1.85, c: 5486.78 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 4.97, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.99, c: 6275.96 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 0.03, c: 2544.31 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 1.43, c: 2146.17 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 1.21, c: 10977.08 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.83, c: 1748.02 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.26, c: 5088.63 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 5.27, c: 1194.45 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.08, c: 4694.00 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 0.77, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 1.30, c: 6286.60 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 4.24, c: 1349.87 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.70, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 5.64, c: 951.72 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.30, c: 2352.87 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.65, c: 9437.76 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.67, c: 4690.48 }
    ];

const G_L2_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 20] =
    [
        aavsop87::VSOP87Coefficient { a: 52919.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 8720.0, b: 1.0721, c: 6283.0758 },
        aavsop87::VSOP87Coefficient { a: 309.0, b: 0.867, c: 12566.152 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 0.05, c: 3.52 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 5.19, c: 26.30 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 3.68, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 0.76, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.06, c: 77713.77 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 0.83, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 4.66, c: 1577.34 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 1.03, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 3.44, c: 5573.14 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.14, c: 796.30 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 6.05, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 1.19, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 6.12, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.31, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.28, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.38, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.75, c: 0.98 }
    ];

const G_L3_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 289.0, b: 5.844, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 5.49, c: 12566.15 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.20, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.72, c: 3.52 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.30, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.97, c: 242.73 }
    ];

const G_L4_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 4.13, c: 6283.08 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.84, c: 12566.15 }
    ];

const G_L5_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_B0_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 5] =
    [
        aavsop87::VSOP87Coefficient { a: 280.0, b: 3.199, c: 84334.662 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 5.422, c: 5507.553 },
        aavsop87::VSOP87Coefficient { a: 80.0, b: 3.88, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 3.70, c: 2352.87 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.00, c: 1577.34 }
    ];

const G_B1_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.90, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.73, c: 5223.69 }
    ];

const G_R0_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 40] =
    [
        aavsop87::VSOP87Coefficient { a: 100013989.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1670700.0, b: 3.0984635, c: 6283.0758500 },
        aavsop87::VSOP87Coefficient { a: 13956.0, b: 3.05525, c: 12566.15170 },
        aavsop87::VSOP87Coefficient { a: 3084.0, b: 5.1985, c: 77713.7715 },
        aavsop87::VSOP87Coefficient { a: 1628.0, b: 1.1739, c: 5753.3849 },
        aavsop87::VSOP87Coefficient { a: 1576.0, b: 2.8469, c: 7860.4194 },
        aavsop87::VSOP87Coefficient { a: 925.0, b: 5.453, c: 11506.770 },
        aavsop87::VSOP87Coefficient { a: 542.0, b: 4.564, c: 3930.210 },
        aavsop87::VSOP87Coefficient { a: 472.0, b: 3.661, c: 5884.927 },
        aavsop87::VSOP87Coefficient { a: 346.0, b: 0.964, c: 5507.553 },
        aavsop87::VSOP87Coefficient { a: 329.0, b: 5.900, c: 5223.694 },
        aavsop87::VSOP87Coefficient { a: 307.0, b: 0.299, c: 5573.143 },
        aavsop87::VSOP87Coefficient { a: 243.0, b: 4.273, c: 11790.629 },
        aavsop87::VSOP87Coefficient { a: 212.0, b: 5.847, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 186.0, b: 5.022, c: 10977.079 },
        aavsop87::VSOP87Coefficient { a: 175.0, b: 3.012, c: 18849.228 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 5.055, c: 5486.778 },
        aavsop87::VSOP87Coefficient { a: 98.0, b: 0.89, c: 6069.78 },
        aavsop87::VSOP87Coefficient { a: 86.0, b: 5.69, c: 15720.84 },
        aavsop87::VSOP87Coefficient { a: 86.0, b: 1.27, c: 161000.69 },
        aavsop87::VSOP87Coefficient { a: 65.0, b: 0.27, c: 17260.15 },
        aavsop87::VSOP87Coefficient { a: 63.0, b: 0.92, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 2.01, c: 83996.85 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 5.24, c: 71430.70 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 3.25, c: 2544.31 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 2.58, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 5.54, c: 9437.76 },
        aavsop87::VSOP87Coefficient { a: 43.0, b: 6.01, c: 6275.96 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 5.36, c: 4694.00 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 2.39, c: 8827.39 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 0.83, c: 19651.05 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 4.90, c: 12139.55 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 1.67, c: 12036.46 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 1.84, c: 2942.46 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 0.24, c: 7084.90 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 0.18, c: 5088.63 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 1.78, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 1.21, c: 6286.60 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 1.90, c: 6279.55 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 4.59, c: 10447.39 }
    ];

const G_R1_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 10] =
    [
        aavsop87::VSOP87Coefficient { a: 103019.0, b: 1.107490, c: 6283.075850 },
        aavsop87::VSOP87Coefficient { a: 1721.0, b: 1.0644, c: 12566.1517 },
        aavsop87::VSOP87Coefficient { a: 702.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 1.02, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 2.84, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 1.32, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 1.42, c: 1577.34 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 5.91, c: 10977.08 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 1.42, c: 6275.96 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.27, c: 5486.78 }
    ];

const G_R2_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 6] =
    [
        aavsop87::VSOP87Coefficient { a: 4359.0, b: 5.7846, c: 6283.0758 },
        aavsop87::VSOP87Coefficient { a: 124.0, b: 5.579, c: 12566.152 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.63, c: 77713.77 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.87, c: 5573.14 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.47, c: 18849.23 }
    ];

const G_R3_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 145.0, b: 4.273, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 3.92, c: 12566.15 }
    ];

const G_R4_EARTH_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 4.0, b: 2.56, c: 6283.08 }
    ];

const G_L1_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 34] =
    [
        aavsop87::VSOP87Coefficient { a: 628307584999.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 206059.0, b: 2.678235, c: 6283.075850 },
        aavsop87::VSOP87Coefficient { a: 4303.0, b: 2.6351, c: 12566.1517 },
        aavsop87::VSOP87Coefficient { a: 425.0, b: 1.590, c: 3.523 },
        aavsop87::VSOP87Coefficient { a: 119.0, b: 5.796, c: 26.298 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 2.966, c: 1577.344 },
        aavsop87::VSOP87Coefficient { a: 93.0, b: 2.59, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 1.14, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 68.0, b: 1.87, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 4.41, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 2.89, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 2.17, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 0.40, c: 796.30 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 0.47, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 2.65, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 5.43, c: 0.98 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 1.85, c: 5486.78 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 4.97, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.99, c: 6275.96 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 0.03, c: 2544.31 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 1.43, c: 2146.17 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 1.21, c: 10977.08 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.83, c: 1748.02 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.26, c: 5088.63 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 5.27, c: 1194.45 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.08, c: 4694.00 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 0.77, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 1.30, c: 6286.60 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 4.24, c: 1349.87 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.70, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 5.64, c: 951.72 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.30, c: 2352.87 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.65, c: 9437.76 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.67, c: 4690.48 }
    ];

const G_L2_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 20] =
    [
        aavsop87::VSOP87Coefficient { a: 8722.0, b: 1.0725, c: 6283.0758 },
        aavsop87::VSOP87Coefficient { a: 991.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 295.0, b: 0.437, c: 12566.152 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 0.05, c: 3.52 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 5.19, c: 26.30 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 3.69, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.30, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.06, c: 77713.77 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 0.83, c: 775.52 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 4.66, c: 1577.34 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 1.03, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 3.44, c: 5573.14 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.14, c: 796.30 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 6.05, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 1.19, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 6.12, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.30, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.28, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.38, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.75, c: 0.98 }
    ];

const G_L3_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 289.0, b: 5.842, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 6.05, c: 12566.15 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.20, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.72, c: 3.52 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.97, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.54, c: 18849.23 }
    ];

const G_L4_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 8.0, b: 4.14, c: 6283.08 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.28, c: 12566.15 }
    ];

const G_B1_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 227778.0, b: 3.413766, c: 6283.075850 },
        aavsop87::VSOP87Coefficient { a: 3806.0, b: 3.3706, c: 12566.1517 },
        aavsop87::VSOP87Coefficient { a: 3620.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 3.33, c: 18849.23 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 3.89, c: 5507.55 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 1.79, c: 5223.69 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.20, c: 2352.87 }
    ];

const G_B2_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 9721.0, b: 5.1519, c: 6283.07585 },
        aavsop87::VSOP87Coefficient { a: 233.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 134.0, b: 0.644, c: 12566.152 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 1.07, c: 18849.23 }
    ];

const G_B3_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 276.0, b: 0.595, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 0.12, c: 12566.15 }
    ];

const G_B4_EARTH_COEFFICIENTS_J2000: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.27, c: 6283.08 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.0, c: 0.0 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ear::l(jd)))
    } else {
        let rho = (jd - 2451545.0) / 365250.0;
        let rhosquared = rho * rho;
        let rhocubed = rhosquared * rho;
        let rho4 = rhocubed * rho;
        let rho5 = rho4 * rho;

        //Calculate L0
        let n_l0coefficients = G_L0_EARTH_COEFFICIENTS.len();
        let mut l0 = 0.0;
        for i in 0..n_l0coefficients {
            l0 += G_L0_EARTH_COEFFICIENTS[i].a * f64::cos(G_L0_EARTH_COEFFICIENTS[i].b + G_L0_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L1
        let n_l1coefficients = G_L1_EARTH_COEFFICIENTS.len();
        let mut l1 = 0.0;
        for i in 0..n_l1coefficients {
            l1 += G_L1_EARTH_COEFFICIENTS[i].a * f64::cos(G_L1_EARTH_COEFFICIENTS[i].b + G_L1_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L2
        let n_l2coefficients = G_L2_EARTH_COEFFICIENTS.len();
        let mut l2 = 0.0;
        for i in 0..n_l2coefficients {
            l2 += G_L2_EARTH_COEFFICIENTS[i].a * f64::cos(G_L2_EARTH_COEFFICIENTS[i].b + G_L2_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L3
        let n_l3coefficients = G_L3_EARTH_COEFFICIENTS.len();
        let mut l3 = 0.0;
        for i in 0..n_l3coefficients {
            l3 += G_L3_EARTH_COEFFICIENTS[i].a * f64::cos(G_L3_EARTH_COEFFICIENTS[i].b + G_L3_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L4
        let n_l4coefficients = G_L4_EARTH_COEFFICIENTS.len();
        let mut l4 = 0.0;
        for i in 0..n_l4coefficients {
            l4 += G_L4_EARTH_COEFFICIENTS[i].a * f64::cos(G_L4_EARTH_COEFFICIENTS[i].b + G_L4_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L5
        let n_l5coefficients = G_L5_EARTH_COEFFICIENTS.len();
        let mut l5 = 0.0;
        for i in 0..n_l5coefficients {
            l5 += G_L5_EARTH_COEFFICIENTS[i].a * f64::cos(G_L5_EARTH_COEFFICIENTS[i].b + G_L5_EARTH_COEFFICIENTS[i].c * rho);
        }

        let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

        //convert results back to degrees
        aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
    }
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ear::b(jd)))
    } else {
        let rho = (jd - 2451545.0) / 365250.0;

        //Calculate B0
        let n_b0coefficients = G_B0_EARTH_COEFFICIENTS.len();
        let mut b0 = 0.0;
        for i in 0..n_b0coefficients {
            b0 += G_B0_EARTH_COEFFICIENTS[i].a * f64::cos(G_B0_EARTH_COEFFICIENTS[i].b + G_B0_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate B1
        let n_b1coefficients = G_B1_EARTH_COEFFICIENTS.len();
        let mut b1 = 0.0;
        for i in 0..n_b1coefficients {
            b1 += G_B1_EARTH_COEFFICIENTS[i].a * f64::cos(G_B1_EARTH_COEFFICIENTS[i].b + G_B1_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Note for Earth there are no B2, B3 or B4 coefficients to calculate

        let value = (b0 + b1 * rho) / 100000000.0;

        //convert results back to degrees
        aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
    }
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        aavsop87d_ear::r(jd)
    } else {
        let rho = (jd - 2451545.0) / 365250.0;
        let rhosquared = rho * rho;
        let rhocubed = rhosquared * rho;
        let rho4 = rhocubed * rho;

        //Calculate R0
        let n_r0coefficients = G_R0_EARTH_COEFFICIENTS.len();
        let mut r0 = 0.0;
        for i in 0..n_r0coefficients {
            r0 += G_R0_EARTH_COEFFICIENTS[i].a * f64::cos(G_R0_EARTH_COEFFICIENTS[i].b + G_R0_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate R1
        let n_r1coefficients = G_R1_EARTH_COEFFICIENTS.len();
        let mut r1 = 0.0;
        for i in 0..n_r1coefficients {
            r1 += G_R1_EARTH_COEFFICIENTS[i].a * f64::cos(G_R1_EARTH_COEFFICIENTS[i].b + G_R1_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate R2
        let n_r2coefficients = G_R2_EARTH_COEFFICIENTS.len();
        let mut r2 = 0.0;
        for i in 0..n_r2coefficients {
            r2 += G_R2_EARTH_COEFFICIENTS[i].a * f64::cos(G_R2_EARTH_COEFFICIENTS[i].b + G_R2_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate R3
        let n_r3coefficients = G_R3_EARTH_COEFFICIENTS.len();
        let mut r3 = 0.0;
        for i in 0..n_r3coefficients {
            r3 += G_R3_EARTH_COEFFICIENTS[i].a * f64::cos(G_R3_EARTH_COEFFICIENTS[i].b + G_R3_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate R4
        let n_r4coefficients = G_R4_EARTH_COEFFICIENTS.len();
        let mut r4 = 0.0;
        for i in 0..n_r4coefficients {
            r4 += G_R4_EARTH_COEFFICIENTS[i].a * f64::cos(G_R4_EARTH_COEFFICIENTS[i].b + G_R4_EARTH_COEFFICIENTS[i].c * rho);
        }

        (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4) / 100000000.0
    }
}

pub fn ecliptic_longitude_j2000(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87b_ear::l(jd)))
    } else {
        let rho = (jd - 2451545.0) / 365250.0;
        let rhosquared = rho * rho;
        let rhocubed = rhosquared * rho;
        let rho4 = rhocubed * rho;

        //Calculate L0
        let n_l0coefficients = G_L0_EARTH_COEFFICIENTS.len();
        let mut l0 = 0.0;
        for i in 0..n_l0coefficients {
            l0 += G_L0_EARTH_COEFFICIENTS[i].a * f64::cos(G_L0_EARTH_COEFFICIENTS[i].b + G_L0_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate L1
        let n_l1coefficients = G_L1_EARTH_COEFFICIENTS_J2000.len();
        let mut l1 = 0.0;
        for i in 0..n_l1coefficients {
            l1 += G_L1_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_L1_EARTH_COEFFICIENTS_J2000[i].b + G_L1_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate L2
        let n_l2coefficients = G_L2_EARTH_COEFFICIENTS_J2000.len();
        let mut l2 = 0.0;
        for i in 0..n_l2coefficients {
            l2 += G_L2_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_L2_EARTH_COEFFICIENTS_J2000[i].b + G_L2_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate L3
        let n_l3coefficients = G_L3_EARTH_COEFFICIENTS_J2000.len();
        let mut l3 = 0.0;
        for i in 0..n_l3coefficients {
            l3 += G_L3_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_L3_EARTH_COEFFICIENTS_J2000[i].b + G_L3_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate L4
        let n_l4coefficients = G_L4_EARTH_COEFFICIENTS_J2000.len();
        let mut l4 = 0.0;
        for i in 0..n_l4coefficients {
            l4 += G_L4_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_L4_EARTH_COEFFICIENTS_J2000[i].b + G_L4_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4) / 100000000.0;

        //convert results back to degrees
        aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
    }
}

pub fn ecliptic_latitude_j2000(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87b_ear::b(jd)))
    } else {
        let rho = (jd - 2451545.0) / 365250.0;
        let rhosquared = rho * rho;
        let rhocubed = rhosquared * rho;
        let rho4 = rhocubed * rho;

        //Calculate B0
        let n_b0coefficients = G_B0_EARTH_COEFFICIENTS.len();
        let mut b0 = 0.0;
        for i in 0..n_b0coefficients {
            b0 += G_B0_EARTH_COEFFICIENTS[i].a * f64::cos(G_B0_EARTH_COEFFICIENTS[i].b + G_B0_EARTH_COEFFICIENTS[i].c * rho);
        }

        //Calculate B1
        let n_b1coefficients = G_B1_EARTH_COEFFICIENTS_J2000.len();
        let mut b1 = 0.0;
        for i in 0..n_b1coefficients {
            b1 += G_B1_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_B1_EARTH_COEFFICIENTS_J2000[i].b + G_B1_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate B2
        let n_b2coefficients = G_B2_EARTH_COEFFICIENTS_J2000.len();
        let mut b2 = 0.0;
        for i in 0..n_b2coefficients {
            b2 += G_B2_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_B2_EARTH_COEFFICIENTS_J2000[i].b + G_B2_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate B3
        let n_b3coefficients = G_B3_EARTH_COEFFICIENTS_J2000.len();
        let mut b3 = 0.0;
        for i in 0..n_b3coefficients {
            b3 += G_B3_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_B3_EARTH_COEFFICIENTS_J2000[i].b + G_B3_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        //Calculate B4
        let n_b4coefficients = G_B4_EARTH_COEFFICIENTS_J2000.len();
        let mut b4 = 0.0;
        for i in 0..n_b4coefficients {
            b4 += G_B4_EARTH_COEFFICIENTS_J2000[i].a * f64::cos(G_B4_EARTH_COEFFICIENTS_J2000[i].b + G_B4_EARTH_COEFFICIENTS_J2000[i].c * rho);
        }

        let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

        //convert results back to degrees
        aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
    }
}

pub fn sun_mean_anomaly(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;
    aacoordinatetransformation::map_to_0to360_range(357.5291092 + 35999.0502909 * t - 0.0001536 * tsquared + tcubed / 24490000.0)
}
