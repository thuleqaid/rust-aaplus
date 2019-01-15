use super::aavsop87;
use super::aacoordinatetransformation;
use super::aavsop87d_mar;

const G_L0MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 69] =
    [
        aavsop87::VSOP87Coefficient { a: 620347712.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 18656368.0, b: 5.05037100, c: 3340.61242670 },
        aavsop87::VSOP87Coefficient { a: 1108217.0, b: 5.4009984, c: 6681.2248534 },
        aavsop87::VSOP87Coefficient { a: 91798.0, b: 5.75479, c: 10021.83728 },
        aavsop87::VSOP87Coefficient { a: 27745.0, b: 5.97050, c: 3.52312 },
        aavsop87::VSOP87Coefficient { a: 12316.0, b: 0.84956, c: 2810.92146 },
        aavsop87::VSOP87Coefficient { a: 10610.0, b: 2.93959, c: 2281.23050 },
        aavsop87::VSOP87Coefficient { a: 8927.0, b: 4.1570, c: 0.0173 },
        aavsop87::VSOP87Coefficient { a: 8716.0, b: 6.1101, c: 13362.4497 },
        aavsop87::VSOP87Coefficient { a: 7775.0, b: 3.3397, c: 5621.8429 },
        aavsop87::VSOP87Coefficient { a: 6798.0, b: 0.3646, c: 398.1490 },
        aavsop87::VSOP87Coefficient { a: 4161.0, b: 0.2281, c: 2942.4634 },
        aavsop87::VSOP87Coefficient { a: 3575.0, b: 1.6619, c: 2544.3144 },
        aavsop87::VSOP87Coefficient { a: 3075.0, b: 0.8570, c: 191.4483 },
        aavsop87::VSOP87Coefficient { a: 2938.0, b: 6.0789, c: 0.0673 },
        aavsop87::VSOP87Coefficient { a: 2628.0, b: 0.6481, c: 3337.0893 },
        aavsop87::VSOP87Coefficient { a: 2580.0, b: 0.0300, c: 3344.1355 },
        aavsop87::VSOP87Coefficient { a: 2389.0, b: 5.0390, c: 796.2980 },
        aavsop87::VSOP87Coefficient { a: 1799.0, b: 0.6563, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 1546.0, b: 2.9158, c: 1751.5395 },
        aavsop87::VSOP87Coefficient { a: 1528.0, b: 1.1498, c: 6151.5339 },
        aavsop87::VSOP87Coefficient { a: 1286.0, b: 3.0680, c: 2146.1654 },
        aavsop87::VSOP87Coefficient { a: 1264.0, b: 3.6228, c: 5092.1520 },
        aavsop87::VSOP87Coefficient { a: 1025.0, b: 3.6933, c: 8962.4553 },
        aavsop87::VSOP87Coefficient { a: 892.0, b: 0.183, c: 16703.062 },
        aavsop87::VSOP87Coefficient { a: 859.0, b: 2.401, c: 2914.014 },
        aavsop87::VSOP87Coefficient { a: 833.0, b: 4.495, c: 3340.630 },
        aavsop87::VSOP87Coefficient { a: 833.0, b: 2.464, c: 3340.595 },
        aavsop87::VSOP87Coefficient { a: 749.0, b: 3.822, c: 155.420 },
        aavsop87::VSOP87Coefficient { a: 724.0, b: 0.675, c: 3738.761 },
        aavsop87::VSOP87Coefficient { a: 713.0, b: 3.663, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 655.0, b: 0.489, c: 3127.313 },
        aavsop87::VSOP87Coefficient { a: 636.0, b: 2.922, c: 8432.764 },
        aavsop87::VSOP87Coefficient { a: 553.0, b: 4.475, c: 1748.016 },
        aavsop87::VSOP87Coefficient { a: 550.0, b: 3.810, c: 0.980 },
        aavsop87::VSOP87Coefficient { a: 472.0, b: 3.625, c: 1194.447 },
        aavsop87::VSOP87Coefficient { a: 426.0, b: 0.554, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 415.0, b: 0.497, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 312.0, b: 0.999, c: 6677.702 },
        aavsop87::VSOP87Coefficient { a: 307.0, b: 0.381, c: 6684.748 },
        aavsop87::VSOP87Coefficient { a: 302.0, b: 4.486, c: 3532.061 },
        aavsop87::VSOP87Coefficient { a: 299.0, b: 2.783, c: 6254.627 },
        aavsop87::VSOP87Coefficient { a: 293.0, b: 4.221, c: 20.775 },
        aavsop87::VSOP87Coefficient { a: 284.0, b: 5.769, c: 3149.164 },
        aavsop87::VSOP87Coefficient { a: 281.0, b: 5.882, c: 1349.867 },
        aavsop87::VSOP87Coefficient { a: 274.0, b: 0.542, c: 3340.545 },
        aavsop87::VSOP87Coefficient { a: 274.0, b: 0.134, c: 3340.680 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 5.372, c: 4136.910 },
        aavsop87::VSOP87Coefficient { a: 236.0, b: 5.755, c: 3333.499 },
        aavsop87::VSOP87Coefficient { a: 231.0, b: 1.282, c: 3870.303 },
        aavsop87::VSOP87Coefficient { a: 221.0, b: 3.505, c: 382.897 },
        aavsop87::VSOP87Coefficient { a: 204.0, b: 2.821, c: 1221.849 },
        aavsop87::VSOP87Coefficient { a: 193.0, b: 3.357, c: 3.590 },
        aavsop87::VSOP87Coefficient { a: 189.0, b: 1.491, c: 9492.146 },
        aavsop87::VSOP87Coefficient { a: 179.0, b: 1.006, c: 951.718 },
        aavsop87::VSOP87Coefficient { a: 174.0, b: 2.414, c: 553.569 },
        aavsop87::VSOP87Coefficient { a: 172.0, b: 0.439, c: 5486.778 },
        aavsop87::VSOP87Coefficient { a: 160.0, b: 3.949, c: 4562.461 },
        aavsop87::VSOP87Coefficient { a: 144.0, b: 1.419, c: 135.065 },
        aavsop87::VSOP87Coefficient { a: 140.0, b: 3.326, c: 2700.715 },
        aavsop87::VSOP87Coefficient { a: 138.0, b: 4.301, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 4.045, c: 12303.068 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 2.208, c: 1592.596 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 1.807, c: 5088.629 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 3.128, c: 7903.073 },
        aavsop87::VSOP87Coefficient { a: 113.0, b: 3.701, c: 1589.073 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 1.052, c: 242.729 },
        aavsop87::VSOP87Coefficient { a: 105.0, b: 0.785, c: 8827.390 },
        aavsop87::VSOP87Coefficient { a: 100.0, b: 3.243, c: 11773.377 }
    ];

const G_L1MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 46] =
    [
        aavsop87::VSOP87Coefficient { a: 334085627474.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1458227.0, b: 3.6042605, c: 3340.6124267 },
        aavsop87::VSOP87Coefficient { a: 164901.0, b: 3.926313, c: 6681.224853 },
        aavsop87::VSOP87Coefficient { a: 19963.0, b: 4.26594, c: 10021.83728 },
        aavsop87::VSOP87Coefficient { a: 3452.0, b: 4.7321, c: 3.5231 },
        aavsop87::VSOP87Coefficient { a: 2485.0, b: 4.6128, c: 13362.4497 },
        aavsop87::VSOP87Coefficient { a: 842.0, b: 4.459, c: 2281.230 },
        aavsop87::VSOP87Coefficient { a: 538.0, b: 5.016, c: 398.149 },
        aavsop87::VSOP87Coefficient { a: 521.0, b: 4.994, c: 3344.136 },
        aavsop87::VSOP87Coefficient { a: 433.0, b: 2.561, c: 191.448 },
        aavsop87::VSOP87Coefficient { a: 430.0, b: 5.316, c: 155.420 },
        aavsop87::VSOP87Coefficient { a: 382.0, b: 3.539, c: 796.298 },
        aavsop87::VSOP87Coefficient { a: 314.0, b: 4.963, c: 16703.062 },
        aavsop87::VSOP87Coefficient { a: 283.0, b: 3.160, c: 2544.314 },
        aavsop87::VSOP87Coefficient { a: 206.0, b: 4.569, c: 2146.165 },
        aavsop87::VSOP87Coefficient { a: 169.0, b: 1.329, c: 3337.089 },
        aavsop87::VSOP87Coefficient { a: 158.0, b: 4.185, c: 1751.540 },
        aavsop87::VSOP87Coefficient { a: 134.0, b: 2.233, c: 0.980 },
        aavsop87::VSOP87Coefficient { a: 134.0, b: 5.974, c: 1748.016 },
        aavsop87::VSOP87Coefficient { a: 118.0, b: 6.024, c: 6151.534 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 2.213, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 2.129, c: 1194.447 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 5.428, c: 3738.761 },
        aavsop87::VSOP87Coefficient { a: 91.0, b: 1.10, c: 1349.87 },
        aavsop87::VSOP87Coefficient { a: 85.0, b: 3.91, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 5.30, c: 6684.75 },
        aavsop87::VSOP87Coefficient { a: 81.0, b: 4.43, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 80.0, b: 2.25, c: 8962.46 },
        aavsop87::VSOP87Coefficient { a: 73.0, b: 2.50, c: 951.72 },
        aavsop87::VSOP87Coefficient { a: 73.0, b: 5.84, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 71.0, b: 3.86, c: 2914.01 },
        aavsop87::VSOP87Coefficient { a: 68.0, b: 5.02, c: 382.90 },
        aavsop87::VSOP87Coefficient { a: 65.0, b: 1.02, c: 3340.60 },
        aavsop87::VSOP87Coefficient { a: 65.0, b: 3.05, c: 3340.63 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 4.15, c: 3149.16 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 3.89, c: 4136.91 },
        aavsop87::VSOP87Coefficient { a: 48.0, b: 4.87, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 48.0, b: 1.18, c: 3333.50 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 1.31, c: 3185.19 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 0.71, c: 1592.60 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 2.73, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 5.32, c: 20043.67 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 5.41, c: 6283.08 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 0.05, c: 9492.15 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 3.89, c: 1221.85 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.11, c: 2700.72 }
    ];

const G_L2MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 33] =
    [
        aavsop87::VSOP87Coefficient { a: 58016.0, b: 2.04979, c: 3340.61243 },
        aavsop87::VSOP87Coefficient { a: 54188.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 13908.0, b: 2.45742, c: 6681.22485 },
        aavsop87::VSOP87Coefficient { a: 2465.0, b: 2.8000, c: 10021.8373 },
        aavsop87::VSOP87Coefficient { a: 398.0, b: 3.141, c: 13362.450 },
        aavsop87::VSOP87Coefficient { a: 222.0, b: 3.194, c: 3.523 },
        aavsop87::VSOP87Coefficient { a: 121.0, b: 0.543, c: 155.420 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 3.49, c: 16703.06 },
        aavsop87::VSOP87Coefficient { a: 54.0, b: 3.54, c: 3344.14 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 6.00, c: 2281.23 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.14, c: 191.45 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 2.00, c: 796.30 },
        aavsop87::VSOP87Coefficient { a: 23.0, b: 4.33, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 22.0, b: 3.45, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 5.42, c: 553.57 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 0.66, c: 0.98 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 6.11, c: 2146.17 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 1.22, c: 1748.02 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 6.10, c: 3185.19 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 4.02, c: 951.72 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 2.62, c: 1349.87 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 0.60, c: 1194.45 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.86, c: 6684.75 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 4.72, c: 2544.31 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 0.25, c: 382.90 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.68, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.83, c: 20043.67 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.88, c: 3738.76 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.46, c: 1751.54 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 2.58, c: 3149.16 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 2.38, c: 4136.91 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.48, c: 1592.60 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.34, c: 3097.88 }
    ];

const G_L3MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 12] =
    [
        aavsop87::VSOP87Coefficient { a: 1482.0, b: 0.4443, c: 3340.6124 },
        aavsop87::VSOP87Coefficient { a: 662.0, b: 0.885, c: 6681.225 },
        aavsop87::VSOP87Coefficient { a: 188.0, b: 1.288, c: 10021.837 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 1.65, c: 13362.45 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 23.0, b: 2.05, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 1.58, c: 3.52 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.00, c: 16703.06 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 2.82, c: 242.73 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 2.02, c: 3344.14 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.59, c: 3185.19 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.65, c: 553.57 }
    ];

const G_L4MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 8] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 5.64, c: 6681.22 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 5.14, c: 3340.61 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 6.03, c: 10021.84 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.13, c: 13362.45 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.56, c: 155.42 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.49, c: 16703.06 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.32, c: 242.73 }
    ];

const G_L5MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.04, c: 6681.22 }
    ];

const G_B0MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 16] =
    [
        aavsop87::VSOP87Coefficient { a: 3197135.0, b: 3.7683204, c: 3340.6124267 },
        aavsop87::VSOP87Coefficient { a: 298033.0, b: 4.106170, c: 6681.224853 },
        aavsop87::VSOP87Coefficient { a: 289105.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 31366.0, b: 4.44651, c: 10021.83728 },
        aavsop87::VSOP87Coefficient { a: 3484.0, b: 4.7881, c: 13362.4497 },
        aavsop87::VSOP87Coefficient { a: 443.0, b: 5.026, c: 3344.136 },
        aavsop87::VSOP87Coefficient { a: 443.0, b: 5.652, c: 3337.089 },
        aavsop87::VSOP87Coefficient { a: 399.0, b: 5.131, c: 16703.062 },
        aavsop87::VSOP87Coefficient { a: 293.0, b: 3.793, c: 2281.230 },
        aavsop87::VSOP87Coefficient { a: 182.0, b: 6.136, c: 6151.534 },
        aavsop87::VSOP87Coefficient { a: 163.0, b: 4.264, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 160.0, b: 2.232, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 2.165, c: 5621.843 },
        aavsop87::VSOP87Coefficient { a: 143.0, b: 1.182, c: 3340.595 },
        aavsop87::VSOP87Coefficient { a: 143.0, b: 3.213, c: 3340.630 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 2.418, c: 8962.455 }
    ];

const G_B1MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 9] =
    [
        aavsop87::VSOP87Coefficient { a: 350069.0, b: 5.368478, c: 3340.612427 },
        aavsop87::VSOP87Coefficient { a: 14116.0, b: 3.14159, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9671.0, b: 5.4788, c: 6681.2249 },
        aavsop87::VSOP87Coefficient { a: 1472.0, b: 3.2021, c: 10021.8373 },
        aavsop87::VSOP87Coefficient { a: 426.0, b: 3.408, c: 13362.450 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 0.776, c: 3337.089 },
        aavsop87::VSOP87Coefficient { a: 79.0, b: 3.72, c: 16703.06 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 3.46, c: 5621.84 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 2.48, c: 2281.23 }
    ];

const G_B2MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 16727.0, b: 0.60221, c: 3340.61243 },
        aavsop87::VSOP87Coefficient { a: 4987.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 302.0, b: 5.559, c: 6681.225 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 1.90, c: 13362.45 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 0.92, c: 10021.84 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.24, c: 3337.09 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.25, c: 16703.06 }
    ];

const G_B3MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 607.0, b: 1.981, c: 3340.612 },
        aavsop87::VSOP87Coefficient { a: 43.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 1.80, c: 6681.22 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.45, c: 10021.84 }
    ];

const G_B4MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 3] =
    [
        aavsop87::VSOP87Coefficient { a: 13.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 3.46, c: 3340.61 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.50, c: 6681.22 }
    ];

const G_R0MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 45] =
    [
        aavsop87::VSOP87Coefficient { a: 153033488.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 14184953.0, b: 3.47971284, c: 3340.61242670 },
        aavsop87::VSOP87Coefficient { a: 660776.0, b: 3.817834, c: 6681.224853 },
        aavsop87::VSOP87Coefficient { a: 46179.0, b: 4.15595, c: 10021.83728 },
        aavsop87::VSOP87Coefficient { a: 8110.0, b: 5.5596, c: 2810.9215 },
        aavsop87::VSOP87Coefficient { a: 7485.0, b: 1.7724, c: 5621.8429 },
        aavsop87::VSOP87Coefficient { a: 5523.0, b: 1.3644, c: 2281.2305 },
        aavsop87::VSOP87Coefficient { a: 3825.0, b: 4.4941, c: 13362.4497 },
        aavsop87::VSOP87Coefficient { a: 2484.0, b: 4.9255, c: 2942.4634 },
        aavsop87::VSOP87Coefficient { a: 2307.0, b: 0.0908, c: 2544.3144 },
        aavsop87::VSOP87Coefficient { a: 1999.0, b: 5.3606, c: 3337.0893 },
        aavsop87::VSOP87Coefficient { a: 1960.0, b: 4.7425, c: 3344.1355 },
        aavsop87::VSOP87Coefficient { a: 1167.0, b: 2.1126, c: 5092.1520 },
        aavsop87::VSOP87Coefficient { a: 1103.0, b: 5.0091, c: 398.1490 },
        aavsop87::VSOP87Coefficient { a: 992.0, b: 5.839, c: 6151.534 },
        aavsop87::VSOP87Coefficient { a: 899.0, b: 4.408, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 807.0, b: 2.102, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 798.0, b: 3.448, c: 796.298 },
        aavsop87::VSOP87Coefficient { a: 741.0, b: 1.499, c: 2146.165 },
        aavsop87::VSOP87Coefficient { a: 726.0, b: 1.245, c: 8432.764 },
        aavsop87::VSOP87Coefficient { a: 692.0, b: 2.134, c: 8962.455 },
        aavsop87::VSOP87Coefficient { a: 633.0, b: 0.894, c: 3340.595 },
        aavsop87::VSOP87Coefficient { a: 633.0, b: 2.924, c: 3340.630 },
        aavsop87::VSOP87Coefficient { a: 630.0, b: 1.287, c: 1751.540 },
        aavsop87::VSOP87Coefficient { a: 574.0, b: 0.829, c: 2914.014 },
        aavsop87::VSOP87Coefficient { a: 526.0, b: 5.383, c: 3738.761 },
        aavsop87::VSOP87Coefficient { a: 473.0, b: 5.199, c: 3127.313 },
        aavsop87::VSOP87Coefficient { a: 348.0, b: 4.832, c: 16703.062 },
        aavsop87::VSOP87Coefficient { a: 284.0, b: 2.907, c: 3532.061 },
        aavsop87::VSOP87Coefficient { a: 280.0, b: 5.257, c: 6283.076 },
        aavsop87::VSOP87Coefficient { a: 276.0, b: 1.218, c: 6254.627 },
        aavsop87::VSOP87Coefficient { a: 275.0, b: 2.908, c: 1748.016 },
        aavsop87::VSOP87Coefficient { a: 270.0, b: 3.764, c: 5884.927 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 2.037, c: 1194.447 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 5.105, c: 5486.778 },
        aavsop87::VSOP87Coefficient { a: 228.0, b: 3.255, c: 6872.673 },
        aavsop87::VSOP87Coefficient { a: 223.0, b: 4.199, c: 3149.164 },
        aavsop87::VSOP87Coefficient { a: 219.0, b: 5.583, c: 191.448 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 5.255, c: 3340.545 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 4.846, c: 3340.680 },
        aavsop87::VSOP87Coefficient { a: 186.0, b: 5.699, c: 6677.702 },
        aavsop87::VSOP87Coefficient { a: 183.0, b: 5.081, c: 6684.748 },
        aavsop87::VSOP87Coefficient { a: 179.0, b: 4.184, c: 3333.499 },
        aavsop87::VSOP87Coefficient { a: 176.0, b: 5.953, c: 3870.303 },
        aavsop87::VSOP87Coefficient { a: 164.0, b: 3.799, c: 4136.910 }
    ];

const G_R1MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 27] =
    [
        aavsop87::VSOP87Coefficient { a: 1107433.0, b: 2.0325052, c: 3340.6124267 },
        aavsop87::VSOP87Coefficient { a: 103176.0, b: 2.370718, c: 6681.224853 },
        aavsop87::VSOP87Coefficient { a: 12877.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 10816.0, b: 2.70888, c: 10021.83728 },
        aavsop87::VSOP87Coefficient { a: 1195.0, b: 3.0470, c: 13362.4497 },
        aavsop87::VSOP87Coefficient { a: 439.0, b: 2.888, c: 2281.230 },
        aavsop87::VSOP87Coefficient { a: 396.0, b: 3.423, c: 3344.136 },
        aavsop87::VSOP87Coefficient { a: 183.0, b: 1.584, c: 2544.314 },
        aavsop87::VSOP87Coefficient { a: 136.0, b: 3.385, c: 16703.062 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 6.043, c: 3337.089 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 0.630, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 127.0, b: 1.954, c: 796.298 },
        aavsop87::VSOP87Coefficient { a: 118.0, b: 2.998, c: 2146.165 },
        aavsop87::VSOP87Coefficient { a: 88.0, b: 3.42, c: 398.15 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 3.86, c: 3738.76 },
        aavsop87::VSOP87Coefficient { a: 76.0, b: 4.45, c: 6151.53 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 2.76, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 2.55, c: 1751.54 },
        aavsop87::VSOP87Coefficient { a: 66.0, b: 4.41, c: 1748.02 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 0.54, c: 1194.45 },
        aavsop87::VSOP87Coefficient { a: 54.0, b: 0.68, c: 8962.46 },
        aavsop87::VSOP87Coefficient { a: 51.0, b: 3.73, c: 6684.75 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 5.73, c: 3340.60 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 1.48, c: 3340.63 },
        aavsop87::VSOP87Coefficient { a: 48.0, b: 2.58, c: 3149.16 },
        aavsop87::VSOP87Coefficient { a: 48.0, b: 2.29, c: 2914.01 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 2.32, c: 4136.91 }
    ];

const G_R2MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 11] =
    [
        aavsop87::VSOP87Coefficient { a: 44242.0, b: 0.47931, c: 3340.61243 },
        aavsop87::VSOP87Coefficient { a: 8138.0, b: 0.8700, c: 6681.2249 },
        aavsop87::VSOP87Coefficient { a: 1275.0, b: 1.2259, c: 10021.8373 },
        aavsop87::VSOP87Coefficient { a: 187.0, b: 1.573, c: 13362.450 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 1.97, c: 3344.14 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 1.92, c: 16703.06 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 4.43, c: 2281.23 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 4.53, c: 3185.19 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 5.39, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 0.42, c: 796.30 }
    ];

const G_R3MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 6] =
    [
        aavsop87::VSOP87Coefficient { a: 1113.0, b: 5.1499, c: 3340.6124 },
        aavsop87::VSOP87Coefficient { a: 424.0, b: 5.613, c: 6681.225 },
        aavsop87::VSOP87Coefficient { a: 100.0, b: 5.997, c: 10021.837 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 0.08, c: 13362.45 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.43, c: 16703.06 }
    ];

const G_R4MARS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 20.0, b: 3.58, c: 3340.61 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 4.05, c: 6681.22 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.46, c: 10021.84 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.84, c: 13362.45 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_mar::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate l0
    let n_l0coefficients = G_L0MARS_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0MARS_COEFFICIENTS[i].a * f64::cos(G_L0MARS_COEFFICIENTS[i].b + G_L0MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1MARS_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1MARS_COEFFICIENTS[i].a * f64::cos(G_L1MARS_COEFFICIENTS[i].b + G_L1MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2MARS_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2MARS_COEFFICIENTS[i].a * f64::cos(G_L2MARS_COEFFICIENTS[i].b + G_L2MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3MARS_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3MARS_COEFFICIENTS[i].a * f64::cos(G_L3MARS_COEFFICIENTS[i].b + G_L3MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4MARS_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4MARS_COEFFICIENTS[i].a * f64::cos(G_L4MARS_COEFFICIENTS[i].b + G_L4MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l5
    let n_l5coefficients = G_L5MARS_COEFFICIENTS.len();
    let mut l5 = 0.0f64;
    for i in 0..n_l5coefficients {
        l5 += G_L5MARS_COEFFICIENTS[i].a * f64::cos(G_L5MARS_COEFFICIENTS[i].b + G_L5MARS_COEFFICIENTS[i].c * rho);
    }

    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_mar::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate b0
    let n_b0coefficients = G_B0MARS_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0MARS_COEFFICIENTS[i].a * f64::cos(G_B0MARS_COEFFICIENTS[i].b + G_B0MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1MARS_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1MARS_COEFFICIENTS[i].a * f64::cos(G_B1MARS_COEFFICIENTS[i].b + G_B1MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2MARS_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2MARS_COEFFICIENTS[i].a * f64::cos(G_B2MARS_COEFFICIENTS[i].b + G_B2MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3MARS_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3MARS_COEFFICIENTS[i].a * f64::cos(G_B3MARS_COEFFICIENTS[i].b + G_B3MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4MARS_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4MARS_COEFFICIENTS[i].a * f64::cos(G_B4MARS_COEFFICIENTS[i].b + G_B4MARS_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_mar::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate r0
    let n_r0coefficients = G_R0MARS_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0MARS_COEFFICIENTS[i].a * f64::cos(G_R0MARS_COEFFICIENTS[i].b + G_R0MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1MARS_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1MARS_COEFFICIENTS[i].a * f64::cos(G_R1MARS_COEFFICIENTS[i].b + G_R1MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2MARS_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2MARS_COEFFICIENTS[i].a * f64::cos(G_R2MARS_COEFFICIENTS[i].b + G_R2MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3MARS_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3MARS_COEFFICIENTS[i].a * f64::cos(G_R3MARS_COEFFICIENTS[i].b + G_R3MARS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r4
    let n_r4coefficients = G_R4MARS_COEFFICIENTS.len();
    let mut r4 = 0.0f64;
    for i in 0..n_r4coefficients {
        r4 += G_R4MARS_COEFFICIENTS[i].a * f64::cos(G_R4MARS_COEFFICIENTS[i].b + G_R4MARS_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4) / 100000000.0
}
