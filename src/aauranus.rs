use super::aavsop87;
use super::aacoordinatetransformation;
use super::aavsop87d_ura;

const G_L0URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 91] =
    [
        aavsop87::VSOP87Coefficient { a: 548129294.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9260408.0, b: 0.8910642, c: 74.7815986 },
        aavsop87::VSOP87Coefficient { a: 1504248.0, b: 3.6271926, c: 1.4844727 },
        aavsop87::VSOP87Coefficient { a: 365982.0, b: 1.899622, c: 73.297126 },
        aavsop87::VSOP87Coefficient { a: 272328.0, b: 3.358237, c: 149.563197 },
        aavsop87::VSOP87Coefficient { a: 70328.0, b: 5.39254, c: 63.73590 },
        aavsop87::VSOP87Coefficient { a: 68893.0, b: 6.09292, c: 76.26607 },
        aavsop87::VSOP87Coefficient { a: 61999.0, b: 2.26952, c: 2.96895 },
        aavsop87::VSOP87Coefficient { a: 61951.0, b: 2.85099, c: 11.04570 },
        aavsop87::VSOP87Coefficient { a: 26469.0, b: 3.14152, c: 71.81265 },
        aavsop87::VSOP87Coefficient { a: 25711.0, b: 6.11380, c: 454.90937 },
        aavsop87::VSOP87Coefficient { a: 21079.0, b: 4.36059, c: 148.07872 },
        aavsop87::VSOP87Coefficient { a: 17819.0, b: 1.74437, c: 36.64856 },
        aavsop87::VSOP87Coefficient { a: 14613.0, b: 4.73732, c: 3.93215 },
        aavsop87::VSOP87Coefficient { a: 11163.0, b: 5.82682, c: 224.34480 },
        aavsop87::VSOP87Coefficient { a: 10998.0, b: 0.48865, c: 138.51750 },
        aavsop87::VSOP87Coefficient { a: 9527.0, b: 2.9552, c: 35.1641 },
        aavsop87::VSOP87Coefficient { a: 7546.0, b: 5.2363, c: 109.9457 },
        aavsop87::VSOP87Coefficient { a: 4220.0, b: 3.2333, c: 70.8494 },
        aavsop87::VSOP87Coefficient { a: 4052.0, b: 2.2775, c: 151.0477 },
        aavsop87::VSOP87Coefficient { a: 3490.0, b: 5.4831, c: 146.5943 },
        aavsop87::VSOP87Coefficient { a: 3355.0, b: 1.0655, c: 4.4534 },
        aavsop87::VSOP87Coefficient { a: 3144.0, b: 4.7520, c: 77.7505 },
        aavsop87::VSOP87Coefficient { a: 2927.0, b: 4.6290, c: 9.5612 },
        aavsop87::VSOP87Coefficient { a: 2922.0, b: 5.3524, c: 85.8273 },
        aavsop87::VSOP87Coefficient { a: 2273.0, b: 4.3660, c: 70.3282 },
        aavsop87::VSOP87Coefficient { a: 2149.0, b: 0.6075, c: 38.1330 },
        aavsop87::VSOP87Coefficient { a: 2051.0, b: 1.5177, c: 0.1119 },
        aavsop87::VSOP87Coefficient { a: 1992.0, b: 4.9244, c: 277.0350 },
        aavsop87::VSOP87Coefficient { a: 1667.0, b: 3.6274, c: 380.1278 },
        aavsop87::VSOP87Coefficient { a: 1533.0, b: 2.5859, c: 52.6902 },
        aavsop87::VSOP87Coefficient { a: 1376.0, b: 2.0428, c: 65.2204 },
        aavsop87::VSOP87Coefficient { a: 1372.0, b: 4.1964, c: 111.4302 },
        aavsop87::VSOP87Coefficient { a: 1284.0, b: 3.1135, c: 202.2534 },
        aavsop87::VSOP87Coefficient { a: 1282.0, b: 0.5427, c: 222.8603 },
        aavsop87::VSOP87Coefficient { a: 1244.0, b: 0.9161, c: 2.4477 },
        aavsop87::VSOP87Coefficient { a: 1221.0, b: 0.1990, c: 108.4612 },
        aavsop87::VSOP87Coefficient { a: 1151.0, b: 4.1790, c: 33.6796 },
        aavsop87::VSOP87Coefficient { a: 1150.0, b: 0.9334, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 1090.0, b: 1.7750, c: 12.5302 },
        aavsop87::VSOP87Coefficient { a: 1072.0, b: 0.2356, c: 62.2514 },
        aavsop87::VSOP87Coefficient { a: 946.0, b: 1.192, c: 127.472 },
        aavsop87::VSOP87Coefficient { a: 708.0, b: 5.183, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 653.0, b: 0.966, c: 78.714 },
        aavsop87::VSOP87Coefficient { a: 628.0, b: 0.182, c: 984.600 },
        aavsop87::VSOP87Coefficient { a: 607.0, b: 5.432, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 559.0, b: 3.358, c: 0.521 },
        aavsop87::VSOP87Coefficient { a: 524.0, b: 2.013, c: 299.126 },
        aavsop87::VSOP87Coefficient { a: 483.0, b: 2.106, c: 0.963 },
        aavsop87::VSOP87Coefficient { a: 471.0, b: 1.407, c: 184.727 },
        aavsop87::VSOP87Coefficient { a: 467.0, b: 0.415, c: 145.110 },
        aavsop87::VSOP87Coefficient { a: 434.0, b: 5.521, c: 183.243 },
        aavsop87::VSOP87Coefficient { a: 405.0, b: 5.987, c: 8.077 },
        aavsop87::VSOP87Coefficient { a: 399.0, b: 0.338, c: 415.552 },
        aavsop87::VSOP87Coefficient { a: 396.0, b: 5.870, c: 351.817 },
        aavsop87::VSOP87Coefficient { a: 379.0, b: 2.350, c: 56.622 },
        aavsop87::VSOP87Coefficient { a: 310.0, b: 5.833, c: 145.631 },
        aavsop87::VSOP87Coefficient { a: 300.0, b: 5.644, c: 22.091 },
        aavsop87::VSOP87Coefficient { a: 294.0, b: 5.839, c: 39.618 },
        aavsop87::VSOP87Coefficient { a: 252.0, b: 1.637, c: 221.376 },
        aavsop87::VSOP87Coefficient { a: 249.0, b: 4.746, c: 225.829 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 2.350, c: 137.033 },
        aavsop87::VSOP87Coefficient { a: 224.0, b: 0.516, c: 84.343 },
        aavsop87::VSOP87Coefficient { a: 223.0, b: 2.843, c: 0.261 },
        aavsop87::VSOP87Coefficient { a: 220.0, b: 1.922, c: 67.668 },
        aavsop87::VSOP87Coefficient { a: 217.0, b: 6.142, c: 5.938 },
        aavsop87::VSOP87Coefficient { a: 216.0, b: 4.778, c: 340.771 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 5.580, c: 68.844 },
        aavsop87::VSOP87Coefficient { a: 202.0, b: 1.297, c: 0.048 },
        aavsop87::VSOP87Coefficient { a: 199.0, b: 0.956, c: 152.532 },
        aavsop87::VSOP87Coefficient { a: 194.0, b: 1.888, c: 456.394 },
        aavsop87::VSOP87Coefficient { a: 193.0, b: 0.916, c: 453.425 },
        aavsop87::VSOP87Coefficient { a: 187.0, b: 1.319, c: 0.160 },
        aavsop87::VSOP87Coefficient { a: 182.0, b: 3.536, c: 79.235 },
        aavsop87::VSOP87Coefficient { a: 173.0, b: 1.539, c: 160.609 },
        aavsop87::VSOP87Coefficient { a: 172.0, b: 5.680, c: 219.891 },
        aavsop87::VSOP87Coefficient { a: 170.0, b: 3.677, c: 5.417 },
        aavsop87::VSOP87Coefficient { a: 169.0, b: 5.879, c: 18.159 },
        aavsop87::VSOP87Coefficient { a: 165.0, b: 1.424, c: 106.977 },
        aavsop87::VSOP87Coefficient { a: 163.0, b: 3.050, c: 112.915 },
        aavsop87::VSOP87Coefficient { a: 158.0, b: 0.738, c: 54.175 },
        aavsop87::VSOP87Coefficient { a: 147.0, b: 1.263, c: 59.804 },
        aavsop87::VSOP87Coefficient { a: 143.0, b: 1.300, c: 35.425 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 5.386, c: 32.195 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 4.260, c: 909.819 },
        aavsop87::VSOP87Coefficient { a: 124.0, b: 1.374, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 2.027, c: 554.070 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 5.706, c: 77.963 },
        aavsop87::VSOP87Coefficient { a: 104.0, b: 5.028, c: 0.751 },
        aavsop87::VSOP87Coefficient { a: 104.0, b: 1.458, c: 24.379 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 0.681, c: 14.978 }
    ];

const G_L1URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 57] =
    [
        aavsop87::VSOP87Coefficient { a: 7502543122.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 154458.0, b: 5.242017, c: 74.781599 },
        aavsop87::VSOP87Coefficient { a: 24456.0, b: 1.71256, c: 1.48447 },
        aavsop87::VSOP87Coefficient { a: 9258.0, b: 0.4284, c: 11.0457 },
        aavsop87::VSOP87Coefficient { a: 8266.0, b: 1.5022, c: 63.7359 },
        aavsop87::VSOP87Coefficient { a: 7842.0, b: 1.3198, c: 149.5632 },
        aavsop87::VSOP87Coefficient { a: 3899.0, b: 0.4648, c: 3.9322 },
        aavsop87::VSOP87Coefficient { a: 2284.0, b: 4.1737, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 1927.0, b: 0.5301, c: 2.9689 },
        aavsop87::VSOP87Coefficient { a: 1233.0, b: 1.5863, c: 70.8494 },
        aavsop87::VSOP87Coefficient { a: 791.0, b: 5.436, c: 3.181 },
        aavsop87::VSOP87Coefficient { a: 767.0, b: 1.996, c: 73.297 },
        aavsop87::VSOP87Coefficient { a: 482.0, b: 2.984, c: 85.827 },
        aavsop87::VSOP87Coefficient { a: 450.0, b: 4.138, c: 138.517 },
        aavsop87::VSOP87Coefficient { a: 446.0, b: 3.723, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 427.0, b: 4.731, c: 71.813 },
        aavsop87::VSOP87Coefficient { a: 354.0, b: 2.583, c: 148.079 },
        aavsop87::VSOP87Coefficient { a: 348.0, b: 2.454, c: 9.561 },
        aavsop87::VSOP87Coefficient { a: 317.0, b: 5.579, c: 52.690 },
        aavsop87::VSOP87Coefficient { a: 206.0, b: 2.363, c: 2.448 },
        aavsop87::VSOP87Coefficient { a: 189.0, b: 4.202, c: 56.622 },
        aavsop87::VSOP87Coefficient { a: 184.0, b: 0.284, c: 151.048 },
        aavsop87::VSOP87Coefficient { a: 180.0, b: 5.684, c: 12.530 },
        aavsop87::VSOP87Coefficient { a: 171.0, b: 3.001, c: 78.714 },
        aavsop87::VSOP87Coefficient { a: 158.0, b: 2.909, c: 0.963 },
        aavsop87::VSOP87Coefficient { a: 155.0, b: 5.591, c: 4.453 },
        aavsop87::VSOP87Coefficient { a: 154.0, b: 4.652, c: 35.164 },
        aavsop87::VSOP87Coefficient { a: 152.0, b: 2.942, c: 77.751 },
        aavsop87::VSOP87Coefficient { a: 143.0, b: 2.590, c: 62.251 },
        aavsop87::VSOP87Coefficient { a: 121.0, b: 4.148, c: 127.472 },
        aavsop87::VSOP87Coefficient { a: 116.0, b: 3.732, c: 65.220 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 4.188, c: 145.631 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 6.034, c: 0.112 },
        aavsop87::VSOP87Coefficient { a: 88.0, b: 3.99, c: 18.16 },
        aavsop87::VSOP87Coefficient { a: 88.0, b: 6.16, c: 202.25 },
        aavsop87::VSOP87Coefficient { a: 81.0, b: 2.64, c: 22.09 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 6.05, c: 70.33 },
        aavsop87::VSOP87Coefficient { a: 69.0, b: 4.05, c: 77.96 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 3.70, c: 67.67 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 3.54, c: 351.82 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 5.91, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 43.0, b: 5.72, c: 5.42 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 4.92, c: 222.86 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 5.90, c: 33.68 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 3.29, c: 8.08 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 3.33, c: 71.60 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 5.08, c: 38.13 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 5.62, c: 984.60 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 5.50, c: 59.80 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 5.46, c: 160.61 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 1.66, c: 447.80 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 1.15, c: 462.02 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 4.52, c: 84.34 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.54, c: 131.40 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 6.15, c: 299.13 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 4.99, c: 137.03 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 5.74, c: 380.13 }
    ];

const G_L2URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 35] =
    [
        aavsop87::VSOP87Coefficient { a: 53033.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 2358.0, b: 2.2601, c: 74.7816 },
        aavsop87::VSOP87Coefficient { a: 769.0, b: 4.526, c: 11.046 },
        aavsop87::VSOP87Coefficient { a: 552.0, b: 3.258, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 542.0, b: 2.276, c: 3.932 },
        aavsop87::VSOP87Coefficient { a: 529.0, b: 4.923, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 258.0, b: 3.691, c: 3.181 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 5.858, c: 149.563 },
        aavsop87::VSOP87Coefficient { a: 182.0, b: 6.218, c: 70.849 },
        aavsop87::VSOP87Coefficient { a: 54.0, b: 1.44, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 6.03, c: 56.62 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 3.91, c: 2.45 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 0.81, c: 85.83 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 1.78, c: 52.69 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 4.46, c: 2.97 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 0.86, c: 9.56 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 5.10, c: 73.30 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 2.11, c: 18.16 },
        aavsop87::VSOP87Coefficient { a: 22.0, b: 5.99, c: 138.52 },
        aavsop87::VSOP87Coefficient { a: 22.0, b: 4.82, c: 78.71 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 2.40, c: 77.96 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 2.17, c: 224.34 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.54, c: 145.63 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 3.47, c: 12.53 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 0.02, c: 22.09 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 0.08, c: 127.47 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 5.16, c: 71.60 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 4.46, c: 62.25 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 4.26, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.50, c: 67.67 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 1.25, c: 5.42 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 3.36, c: 447.80 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.45, c: 65.22 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.52, c: 151.05 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.73, c: 462.02 }
    ];

const G_L3URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 18] =
    [
        aavsop87::VSOP87Coefficient { a: 121.0, b: 0.024, c: 74.782 },
        aavsop87::VSOP87Coefficient { a: 68.0, b: 4.12, c: 3.93 },
        aavsop87::VSOP87Coefficient { a: 53.0, b: 2.39, c: 11.05 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 2.04, c: 3.18 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 2.96, c: 1.48 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 4.89, c: 63.74 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 4.55, c: 70.85 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 2.31, c: 149.56 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 1.58, c: 56.62 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 0.23, c: 18.16 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 5.39, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 0.95, c: 77.96 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.98, c: 85.83 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.13, c: 52.69 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.37, c: 78.71 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 0.86, c: 145.63 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.66, c: 9.56 }
    ];

const G_L4URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.58, c: 74.78 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.35, c: 11.05 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.42, c: 56.62 }
    ];

const G_B0URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 28] =
    [
        aavsop87::VSOP87Coefficient { a: 1346278.0, b: 2.6187781, c: 74.7815986 },
        aavsop87::VSOP87Coefficient { a: 62341.0, b: 5.08111, c: 149.56320 },
        aavsop87::VSOP87Coefficient { a: 61601.0, b: 3.14159, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9964.0, b: 1.6160, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 9926.0, b: 0.5763, c: 73.2971 },
        aavsop87::VSOP87Coefficient { a: 3259.0, b: 1.2612, c: 224.3448 },
        aavsop87::VSOP87Coefficient { a: 2972.0, b: 2.2437, c: 1.4845 },
        aavsop87::VSOP87Coefficient { a: 2010.0, b: 6.0555, c: 148.0787 },
        aavsop87::VSOP87Coefficient { a: 1522.0, b: 0.2796, c: 63.7359 },
        aavsop87::VSOP87Coefficient { a: 924.0, b: 4.038, c: 151.048 },
        aavsop87::VSOP87Coefficient { a: 761.0, b: 6.140, c: 71.813 },
        aavsop87::VSOP87Coefficient { a: 522.0, b: 3.321, c: 138.517 },
        aavsop87::VSOP87Coefficient { a: 463.0, b: 0.743, c: 85.827 },
        aavsop87::VSOP87Coefficient { a: 437.0, b: 3.381, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 435.0, b: 0.341, c: 77.751 },
        aavsop87::VSOP87Coefficient { a: 431.0, b: 3.554, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 420.0, b: 5.213, c: 11.046 },
        aavsop87::VSOP87Coefficient { a: 245.0, b: 0.788, c: 2.969 },
        aavsop87::VSOP87Coefficient { a: 233.0, b: 2.257, c: 222.860 },
        aavsop87::VSOP87Coefficient { a: 216.0, b: 1.591, c: 38.133 },
        aavsop87::VSOP87Coefficient { a: 180.0, b: 3.725, c: 299.126 },
        aavsop87::VSOP87Coefficient { a: 175.0, b: 1.236, c: 146.594 },
        aavsop87::VSOP87Coefficient { a: 174.0, b: 1.937, c: 380.128 },
        aavsop87::VSOP87Coefficient { a: 160.0, b: 5.336, c: 111.430 },
        aavsop87::VSOP87Coefficient { a: 144.0, b: 5.962, c: 35.164 },
        aavsop87::VSOP87Coefficient { a: 116.0, b: 5.739, c: 70.849 },
        aavsop87::VSOP87Coefficient { a: 106.0, b: 0.941, c: 70.328 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 2.619, c: 78.714 }
    ];

const G_B1URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 20] =
    [
        aavsop87::VSOP87Coefficient { a: 206366.0, b: 4.123943, c: 74.781599 },
        aavsop87::VSOP87Coefficient { a: 8563.0, b: 0.3382, c: 149.5632 },
        aavsop87::VSOP87Coefficient { a: 1726.0, b: 2.1219, c: 73.2971 },
        aavsop87::VSOP87Coefficient { a: 1374.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1369.0, b: 3.0686, c: 76.2661 },
        aavsop87::VSOP87Coefficient { a: 451.0, b: 3.777, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 400.0, b: 2.848, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 307.0, b: 1.255, c: 148.079 },
        aavsop87::VSOP87Coefficient { a: 154.0, b: 3.786, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 112.0, b: 5.573, c: 151.048 },
        aavsop87::VSOP87Coefficient { a: 111.0, b: 5.329, c: 138.517 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 3.59, c: 71.81 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 3.40, c: 85.83 },
        aavsop87::VSOP87Coefficient { a: 54.0, b: 1.70, c: 77.75 },
        aavsop87::VSOP87Coefficient { a: 42.0, b: 1.21, c: 11.05 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 4.45, c: 78.71 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 3.77, c: 222.86 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 2.56, c: 2.97 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.34, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 0.42, c: 380.13 }
    ];

const G_B2URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 11] =
    [
        aavsop87::VSOP87Coefficient { a: 9212.0, b: 5.8004, c: 74.7816 },
        aavsop87::VSOP87Coefficient { a: 557.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 286.0, b: 2.177, c: 149.563 },
        aavsop87::VSOP87Coefficient { a: 95.0, b: 3.84, c: 73.30 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 4.88, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 5.46, c: 1.48 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 0.88, c: 138.52 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 2.85, c: 148.08 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 5.07, c: 63.74 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 5.00, c: 224.34 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 6.27, c: 78.71 }
    ];

const G_B3URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 4] =
    [
        aavsop87::VSOP87Coefficient { a: 268.0, b: 1.251, c: 74.782 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.01, c: 149.56 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.78, c: 73.30 }
    ];

const G_B4URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.85, c: 74.78 }
    ];

const G_R0URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 59] =
    [
        aavsop87::VSOP87Coefficient { a: 1921264848.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 88784984.0, b: 5.60377527, c: 74.78159857 },
        aavsop87::VSOP87Coefficient { a: 3440836.0, b: 0.3283610, c: 73.2971259 },
        aavsop87::VSOP87Coefficient { a: 2055653.0, b: 1.7829517, c: 149.5631971 },
        aavsop87::VSOP87Coefficient { a: 649322.0, b: 4.522473, c: 76.266071 },
        aavsop87::VSOP87Coefficient { a: 602248.0, b: 3.860038, c: 63.735898 },
        aavsop87::VSOP87Coefficient { a: 496404.0, b: 1.401399, c: 454.909367 },
        aavsop87::VSOP87Coefficient { a: 338526.0, b: 1.580027, c: 138.517497 },
        aavsop87::VSOP87Coefficient { a: 243508.0, b: 1.570866, c: 71.812653 },
        aavsop87::VSOP87Coefficient { a: 190522.0, b: 1.998094, c: 1.484473 },
        aavsop87::VSOP87Coefficient { a: 161858.0, b: 2.791379, c: 148.078724 },
        aavsop87::VSOP87Coefficient { a: 143706.0, b: 1.383686, c: 11.045700 },
        aavsop87::VSOP87Coefficient { a: 93192.0, b: 0.17437, c: 36.64856 },
        aavsop87::VSOP87Coefficient { a: 89806.0, b: 3.66105, c: 109.94569 },
        aavsop87::VSOP87Coefficient { a: 71424.0, b: 4.24509, c: 224.34480 },
        aavsop87::VSOP87Coefficient { a: 46677.0, b: 1.39977, c: 35.16409 },
        aavsop87::VSOP87Coefficient { a: 39026.0, b: 3.36235, c: 277.03499 },
        aavsop87::VSOP87Coefficient { a: 39010.0, b: 1.66971, c: 70.84945 },
        aavsop87::VSOP87Coefficient { a: 36755.0, b: 3.88649, c: 146.59425 },
        aavsop87::VSOP87Coefficient { a: 30349.0, b: 0.70100, c: 151.04767 },
        aavsop87::VSOP87Coefficient { a: 29156.0, b: 3.18056, c: 77.75054 },
        aavsop87::VSOP87Coefficient { a: 25786.0, b: 3.78538, c: 85.82730 },
        aavsop87::VSOP87Coefficient { a: 25620.0, b: 5.25656, c: 380.12777 },
        aavsop87::VSOP87Coefficient { a: 22637.0, b: 0.72519, c: 529.69097 },
        aavsop87::VSOP87Coefficient { a: 20473.0, b: 2.79640, c: 70.32818 },
        aavsop87::VSOP87Coefficient { a: 20472.0, b: 1.55589, c: 202.25340 },
        aavsop87::VSOP87Coefficient { a: 17901.0, b: 0.55455, c: 2.96895 },
        aavsop87::VSOP87Coefficient { a: 15503.0, b: 5.35405, c: 38.13304 },
        aavsop87::VSOP87Coefficient { a: 14702.0, b: 4.90434, c: 108.46122 },
        aavsop87::VSOP87Coefficient { a: 12897.0, b: 2.62154, c: 111.43016 },
        aavsop87::VSOP87Coefficient { a: 12328.0, b: 5.96039, c: 127.47180 },
        aavsop87::VSOP87Coefficient { a: 11959.0, b: 1.75044, c: 984.60033 },
        aavsop87::VSOP87Coefficient { a: 11853.0, b: 0.99343, c: 52.69020 },
        aavsop87::VSOP87Coefficient { a: 11696.0, b: 3.29826, c: 3.93215 },
        aavsop87::VSOP87Coefficient { a: 11495.0, b: 0.43774, c: 65.22037 },
        aavsop87::VSOP87Coefficient { a: 10793.0, b: 1.42105, c: 213.29910 },
        aavsop87::VSOP87Coefficient { a: 9111.0, b: 4.9964, c: 62.2514 },
        aavsop87::VSOP87Coefficient { a: 8421.0, b: 5.2535, c: 222.8603 },
        aavsop87::VSOP87Coefficient { a: 8402.0, b: 5.0388, c: 415.5525 },
        aavsop87::VSOP87Coefficient { a: 7449.0, b: 0.7949, c: 351.8166 },
        aavsop87::VSOP87Coefficient { a: 7329.0, b: 3.9728, c: 183.2428 },
        aavsop87::VSOP87Coefficient { a: 6046.0, b: 5.6796, c: 78.7138 },
        aavsop87::VSOP87Coefficient { a: 5524.0, b: 3.1150, c: 9.5612 },
        aavsop87::VSOP87Coefficient { a: 5445.0, b: 5.1058, c: 145.1098 },
        aavsop87::VSOP87Coefficient { a: 5238.0, b: 2.6296, c: 33.6796 },
        aavsop87::VSOP87Coefficient { a: 4079.0, b: 3.2206, c: 340.7709 },
        aavsop87::VSOP87Coefficient { a: 3919.0, b: 4.2502, c: 39.6175 },
        aavsop87::VSOP87Coefficient { a: 3802.0, b: 6.1099, c: 184.7273 },
        aavsop87::VSOP87Coefficient { a: 3781.0, b: 3.4584, c: 456.3938 },
        aavsop87::VSOP87Coefficient { a: 3687.0, b: 2.4872, c: 453.4249 },
        aavsop87::VSOP87Coefficient { a: 3102.0, b: 4.1403, c: 219.8914 },
        aavsop87::VSOP87Coefficient { a: 2963.0, b: 0.8298, c: 56.6224 },
        aavsop87::VSOP87Coefficient { a: 2942.0, b: 0.4239, c: 299.1264 },
        aavsop87::VSOP87Coefficient { a: 2940.0, b: 2.1464, c: 137.0330 },
        aavsop87::VSOP87Coefficient { a: 2938.0, b: 3.6766, c: 140.0020 },
        aavsop87::VSOP87Coefficient { a: 2865.0, b: 0.3100, c: 12.5302 },
        aavsop87::VSOP87Coefficient { a: 2538.0, b: 4.8546, c: 131.4039 },
        aavsop87::VSOP87Coefficient { a: 2364.0, b: 0.4425, c: 554.0700 },
        aavsop87::VSOP87Coefficient { a: 2183.0, b: 2.9404, c: 305.3462 }
    ];

const G_R1URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 35] =
    [
        aavsop87::VSOP87Coefficient { a: 1479896.0, b: 3.6720571, c: 74.7815986 },
        aavsop87::VSOP87Coefficient { a: 71212.0, b: 6.22601, c: 63.73590 },
        aavsop87::VSOP87Coefficient { a: 68627.0, b: 6.13411, c: 149.56320 },
        aavsop87::VSOP87Coefficient { a: 24060.0, b: 3.14159, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 21468.0, b: 2.60177, c: 76.26607 },
        aavsop87::VSOP87Coefficient { a: 20857.0, b: 5.24625, c: 11.04570 },
        aavsop87::VSOP87Coefficient { a: 11405.0, b: 0.01848, c: 70.84945 },
        aavsop87::VSOP87Coefficient { a: 7497.0, b: 0.4236, c: 73.2971 },
        aavsop87::VSOP87Coefficient { a: 4244.0, b: 1.4169, c: 85.8273 },
        aavsop87::VSOP87Coefficient { a: 3927.0, b: 3.1551, c: 71.8127 },
        aavsop87::VSOP87Coefficient { a: 3578.0, b: 2.3116, c: 224.3448 },
        aavsop87::VSOP87Coefficient { a: 3506.0, b: 2.5835, c: 138.5175 },
        aavsop87::VSOP87Coefficient { a: 3229.0, b: 5.2550, c: 3.9322 },
        aavsop87::VSOP87Coefficient { a: 3060.0, b: 0.1532, c: 1.4845 },
        aavsop87::VSOP87Coefficient { a: 2564.0, b: 0.9808, c: 148.0787 },
        aavsop87::VSOP87Coefficient { a: 2429.0, b: 3.9944, c: 52.6902 },
        aavsop87::VSOP87Coefficient { a: 1645.0, b: 2.6535, c: 127.4718 },
        aavsop87::VSOP87Coefficient { a: 1584.0, b: 1.4305, c: 78.7138 },
        aavsop87::VSOP87Coefficient { a: 1508.0, b: 5.0600, c: 151.0477 },
        aavsop87::VSOP87Coefficient { a: 1490.0, b: 2.6756, c: 56.6224 },
        aavsop87::VSOP87Coefficient { a: 1413.0, b: 4.5746, c: 202.2534 },
        aavsop87::VSOP87Coefficient { a: 1403.0, b: 1.3699, c: 77.7505 },
        aavsop87::VSOP87Coefficient { a: 1228.0, b: 1.0470, c: 62.2514 },
        aavsop87::VSOP87Coefficient { a: 1033.0, b: 0.2646, c: 131.4039 },
        aavsop87::VSOP87Coefficient { a: 992.0, b: 2.172, c: 65.220 },
        aavsop87::VSOP87Coefficient { a: 862.0, b: 5.055, c: 351.817 },
        aavsop87::VSOP87Coefficient { a: 744.0, b: 3.076, c: 35.164 },
        aavsop87::VSOP87Coefficient { a: 687.0, b: 2.499, c: 77.963 },
        aavsop87::VSOP87Coefficient { a: 647.0, b: 4.473, c: 70.328 },
        aavsop87::VSOP87Coefficient { a: 624.0, b: 0.863, c: 9.561 },
        aavsop87::VSOP87Coefficient { a: 604.0, b: 0.907, c: 984.600 },
        aavsop87::VSOP87Coefficient { a: 575.0, b: 3.231, c: 447.796 },
        aavsop87::VSOP87Coefficient { a: 562.0, b: 2.718, c: 462.023 },
        aavsop87::VSOP87Coefficient { a: 530.0, b: 5.917, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 528.0, b: 5.151, c: 2.969 }
    ];

const G_R2URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 18] =
    [
        aavsop87::VSOP87Coefficient { a: 22440.0, b: 0.69953, c: 74.78160 },
        aavsop87::VSOP87Coefficient { a: 4727.0, b: 1.6990, c: 63.7359 },
        aavsop87::VSOP87Coefficient { a: 1682.0, b: 4.6483, c: 70.8494 },
        aavsop87::VSOP87Coefficient { a: 1650.0, b: 3.0966, c: 11.0457 },
        aavsop87::VSOP87Coefficient { a: 1434.0, b: 3.5212, c: 149.5632 },
        aavsop87::VSOP87Coefficient { a: 770.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 500.0, b: 6.172, c: 76.266 },
        aavsop87::VSOP87Coefficient { a: 461.0, b: 0.767, c: 3.932 },
        aavsop87::VSOP87Coefficient { a: 390.0, b: 4.496, c: 56.622 },
        aavsop87::VSOP87Coefficient { a: 390.0, b: 5.527, c: 85.827 },
        aavsop87::VSOP87Coefficient { a: 292.0, b: 0.204, c: 52.690 },
        aavsop87::VSOP87Coefficient { a: 287.0, b: 3.534, c: 73.297 },
        aavsop87::VSOP87Coefficient { a: 273.0, b: 3.847, c: 138.517 },
        aavsop87::VSOP87Coefficient { a: 220.0, b: 1.964, c: 131.404 },
        aavsop87::VSOP87Coefficient { a: 216.0, b: 0.848, c: 77.963 },
        aavsop87::VSOP87Coefficient { a: 205.0, b: 3.248, c: 78.714 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 4.898, c: 127.472 },
        aavsop87::VSOP87Coefficient { a: 129.0, b: 2.081, c: 3.181 }
    ];

const G_R3URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 10] =
    [
        aavsop87::VSOP87Coefficient { a: 1164.0, b: 4.7345, c: 74.7816 },
        aavsop87::VSOP87Coefficient { a: 212.0, b: 3.343, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 196.0, b: 2.980, c: 70.849 },
        aavsop87::VSOP87Coefficient { a: 105.0, b: 0.958, c: 11.046 },
        aavsop87::VSOP87Coefficient { a: 73.0, b: 1.00, c: 149.56 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 0.03, c: 56.62 },
        aavsop87::VSOP87Coefficient { a: 55.0, b: 2.59, c: 3.93 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 5.65, c: 77.96 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 3.82, c: 76.27 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 3.60, c: 131.40 }
    ];

const G_R4URANUS_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 53.0, b: 3.01, c: 74.78 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 1.91, c: 56.62 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ura::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate l0
    let n_l0coefficients = G_L0URANUS_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0URANUS_COEFFICIENTS[i].a * f64::cos(G_L0URANUS_COEFFICIENTS[i].b + G_L0URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1URANUS_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1URANUS_COEFFICIENTS[i].a * f64::cos(G_L1URANUS_COEFFICIENTS[i].b + G_L1URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2URANUS_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2URANUS_COEFFICIENTS[i].a * f64::cos(G_L2URANUS_COEFFICIENTS[i].b + G_L2URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3URANUS_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3URANUS_COEFFICIENTS[i].a * f64::cos(G_L3URANUS_COEFFICIENTS[i].b + G_L3URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4URANUS_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4URANUS_COEFFICIENTS[i].a * f64::cos(G_L4URANUS_COEFFICIENTS[i].b + G_L4URANUS_COEFFICIENTS[i].c * rho);
    }


    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_ura::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate b0
    let n_b0coefficients = G_B0URANUS_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0URANUS_COEFFICIENTS[i].a * f64::cos(G_B0URANUS_COEFFICIENTS[i].b + G_B0URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1URANUS_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1URANUS_COEFFICIENTS[i].a * f64::cos(G_B1URANUS_COEFFICIENTS[i].b + G_B1URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2URANUS_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2URANUS_COEFFICIENTS[i].a * f64::cos(G_B2URANUS_COEFFICIENTS[i].b + G_B2URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3URANUS_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3URANUS_COEFFICIENTS[i].a * f64::cos(G_B3URANUS_COEFFICIENTS[i].b + G_B3URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4URANUS_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4URANUS_COEFFICIENTS[i].a * f64::cos(G_B4URANUS_COEFFICIENTS[i].b + G_B4URANUS_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_ura::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;

    //Calculate r0
    let n_r0coefficients = G_R0URANUS_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0URANUS_COEFFICIENTS[i].a * f64::cos(G_R0URANUS_COEFFICIENTS[i].b + G_R0URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1URANUS_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1URANUS_COEFFICIENTS[i].a * f64::cos(G_R1URANUS_COEFFICIENTS[i].b + G_R1URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2URANUS_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2URANUS_COEFFICIENTS[i].a * f64::cos(G_R2URANUS_COEFFICIENTS[i].b + G_R2URANUS_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3URANUS_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3URANUS_COEFFICIENTS[i].a * f64::cos(G_R3URANUS_COEFFICIENTS[i].b + G_R3URANUS_COEFFICIENTS[i].c * rho);
    }

//Calculate r4
    let n_r4coefficients = G_R4URANUS_COEFFICIENTS.len();
    let mut r4 = 0.0f64;
    for i in 0..n_r4coefficients {
        r4 += G_R4URANUS_COEFFICIENTS[i].a * f64::cos(G_R4URANUS_COEFFICIENTS[i].b + G_R4URANUS_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4) / 100000000.0
}
