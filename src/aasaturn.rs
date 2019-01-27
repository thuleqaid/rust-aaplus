use super::aavsop87;
use super::aavsop87d_sat;
use super::aacoordinatetransformation;


const G_L0SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 90] =
    [
        aavsop87::VSOP87Coefficient { a: 87401354.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 11107660.0, b: 3.96205090, c: 213.29909544 },
        aavsop87::VSOP87Coefficient { a: 1414151.0, b: 4.5858152, c: 7.1135470 },
        aavsop87::VSOP87Coefficient { a: 398379.0, b: 0.521120, c: 206.185548 },
        aavsop87::VSOP87Coefficient { a: 350769.0, b: 3.303299, c: 426.598191 },
        aavsop87::VSOP87Coefficient { a: 206816.0, b: 0.246584, c: 103.092774 },
        aavsop87::VSOP87Coefficient { a: 79271.0, b: 3.84007, c: 220.41264 },
        aavsop87::VSOP87Coefficient { a: 23990.0, b: 4.66977, c: 110.20632 },
        aavsop87::VSOP87Coefficient { a: 16574.0, b: 0.43719, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 15820.0, b: 0.93809, c: 632.78374 },
        aavsop87::VSOP87Coefficient { a: 15054.0, b: 2.71670, c: 639.89729 },
        aavsop87::VSOP87Coefficient { a: 14907.0, b: 5.76903, c: 316.39187 },
        aavsop87::VSOP87Coefficient { a: 14610.0, b: 1.56519, c: 3.93215 },
        aavsop87::VSOP87Coefficient { a: 13160.0, b: 4.44891, c: 14.22709 },
        aavsop87::VSOP87Coefficient { a: 13005.0, b: 5.98119, c: 11.04570 },
        aavsop87::VSOP87Coefficient { a: 10725.0, b: 3.12940, c: 202.25340 },
        aavsop87::VSOP87Coefficient { a: 6126.0, b: 1.7633, c: 277.0350 },
        aavsop87::VSOP87Coefficient { a: 5863.0, b: 0.2366, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 5228.0, b: 4.2078, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 5020.0, b: 3.1779, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 4593.0, b: 0.6198, c: 199.0720 },
        aavsop87::VSOP87Coefficient { a: 4006.0, b: 2.2448, c: 63.7359 },
        aavsop87::VSOP87Coefficient { a: 3874.0, b: 3.2228, c: 138.5175 },
        aavsop87::VSOP87Coefficient { a: 3269.0, b: 0.7749, c: 949.1756 },
        aavsop87::VSOP87Coefficient { a: 2954.0, b: 0.9828, c: 95.9792 },
        aavsop87::VSOP87Coefficient { a: 2461.0, b: 2.0316, c: 735.8765 },
        aavsop87::VSOP87Coefficient { a: 1758.0, b: 3.2658, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 1640.0, b: 5.5050, c: 846.0828 },
        aavsop87::VSOP87Coefficient { a: 1581.0, b: 4.3727, c: 309.2783 },
        aavsop87::VSOP87Coefficient { a: 1391.0, b: 4.0233, c: 323.5054 },
        aavsop87::VSOP87Coefficient { a: 1124.0, b: 2.8373, c: 415.5525 },
        aavsop87::VSOP87Coefficient { a: 1087.0, b: 4.1834, c: 2.4477 },
        aavsop87::VSOP87Coefficient { a: 1017.0, b: 3.7170, c: 227.5262 },
        aavsop87::VSOP87Coefficient { a: 957.0, b: 0.507, c: 1265.567 },
        aavsop87::VSOP87Coefficient { a: 853.0, b: 3.421, c: 175.166 },
        aavsop87::VSOP87Coefficient { a: 849.0, b: 3.191, c: 209.367 },
        aavsop87::VSOP87Coefficient { a: 789.0, b: 5.007, c: 0.963 },
        aavsop87::VSOP87Coefficient { a: 749.0, b: 2.144, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 744.0, b: 5.253, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 687.0, b: 1.747, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 654.0, b: 1.599, c: 0.048 },
        aavsop87::VSOP87Coefficient { a: 634.0, b: 2.299, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 625.0, b: 0.970, c: 210.118 },
        aavsop87::VSOP87Coefficient { a: 580.0, b: 3.093, c: 74.782 },
        aavsop87::VSOP87Coefficient { a: 546.0, b: 2.127, c: 350.332 },
        aavsop87::VSOP87Coefficient { a: 543.0, b: 1.518, c: 9.561 },
        aavsop87::VSOP87Coefficient { a: 530.0, b: 4.449, c: 117.320 },
        aavsop87::VSOP87Coefficient { a: 478.0, b: 2.965, c: 137.033 },
        aavsop87::VSOP87Coefficient { a: 474.0, b: 5.475, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 452.0, b: 1.044, c: 490.334 },
        aavsop87::VSOP87Coefficient { a: 449.0, b: 1.290, c: 127.472 },
        aavsop87::VSOP87Coefficient { a: 372.0, b: 2.278, c: 217.231 },
        aavsop87::VSOP87Coefficient { a: 355.0, b: 3.013, c: 838.969 },
        aavsop87::VSOP87Coefficient { a: 347.0, b: 1.539, c: 340.771 },
        aavsop87::VSOP87Coefficient { a: 343.0, b: 0.246, c: 0.521 },
        aavsop87::VSOP87Coefficient { a: 330.0, b: 0.247, c: 1581.959 },
        aavsop87::VSOP87Coefficient { a: 322.0, b: 0.961, c: 203.738 },
        aavsop87::VSOP87Coefficient { a: 322.0, b: 2.572, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 309.0, b: 3.495, c: 216.480 },
        aavsop87::VSOP87Coefficient { a: 287.0, b: 2.370, c: 351.817 },
        aavsop87::VSOP87Coefficient { a: 278.0, b: 0.400, c: 211.815 },
        aavsop87::VSOP87Coefficient { a: 249.0, b: 1.470, c: 1368.660 },
        aavsop87::VSOP87Coefficient { a: 227.0, b: 4.910, c: 12.530 },
        aavsop87::VSOP87Coefficient { a: 220.0, b: 4.204, c: 200.769 },
        aavsop87::VSOP87Coefficient { a: 209.0, b: 1.345, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 0.483, c: 1162.475 },
        aavsop87::VSOP87Coefficient { a: 208.0, b: 1.283, c: 39.357 },
        aavsop87::VSOP87Coefficient { a: 204.0, b: 6.011, c: 265.989 },
        aavsop87::VSOP87Coefficient { a: 185.0, b: 3.503, c: 149.563 },
        aavsop87::VSOP87Coefficient { a: 184.0, b: 0.973, c: 4.193 },
        aavsop87::VSOP87Coefficient { a: 182.0, b: 5.491, c: 2.921 },
        aavsop87::VSOP87Coefficient { a: 174.0, b: 1.863, c: 0.751 },
        aavsop87::VSOP87Coefficient { a: 165.0, b: 0.440, c: 5.417 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 5.736, c: 52.690 },
        aavsop87::VSOP87Coefficient { a: 148.0, b: 1.535, c: 5.629 },
        aavsop87::VSOP87Coefficient { a: 146.0, b: 6.231, c: 195.140 },
        aavsop87::VSOP87Coefficient { a: 140.0, b: 4.295, c: 21.341 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 4.068, c: 10.295 },
        aavsop87::VSOP87Coefficient { a: 125.0, b: 6.277, c: 1898.351 },
        aavsop87::VSOP87Coefficient { a: 122.0, b: 1.976, c: 4.666 },
        aavsop87::VSOP87Coefficient { a: 118.0, b: 5.341, c: 554.070 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 2.679, c: 1155.361 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 5.594, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 112.0, b: 1.105, c: 191.208 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 0.166, c: 1.484 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 3.438, c: 536.805 },
        aavsop87::VSOP87Coefficient { a: 107.0, b: 4.012, c: 956.289 },
        aavsop87::VSOP87Coefficient { a: 104.0, b: 2.192, c: 88.866 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 1.197, c: 1685.052 },
        aavsop87::VSOP87Coefficient { a: 101.0, b: 4.965, c: 269.921 }
    ];

const G_L1SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 79] =
    [
        aavsop87::VSOP87Coefficient { a: 21354295596.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 1296855.0, b: 1.8282054, c: 213.2990954 },
        aavsop87::VSOP87Coefficient { a: 564348.0, b: 2.885001, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 107679.0, b: 2.277699, c: 206.185548 },
        aavsop87::VSOP87Coefficient { a: 98323.0, b: 1.08070, c: 426.59819 },
        aavsop87::VSOP87Coefficient { a: 40255.0, b: 2.04128, c: 220.41264 },
        aavsop87::VSOP87Coefficient { a: 19942.0, b: 1.27955, c: 103.09277 },
        aavsop87::VSOP87Coefficient { a: 10512.0, b: 2.74880, c: 14.22709 },
        aavsop87::VSOP87Coefficient { a: 6939.0, b: 0.4049, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 4803.0, b: 2.4419, c: 419.4846 },
        aavsop87::VSOP87Coefficient { a: 4056.0, b: 2.9217, c: 110.2063 },
        aavsop87::VSOP87Coefficient { a: 3769.0, b: 3.6497, c: 3.9322 },
        aavsop87::VSOP87Coefficient { a: 3385.0, b: 2.4169, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 3302.0, b: 1.2626, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 3071.0, b: 2.3274, c: 199.0720 },
        aavsop87::VSOP87Coefficient { a: 1953.0, b: 3.5639, c: 11.0457 },
        aavsop87::VSOP87Coefficient { a: 1249.0, b: 2.6280, c: 95.9792 },
        aavsop87::VSOP87Coefficient { a: 922.0, b: 1.961, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 706.0, b: 4.417, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 650.0, b: 6.174, c: 202.253 },
        aavsop87::VSOP87Coefficient { a: 628.0, b: 6.111, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 487.0, b: 6.040, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 479.0, b: 4.988, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 468.0, b: 4.617, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 417.0, b: 2.117, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 408.0, b: 1.299, c: 209.367 },
        aavsop87::VSOP87Coefficient { a: 352.0, b: 2.317, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 344.0, b: 3.959, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 340.0, b: 3.634, c: 316.392 },
        aavsop87::VSOP87Coefficient { a: 336.0, b: 3.772, c: 735.877 },
        aavsop87::VSOP87Coefficient { a: 332.0, b: 2.861, c: 210.118 },
        aavsop87::VSOP87Coefficient { a: 289.0, b: 2.733, c: 117.320 },
        aavsop87::VSOP87Coefficient { a: 281.0, b: 5.744, c: 2.448 },
        aavsop87::VSOP87Coefficient { a: 266.0, b: 0.543, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 230.0, b: 1.644, c: 216.480 },
        aavsop87::VSOP87Coefficient { a: 192.0, b: 2.965, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 173.0, b: 4.077, c: 846.083 },
        aavsop87::VSOP87Coefficient { a: 167.0, b: 2.597, c: 21.341 },
        aavsop87::VSOP87Coefficient { a: 136.0, b: 2.286, c: 10.295 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 3.441, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 4.095, c: 217.231 },
        aavsop87::VSOP87Coefficient { a: 109.0, b: 6.161, c: 415.552 },
        aavsop87::VSOP87Coefficient { a: 98.0, b: 4.73, c: 838.97 },
        aavsop87::VSOP87Coefficient { a: 94.0, b: 3.48, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 92.0, b: 3.95, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 87.0, b: 1.22, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 3.11, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 78.0, b: 6.24, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 0.29, c: 4.67 },
        aavsop87::VSOP87Coefficient { a: 66.0, b: 5.65, c: 9.56 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 4.29, c: 127.47 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 1.83, c: 195.14 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 2.48, c: 191.96 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 5.02, c: 137.03 },
        aavsop87::VSOP87Coefficient { a: 55.0, b: 0.28, c: 74.78 },
        aavsop87::VSOP87Coefficient { a: 54.0, b: 5.13, c: 490.33 },
        aavsop87::VSOP87Coefficient { a: 51.0, b: 1.46, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 1.18, c: 149.56 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 5.15, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 2.23, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 2.71, c: 5.42 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 0.41, c: 269.92 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 3.89, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 0.65, c: 422.67 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 2.53, c: 12.53 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 3.78, c: 2.92 },
        aavsop87::VSOP87Coefficient { a: 35.0, b: 6.08, c: 5.63 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 3.21, c: 1368.66 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 4.64, c: 277.03 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 5.43, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 0.30, c: 351.82 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.39, c: 1155.36 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 2.43, c: 52.69 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 2.84, c: 203.00 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 6.19, c: 284.15 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 3.39, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 2.03, c: 330.62 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 2.74, c: 265.99 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 4.51, c: 340.77 }
    ];

const G_L2SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 63] =
    [
        aavsop87::VSOP87Coefficient { a: 116441.0, b: 1.179879, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 91921.0, b: 0.07425, c: 213.29910 },
        aavsop87::VSOP87Coefficient { a: 90592.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 15277.0, b: 4.06492, c: 206.18555 },
        aavsop87::VSOP87Coefficient { a: 10631.0, b: 0.25778, c: 220.41264 },
        aavsop87::VSOP87Coefficient { a: 10605.0, b: 5.40964, c: 426.59819 },
        aavsop87::VSOP87Coefficient { a: 4265.0, b: 1.0460, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 1216.0, b: 2.9186, c: 103.0928 },
        aavsop87::VSOP87Coefficient { a: 1165.0, b: 4.6094, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 1082.0, b: 5.6913, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 1045.0, b: 4.0421, c: 199.0720 },
        aavsop87::VSOP87Coefficient { a: 1020.0, b: 0.6337, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 634.0, b: 4.388, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 549.0, b: 5.573, c: 3.932 },
        aavsop87::VSOP87Coefficient { a: 457.0, b: 1.268, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 425.0, b: 0.209, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 274.0, b: 4.288, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 162.0, b: 1.381, c: 11.046 },
        aavsop87::VSOP87Coefficient { a: 129.0, b: 1.566, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 3.881, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 105.0, b: 4.900, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 101.0, b: 0.893, c: 21.341 },
        aavsop87::VSOP87Coefficient { a: 96.0, b: 2.91, c: 316.39 },
        aavsop87::VSOP87Coefficient { a: 95.0, b: 5.63, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 85.0, b: 5.73, c: 209.37 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 6.05, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 82.0, b: 1.02, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 75.0, b: 4.76, c: 210.12 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 0.46, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 66.0, b: 0.48, c: 10.29 },
        aavsop87::VSOP87Coefficient { a: 64.0, b: 0.35, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 61.0, b: 4.88, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 53.0, b: 2.75, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 5.69, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 1.67, c: 202.25 },
        aavsop87::VSOP87Coefficient { a: 42.0, b: 5.71, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 0.07, c: 63.74 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 1.67, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 4.16, c: 191.96 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 0.83, c: 224.34 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 5.66, c: 735.88 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 5.94, c: 217.23 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 4.90, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 1.63, c: 742.99 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 0.58, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 0.21, c: 838.97 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 3.76, c: 195.14 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 4.72, c: 203.00 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 0.13, c: 234.64 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.12, c: 846.08 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 5.92, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 5.60, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 3.20, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 4.99, c: 422.67 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 0.26, c: 330.62 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 4.15, c: 860.31 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.46, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.14, c: 269.92 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.25, c: 429.78 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 4.03, c: 9.56 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 5.40, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.46, c: 284.15 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.93, c: 405.26 }
    ];

const G_L3SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 48] =
    [
        aavsop87::VSOP87Coefficient { a: 16039.0, b: 5.73945, c: 7.11355 },
        aavsop87::VSOP87Coefficient { a: 4250.0, b: 4.5854, c: 213.2991 },
        aavsop87::VSOP87Coefficient { a: 1907.0, b: 4.7608, c: 220.4126 },
        aavsop87::VSOP87Coefficient { a: 1466.0, b: 5.9133, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 1162.0, b: 5.6197, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 1067.0, b: 3.6082, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 239.0, b: 3.861, c: 433.712 },
        aavsop87::VSOP87Coefficient { a: 237.0, b: 5.768, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 166.0, b: 5.116, c: 3.181 },
        aavsop87::VSOP87Coefficient { a: 151.0, b: 2.736, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 4.743, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 63.0, b: 0.23, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 4.74, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 5.47, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 5.96, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 39.0, b: 5.83, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 3.01, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 0.99, c: 3.93 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 1.92, c: 853.20 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 4.97, c: 10.29 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 1.03, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 4.20, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 3.32, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 3.90, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 5.62, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 1.18, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 5.58, c: 11.05 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 5.93, c: 191.96 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 3.95, c: 209.37 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.39, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 4.88, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 0.38, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.25, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.06, c: 210.12 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 4.64, c: 234.64 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 2.31, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.20, c: 860.31 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.59, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.93, c: 224.34 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.42, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.77, c: 330.62 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.35, c: 429.78 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.20, c: 202.25 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 1.19, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 1.35, c: 405.26 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.16, c: 223.59 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.07, c: 654.12 }
    ];

const G_L4SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 27] =
    [
        aavsop87::VSOP87Coefficient { a: 1662.0, b: 3.9983, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 257.0, b: 2.984, c: 220.413 },
        aavsop87::VSOP87Coefficient { a: 236.0, b: 3.902, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 2.741, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 1.515, c: 206.186 },
        aavsop87::VSOP87Coefficient { a: 68.0, b: 1.72, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 2.05, c: 433.71 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 1.24, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 3.01, c: 227.53 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 0.83, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.71, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.42, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.16, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 1.45, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 2.12, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.09, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.77, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.01, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.00, c: 853.20 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.39, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.78, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.83, c: 234.64 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.08, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.24, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.19, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.55, c: 191.96 }
    ];

const G_L5SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 12] =
    [
        aavsop87::VSOP87Coefficient { a: 124.0, b: 2.259, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 2.16, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 1.20, c: 220.41 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.22, c: 227.53 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 0.24, c: 433.71 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 6.23, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.97, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.29, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 6.25, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.28, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.24, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_B0SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 34] =
    [
        aavsop87::VSOP87Coefficient { a: 4330678.0, b: 3.6028443, c: 213.2990954 },
        aavsop87::VSOP87Coefficient { a: 240348.0, b: 2.852385, c: 426.598191 },
        aavsop87::VSOP87Coefficient { a: 84746.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 34116.0, b: 0.57297, c: 206.18555 },
        aavsop87::VSOP87Coefficient { a: 30863.0, b: 3.48442, c: 220.41264 },
        aavsop87::VSOP87Coefficient { a: 14734.0, b: 2.11847, c: 639.89729 },
        aavsop87::VSOP87Coefficient { a: 9917.0, b: 5.7900, c: 419.4846 },
        aavsop87::VSOP87Coefficient { a: 6994.0, b: 4.7360, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 4808.0, b: 5.4331, c: 316.3919 },
        aavsop87::VSOP87Coefficient { a: 4788.0, b: 4.9651, c: 110.2063 },
        aavsop87::VSOP87Coefficient { a: 3432.0, b: 2.7326, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 1506.0, b: 6.0130, c: 103.0928 },
        aavsop87::VSOP87Coefficient { a: 1060.0, b: 5.6310, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 969.0, b: 5.204, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 942.0, b: 1.396, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 708.0, b: 3.803, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 552.0, b: 5.131, c: 202.253 },
        aavsop87::VSOP87Coefficient { a: 400.0, b: 3.359, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 319.0, b: 3.626, c: 209.367 },
        aavsop87::VSOP87Coefficient { a: 316.0, b: 1.997, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 314.0, b: 0.465, c: 217.231 },
        aavsop87::VSOP87Coefficient { a: 284.0, b: 4.886, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 236.0, b: 2.139, c: 11.046 },
        aavsop87::VSOP87Coefficient { a: 215.0, b: 5.950, c: 846.083 },
        aavsop87::VSOP87Coefficient { a: 209.0, b: 2.120, c: 415.552 },
        aavsop87::VSOP87Coefficient { a: 207.0, b: 0.730, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 179.0, b: 2.954, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 141.0, b: 0.644, c: 490.334 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 4.595, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 1.998, c: 735.877 },
        aavsop87::VSOP87Coefficient { a: 135.0, b: 5.245, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 122.0, b: 3.115, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 116.0, b: 3.109, c: 216.480 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 0.963, c: 210.118 }
    ];

const G_B1SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 32] =
    [
        aavsop87::VSOP87Coefficient { a: 397555.0, b: 5.332900, c: 213.299095 },
        aavsop87::VSOP87Coefficient { a: 49479.0, b: 3.14159, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 18572.0, b: 6.09919, c: 426.59819 },
        aavsop87::VSOP87Coefficient { a: 14801.0, b: 2.30586, c: 206.18555 },
        aavsop87::VSOP87Coefficient { a: 9644.0, b: 1.6967, c: 220.4126 },
        aavsop87::VSOP87Coefficient { a: 3757.0, b: 1.2543, c: 419.4846 },
        aavsop87::VSOP87Coefficient { a: 2717.0, b: 5.9117, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 1455.0, b: 0.8516, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 1291.0, b: 2.9177, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 853.0, b: 0.436, c: 316.392 },
        aavsop87::VSOP87Coefficient { a: 298.0, b: 0.919, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 292.0, b: 5.316, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 284.0, b: 1.619, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 275.0, b: 3.889, c: 103.093 },
        aavsop87::VSOP87Coefficient { a: 172.0, b: 0.052, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 166.0, b: 2.444, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 158.0, b: 5.209, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 128.0, b: 1.207, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 110.0, b: 2.457, c: 217.231 },
        aavsop87::VSOP87Coefficient { a: 82.0, b: 2.76, c: 210.12 },
        aavsop87::VSOP87Coefficient { a: 81.0, b: 2.86, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 69.0, b: 1.66, c: 202.25 },
        aavsop87::VSOP87Coefficient { a: 65.0, b: 1.26, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 61.0, b: 1.25, c: 209.37 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 1.82, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 0.82, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 1.82, c: 224.34 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 2.84, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 1.31, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 1.19, c: 846.08 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 4.65, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 4.44, c: 11.05 }
    ];

const G_B2SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 29] =
    [
        aavsop87::VSOP87Coefficient { a: 20630.0, b: 0.50482, c: 213.29910 },
        aavsop87::VSOP87Coefficient { a: 3720.0, b: 3.9983, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 1627.0, b: 6.1819, c: 220.4126 },
        aavsop87::VSOP87Coefficient { a: 1346.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 706.0, b: 3.039, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 365.0, b: 5.099, c: 426.598 },
        aavsop87::VSOP87Coefficient { a: 330.0, b: 5.279, c: 433.712 },
        aavsop87::VSOP87Coefficient { a: 219.0, b: 3.828, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 1.043, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 104.0, b: 6.157, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 93.0, b: 1.98, c: 316.39 },
        aavsop87::VSOP87Coefficient { a: 71.0, b: 4.15, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 2.88, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 4.43, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 3.16, c: 853.20 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 4.53, c: 210.12 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 1.12, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 4.35, c: 217.23 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 5.31, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 0.85, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 5.68, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 4.26, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 3.00, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.53, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 3.32, c: 202.25 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 5.56, c: 209.37 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 0.29, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.16, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 3.61, c: 860.31 }
    ];

const G_B3SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 21] =
    [
        aavsop87::VSOP87Coefficient { a: 666.0, b: 1.990, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 632.0, b: 5.698, c: 206.186 },
        aavsop87::VSOP87Coefficient { a: 398.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 188.0, b: 4.338, c: 220.413 },
        aavsop87::VSOP87Coefficient { a: 92.0, b: 4.84, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 3.42, c: 433.71 },
        aavsop87::VSOP87Coefficient { a: 42.0, b: 2.38, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 4.40, c: 227.53 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 5.85, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 1.99, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 5.37, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 2.55, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 3.46, c: 316.39 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 4.80, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 0.02, c: 210.12 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 3.52, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 5.64, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 1.22, c: 853.20 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 4.71, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.63, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.72, c: 216.48 }
    ];

const G_B4SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 12] =
    [
        aavsop87::VSOP87Coefficient { a: 80.0, b: 1.12, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 3.12, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.48, c: 220.41 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.38, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.56, c: 433.71 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 2.63, c: 227.53 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 1.28, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.43, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.67, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.72, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 6.18, c: 639.90 }
    ];

const G_B5SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 2] =
    [
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.82, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.51, c: 220.41 }
    ];

const G_R0SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 44] =
    [
        aavsop87::VSOP87Coefficient { a: 955758136.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 52921382.0, b: 2.39226220, c: 213.29909544 },
        aavsop87::VSOP87Coefficient { a: 1873680.0, b: 5.2354961, c: 206.1855484 },
        aavsop87::VSOP87Coefficient { a: 1464664.0, b: 1.6476305, c: 426.5981909 },
        aavsop87::VSOP87Coefficient { a: 821891.0, b: 5.935200, c: 316.391870 },
        aavsop87::VSOP87Coefficient { a: 547507.0, b: 5.015326, c: 103.092774 },
        aavsop87::VSOP87Coefficient { a: 371684.0, b: 2.271148, c: 220.412642 },
        aavsop87::VSOP87Coefficient { a: 361778.0, b: 3.139043, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 140618.0, b: 5.704067, c: 632.783739 },
        aavsop87::VSOP87Coefficient { a: 108975.0, b: 3.293136, c: 110.206321 },
        aavsop87::VSOP87Coefficient { a: 69007.0, b: 5.94100, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 61053.0, b: 0.94038, c: 639.89729 },
        aavsop87::VSOP87Coefficient { a: 48913.0, b: 1.55733, c: 202.25340 },
        aavsop87::VSOP87Coefficient { a: 34144.0, b: 0.19519, c: 277.03499 },
        aavsop87::VSOP87Coefficient { a: 32402.0, b: 5.47085, c: 949.17561 },
        aavsop87::VSOP87Coefficient { a: 20937.0, b: 0.46349, c: 735.87651 },
        aavsop87::VSOP87Coefficient { a: 20839.0, b: 1.52103, c: 433.71174 },
        aavsop87::VSOP87Coefficient { a: 20747.0, b: 5.33256, c: 199.07200 },
        aavsop87::VSOP87Coefficient { a: 15298.0, b: 3.05944, c: 529.69097 },
        aavsop87::VSOP87Coefficient { a: 14296.0, b: 2.60434, c: 323.50542 },
        aavsop87::VSOP87Coefficient { a: 12884.0, b: 1.64892, c: 138.51750 },
        aavsop87::VSOP87Coefficient { a: 11993.0, b: 5.98051, c: 846.08283 },
        aavsop87::VSOP87Coefficient { a: 11380.0, b: 1.73106, c: 522.57742 },
        aavsop87::VSOP87Coefficient { a: 9796.0, b: 5.2048, c: 1265.5675 },
        aavsop87::VSOP87Coefficient { a: 7753.0, b: 5.8519, c: 95.9792 },
        aavsop87::VSOP87Coefficient { a: 6771.0, b: 3.0043, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 6466.0, b: 0.1773, c: 1052.2684 },
        aavsop87::VSOP87Coefficient { a: 5850.0, b: 1.4552, c: 415.5525 },
        aavsop87::VSOP87Coefficient { a: 5307.0, b: 0.5974, c: 63.7359 },
        aavsop87::VSOP87Coefficient { a: 4696.0, b: 2.1492, c: 227.5262 },
        aavsop87::VSOP87Coefficient { a: 4044.0, b: 1.6401, c: 209.3669 },
        aavsop87::VSOP87Coefficient { a: 3688.0, b: 0.7802, c: 412.3711 },
        aavsop87::VSOP87Coefficient { a: 3461.0, b: 1.8509, c: 175.1661 },
        aavsop87::VSOP87Coefficient { a: 3420.0, b: 4.9455, c: 1581.9593 },
        aavsop87::VSOP87Coefficient { a: 3401.0, b: 0.5539, c: 350.3321 },
        aavsop87::VSOP87Coefficient { a: 3376.0, b: 3.6953, c: 224.3448 },
        aavsop87::VSOP87Coefficient { a: 2976.0, b: 5.6847, c: 210.1177 },
        aavsop87::VSOP87Coefficient { a: 2885.0, b: 1.3876, c: 838.9693 },
        aavsop87::VSOP87Coefficient { a: 2881.0, b: 0.1796, c: 853.1964 },
        aavsop87::VSOP87Coefficient { a: 2508.0, b: 3.5385, c: 742.9901 },
        aavsop87::VSOP87Coefficient { a: 2448.0, b: 6.1841, c: 1368.6603 },
        aavsop87::VSOP87Coefficient { a: 2406.0, b: 2.9656, c: 117.3199 },
        aavsop87::VSOP87Coefficient { a: 2174.0, b: 0.0151, c: 340.7709 },
        aavsop87::VSOP87Coefficient { a: 2024.0, b: 5.0541, c: 11.0457 }
    ];

const G_R1SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 38] =
    [
        aavsop87::VSOP87Coefficient { a: 6182981.0, b: 0.2584352, c: 213.2990954 },
        aavsop87::VSOP87Coefficient { a: 506578.0, b: 0.711147, c: 206.185548 },
        aavsop87::VSOP87Coefficient { a: 341394.0, b: 5.796358, c: 426.598191 },
        aavsop87::VSOP87Coefficient { a: 188491.0, b: 0.472157, c: 220.412642 },
        aavsop87::VSOP87Coefficient { a: 186262.0, b: 3.141593, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 143891.0, b: 1.407449, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 49621.0, b: 6.01744, c: 103.09277 },
        aavsop87::VSOP87Coefficient { a: 20928.0, b: 5.09246, c: 639.89729 },
        aavsop87::VSOP87Coefficient { a: 19953.0, b: 1.17560, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 18840.0, b: 1.60820, c: 110.20632 },
        aavsop87::VSOP87Coefficient { a: 13877.0, b: 0.75886, c: 199.07200 },
        aavsop87::VSOP87Coefficient { a: 12893.0, b: 5.94330, c: 433.71174 },
        aavsop87::VSOP87Coefficient { a: 5397.0, b: 1.2885, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 4869.0, b: 0.8679, c: 323.5054 },
        aavsop87::VSOP87Coefficient { a: 4247.0, b: 0.3930, c: 227.5262 },
        aavsop87::VSOP87Coefficient { a: 3252.0, b: 1.2585, c: 95.9792 },
        aavsop87::VSOP87Coefficient { a: 3081.0, b: 3.4366, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 2909.0, b: 4.6068, c: 202.2534 },
        aavsop87::VSOP87Coefficient { a: 2856.0, b: 2.1673, c: 735.8765 },
        aavsop87::VSOP87Coefficient { a: 1988.0, b: 2.4505, c: 412.3711 },
        aavsop87::VSOP87Coefficient { a: 1941.0, b: 6.0239, c: 209.3669 },
        aavsop87::VSOP87Coefficient { a: 1581.0, b: 1.2919, c: 210.1177 },
        aavsop87::VSOP87Coefficient { a: 1340.0, b: 4.3080, c: 853.1964 },
        aavsop87::VSOP87Coefficient { a: 1316.0, b: 1.2530, c: 117.3199 },
        aavsop87::VSOP87Coefficient { a: 1203.0, b: 1.8665, c: 316.3919 },
        aavsop87::VSOP87Coefficient { a: 1091.0, b: 0.0753, c: 216.4805 },
        aavsop87::VSOP87Coefficient { a: 966.0, b: 0.480, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 954.0, b: 5.152, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 898.0, b: 0.983, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 882.0, b: 1.885, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 874.0, b: 1.402, c: 224.345 },
        aavsop87::VSOP87Coefficient { a: 785.0, b: 3.064, c: 838.969 },
        aavsop87::VSOP87Coefficient { a: 740.0, b: 1.382, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 658.0, b: 4.144, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 650.0, b: 1.725, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 613.0, b: 3.033, c: 63.736 },
        aavsop87::VSOP87Coefficient { a: 599.0, b: 2.549, c: 217.231 },
        aavsop87::VSOP87Coefficient { a: 503.0, b: 2.130, c: 3.932 }
    ];

const G_R2SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 32] =
    [
        aavsop87::VSOP87Coefficient { a: 436902.0, b: 4.786717, c: 213.299095 },
        aavsop87::VSOP87Coefficient { a: 71923.0, b: 2.50070, c: 206.18555 },
        aavsop87::VSOP87Coefficient { a: 49767.0, b: 4.97168, c: 220.41264 },
        aavsop87::VSOP87Coefficient { a: 43221.0, b: 3.86940, c: 426.59819 },
        aavsop87::VSOP87Coefficient { a: 29646.0, b: 5.96310, c: 7.11355 },
        aavsop87::VSOP87Coefficient { a: 4721.0, b: 2.4753, c: 199.0720 },
        aavsop87::VSOP87Coefficient { a: 4142.0, b: 4.1067, c: 433.7117 },
        aavsop87::VSOP87Coefficient { a: 3789.0, b: 3.0977, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 2964.0, b: 1.3721, c: 103.0928 },
        aavsop87::VSOP87Coefficient { a: 2556.0, b: 2.8507, c: 419.4846 },
        aavsop87::VSOP87Coefficient { a: 2327.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 2208.0, b: 6.2759, c: 110.2063 },
        aavsop87::VSOP87Coefficient { a: 2188.0, b: 5.8555, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 1957.0, b: 4.9245, c: 227.5262 },
        aavsop87::VSOP87Coefficient { a: 924.0, b: 5.464, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 706.0, b: 2.971, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 546.0, b: 4.129, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 431.0, b: 5.178, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 405.0, b: 4.173, c: 209.367 },
        aavsop87::VSOP87Coefficient { a: 391.0, b: 4.481, c: 216.480 },
        aavsop87::VSOP87Coefficient { a: 374.0, b: 5.834, c: 117.320 },
        aavsop87::VSOP87Coefficient { a: 361.0, b: 3.277, c: 647.011 },
        aavsop87::VSOP87Coefficient { a: 356.0, b: 3.192, c: 210.118 },
        aavsop87::VSOP87Coefficient { a: 326.0, b: 2.269, c: 853.196 },
        aavsop87::VSOP87Coefficient { a: 207.0, b: 4.022, c: 735.877 },
        aavsop87::VSOP87Coefficient { a: 204.0, b: 0.088, c: 202.253 },
        aavsop87::VSOP87Coefficient { a: 180.0, b: 3.597, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 178.0, b: 4.097, c: 440.825 },
        aavsop87::VSOP87Coefficient { a: 154.0, b: 3.135, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 148.0, b: 0.136, c: 302.165 },
        aavsop87::VSOP87Coefficient { a: 133.0, b: 2.594, c: 191.958 },
        aavsop87::VSOP87Coefficient { a: 132.0, b: 5.933, c: 309.278 }
    ];

const G_R3SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 28] =
    [
        aavsop87::VSOP87Coefficient { a: 20315.0, b: 3.02187, c: 213.29910 },
        aavsop87::VSOP87Coefficient { a: 8924.0, b: 3.1914, c: 220.4126 },
        aavsop87::VSOP87Coefficient { a: 6909.0, b: 4.3517, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 4087.0, b: 4.2241, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 3879.0, b: 2.0106, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 1071.0, b: 4.2036, c: 199.0720 },
        aavsop87::VSOP87Coefficient { a: 907.0, b: 2.283, c: 433.712 },
        aavsop87::VSOP87Coefficient { a: 606.0, b: 3.175, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 597.0, b: 4.135, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 483.0, b: 1.173, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 393.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 229.0, b: 4.698, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 188.0, b: 4.590, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 150.0, b: 3.202, c: 103.093 },
        aavsop87::VSOP87Coefficient { a: 121.0, b: 3.768, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 4.710, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 101.0, b: 5.819, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 93.0, b: 1.44, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 84.0, b: 2.63, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 73.0, b: 4.15, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 2.31, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 55.0, b: 0.31, c: 853.20 },
        aavsop87::VSOP87Coefficient { a: 50.0, b: 2.39, c: 209.37 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 4.37, c: 191.96 },
        aavsop87::VSOP87Coefficient { a: 41.0, b: 0.69, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 1.84, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 5.94, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.01, c: 21.34 }
    ];

const G_R4SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 23] =
    [
        aavsop87::VSOP87Coefficient { a: 1202.0, b: 1.4150, c: 220.4126 },
        aavsop87::VSOP87Coefficient { a: 708.0, b: 1.162, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 516.0, b: 6.240, c: 206.186 },
        aavsop87::VSOP87Coefficient { a: 427.0, b: 2.469, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 268.0, b: 0.187, c: 426.598 },
        aavsop87::VSOP87Coefficient { a: 170.0, b: 5.959, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 150.0, b: 0.480, c: 433.712 },
        aavsop87::VSOP87Coefficient { a: 145.0, b: 1.442, c: 227.526 },
        aavsop87::VSOP87Coefficient { a: 121.0, b: 2.405, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 5.57, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 5.86, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 0.53, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 2.90, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 0.30, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 1.30, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 2.09, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 0.22, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 2.46, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 1.56, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.28, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.68, c: 216.48 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 1.27, c: 234.64 }
    ];

const G_R5SATURN_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 18] =
    [
        aavsop87::VSOP87Coefficient { a: 129.0, b: 5.913, c: 220.413 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 0.69, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.91, c: 227.53 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 4.95, c: 433.71 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 0.67, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 2.67, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 1.46, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 4.59, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 4.63, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 3.61, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 4.90, c: 440.83 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.07, c: 647.01 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.66, c: 191.96 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.49, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.18, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.70, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.32, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 0.56, c: 117.32 }
    ];

pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_sat::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate l0
    let n_l0coefficients = G_L0SATURN_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0SATURN_COEFFICIENTS[i].a * f64::cos(G_L0SATURN_COEFFICIENTS[i].b + G_L0SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1SATURN_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1SATURN_COEFFICIENTS[i].a * f64::cos(G_L1SATURN_COEFFICIENTS[i].b + G_L1SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2SATURN_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2SATURN_COEFFICIENTS[i].a * f64::cos(G_L2SATURN_COEFFICIENTS[i].b + G_L2SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3SATURN_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3SATURN_COEFFICIENTS[i].a * f64::cos(G_L3SATURN_COEFFICIENTS[i].b + G_L3SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4SATURN_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4SATURN_COEFFICIENTS[i].a * f64::cos(G_L4SATURN_COEFFICIENTS[i].b + G_L4SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate l5
    let n_l5coefficients = G_L5SATURN_COEFFICIENTS.len();
    let mut l5 = 0.0f64;
    for i in 0..n_l5coefficients {
        l5 += G_L5SATURN_COEFFICIENTS[i].a * f64::cos(G_L5SATURN_COEFFICIENTS[i].b + G_L5SATURN_COEFFICIENTS[i].c * rho);
    }

    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_sat::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate b0
    let n_b0coefficients = G_B0SATURN_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0SATURN_COEFFICIENTS[i].a * f64::cos(G_B0SATURN_COEFFICIENTS[i].b + G_B0SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1SATURN_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1SATURN_COEFFICIENTS[i].a * f64::cos(G_B1SATURN_COEFFICIENTS[i].b + G_B1SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2SATURN_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2SATURN_COEFFICIENTS[i].a * f64::cos(G_B2SATURN_COEFFICIENTS[i].b + G_B2SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3SATURN_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3SATURN_COEFFICIENTS[i].a * f64::cos(G_B3SATURN_COEFFICIENTS[i].b + G_B3SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4SATURN_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4SATURN_COEFFICIENTS[i].a * f64::cos(G_B4SATURN_COEFFICIENTS[i].b + G_B4SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate b5
    let n_b5coefficients = G_B5SATURN_COEFFICIENTS.len();
    let mut b5 = 0.0f64;
    for i in 0..n_b5coefficients {
        b5 += G_B5SATURN_COEFFICIENTS[i].a * f64::cos(G_B5SATURN_COEFFICIENTS[i].b + G_B5SATURN_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4 + b5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_sat::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate r0
    let n_r0coefficients = G_R0SATURN_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0SATURN_COEFFICIENTS[i].a * f64::cos(G_R0SATURN_COEFFICIENTS[i].b + G_R0SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1SATURN_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1SATURN_COEFFICIENTS[i].a * f64::cos(G_R1SATURN_COEFFICIENTS[i].b + G_R1SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2SATURN_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2SATURN_COEFFICIENTS[i].a * f64::cos(G_R2SATURN_COEFFICIENTS[i].b + G_R2SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3SATURN_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3SATURN_COEFFICIENTS[i].a * f64::cos(G_R3SATURN_COEFFICIENTS[i].b + G_R3SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate r4
    let n_r4coefficients = G_R4SATURN_COEFFICIENTS.len();
    let mut r4 = 0.0f64;
    for i in 0..n_r4coefficients {
        r4 += G_R4SATURN_COEFFICIENTS[i].a * f64::cos(G_R4SATURN_COEFFICIENTS[i].b + G_R4SATURN_COEFFICIENTS[i].c * rho);
    }

    //Calculate r5
    let n_r5coefficients = G_R5SATURN_COEFFICIENTS.len();
    let mut r5 = 0.0f64;
    for i in 0..n_r5coefficients {
        r5 += G_R5SATURN_COEFFICIENTS[i].a * f64::cos(G_R5SATURN_COEFFICIENTS[i].b + G_R5SATURN_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4 + r5 * rho5) / 100000000.0
}
