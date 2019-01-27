use super::aavsop87;
use super::aavsop87d_jup;
use super::aacoordinatetransformation;


const G_L0JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 64] =
    [
        aavsop87::VSOP87Coefficient { a: 59954691.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 9695899.0, b: 5.0619179, c: 529.6909651 },
        aavsop87::VSOP87Coefficient { a: 573610.0, b: 1.444062, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 306389.0, b: 5.417347, c: 1059.381930 },
        aavsop87::VSOP87Coefficient { a: 97178.0, b: 4.14265, c: 632.78374 },
        aavsop87::VSOP87Coefficient { a: 72903.0, b: 3.64043, c: 522.57742 },
        aavsop87::VSOP87Coefficient { a: 64264.0, b: 3.41145, c: 103.09277 },
        aavsop87::VSOP87Coefficient { a: 39806.0, b: 2.29377, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 38858.0, b: 1.27232, c: 316.39187 },
        aavsop87::VSOP87Coefficient { a: 27965.0, b: 1.78455, c: 536.80451 },
        aavsop87::VSOP87Coefficient { a: 13590.0, b: 5.77481, c: 1589.07290 },
        aavsop87::VSOP87Coefficient { a: 8769.0, b: 3.6300, c: 949.1756 },
        aavsop87::VSOP87Coefficient { a: 8246.0, b: 3.5823, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 7368.0, b: 5.0810, c: 735.8765 },
        aavsop87::VSOP87Coefficient { a: 6263.0, b: 0.0250, c: 213.2991 },
        aavsop87::VSOP87Coefficient { a: 6114.0, b: 4.5132, c: 1162.4747 },
        aavsop87::VSOP87Coefficient { a: 5305.0, b: 4.1863, c: 1052.2684 },
        aavsop87::VSOP87Coefficient { a: 5305.0, b: 1.3067, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 4905.0, b: 1.3208, c: 110.2063 },
        aavsop87::VSOP87Coefficient { a: 4647.0, b: 4.6996, c: 3.9322 },
        aavsop87::VSOP87Coefficient { a: 3045.0, b: 4.3168, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 2610.0, b: 1.5667, c: 846.0828 },
        aavsop87::VSOP87Coefficient { a: 2028.0, b: 1.0638, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 1921.0, b: 0.9717, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 1765.0, b: 2.1415, c: 1066.4955 },
        aavsop87::VSOP87Coefficient { a: 1723.0, b: 3.8804, c: 1265.5675 },
        aavsop87::VSOP87Coefficient { a: 1633.0, b: 3.5820, c: 515.4639 },
        aavsop87::VSOP87Coefficient { a: 1432.0, b: 4.2968, c: 625.6702 },
        aavsop87::VSOP87Coefficient { a: 973.0, b: 4.098, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 884.0, b: 2.437, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 733.0, b: 6.085, c: 838.969 },
        aavsop87::VSOP87Coefficient { a: 731.0, b: 3.806, c: 1581.959 },
        aavsop87::VSOP87Coefficient { a: 709.0, b: 1.293, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 692.0, b: 6.134, c: 2118.764 },
        aavsop87::VSOP87Coefficient { a: 614.0, b: 4.109, c: 1478.867 },
        aavsop87::VSOP87Coefficient { a: 582.0, b: 4.540, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 495.0, b: 3.756, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 441.0, b: 2.958, c: 454.909 },
        aavsop87::VSOP87Coefficient { a: 417.0, b: 1.036, c: 2.448 },
        aavsop87::VSOP87Coefficient { a: 390.0, b: 4.897, c: 1692.166 },
        aavsop87::VSOP87Coefficient { a: 376.0, b: 4.703, c: 1368.660 },
        aavsop87::VSOP87Coefficient { a: 341.0, b: 5.715, c: 533.623 },
        aavsop87::VSOP87Coefficient { a: 330.0, b: 4.740, c: 0.048 },
        aavsop87::VSOP87Coefficient { a: 262.0, b: 1.877, c: 0.963 },
        aavsop87::VSOP87Coefficient { a: 261.0, b: 0.820, c: 380.128 },
        aavsop87::VSOP87Coefficient { a: 257.0, b: 3.724, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 244.0, b: 5.220, c: 728.763 },
        aavsop87::VSOP87Coefficient { a: 235.0, b: 1.227, c: 909.819 },
        aavsop87::VSOP87Coefficient { a: 220.0, b: 1.651, c: 543.918 },
        aavsop87::VSOP87Coefficient { a: 207.0, b: 1.855, c: 525.759 },
        aavsop87::VSOP87Coefficient { a: 202.0, b: 1.807, c: 1375.774 },
        aavsop87::VSOP87Coefficient { a: 197.0, b: 5.293, c: 1155.361 },
        aavsop87::VSOP87Coefficient { a: 175.0, b: 3.730, c: 942.062 },
        aavsop87::VSOP87Coefficient { a: 175.0, b: 3.226, c: 1898.351 },
        aavsop87::VSOP87Coefficient { a: 175.0, b: 5.910, c: 956.289 },
        aavsop87::VSOP87Coefficient { a: 158.0, b: 4.365, c: 1795.258 },
        aavsop87::VSOP87Coefficient { a: 151.0, b: 3.906, c: 74.782 },
        aavsop87::VSOP87Coefficient { a: 149.0, b: 4.377, c: 1685.052 },
        aavsop87::VSOP87Coefficient { a: 141.0, b: 3.136, c: 491.558 },
        aavsop87::VSOP87Coefficient { a: 138.0, b: 1.318, c: 1169.588 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 4.169, c: 1045.155 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 2.500, c: 1596.186 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 3.389, c: 0.521 },
        aavsop87::VSOP87Coefficient { a: 106.0, b: 4.554, c: 526.510 }
    ];

const G_L1JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 61] =
    [
        aavsop87::VSOP87Coefficient { a: 52993480757.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 489741.0, b: 4.220667, c: 529.690965 },
        aavsop87::VSOP87Coefficient { a: 228919.0, b: 6.026475, c: 7.113547 },
        aavsop87::VSOP87Coefficient { a: 27655.0, b: 4.57266, c: 1059.38193 },
        aavsop87::VSOP87Coefficient { a: 20721.0, b: 5.45939, c: 522.57742 },
        aavsop87::VSOP87Coefficient { a: 12106.0, b: 0.16986, c: 536.80451 },
        aavsop87::VSOP87Coefficient { a: 6068.0, b: 4.4242, c: 103.0928 },
        aavsop87::VSOP87Coefficient { a: 5434.0, b: 3.9848, c: 419.4846 },
        aavsop87::VSOP87Coefficient { a: 4238.0, b: 5.8901, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 2212.0, b: 5.2677, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 1746.0, b: 4.9267, c: 1589.0729 },
        aavsop87::VSOP87Coefficient { a: 1296.0, b: 5.5513, c: 3.1814 },
        aavsop87::VSOP87Coefficient { a: 1173.0, b: 5.8565, c: 1052.2684 },
        aavsop87::VSOP87Coefficient { a: 1163.0, b: 0.5145, c: 3.9322 },
        aavsop87::VSOP87Coefficient { a: 1099.0, b: 5.3070, c: 515.4639 },
        aavsop87::VSOP87Coefficient { a: 1007.0, b: 0.4648, c: 735.8765 },
        aavsop87::VSOP87Coefficient { a: 1004.0, b: 3.1504, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 848.0, b: 5.758, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 827.0, b: 4.803, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 816.0, b: 0.586, c: 1066.495 },
        aavsop87::VSOP87Coefficient { a: 725.0, b: 5.518, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 568.0, b: 5.989, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 474.0, b: 4.132, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 413.0, b: 5.737, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 345.0, b: 4.242, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 336.0, b: 3.732, c: 1162.475 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 4.035, c: 949.176 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 6.243, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 199.0, b: 1.505, c: 838.969 },
        aavsop87::VSOP87Coefficient { a: 195.0, b: 2.219, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 187.0, b: 6.086, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 184.0, b: 6.280, c: 543.918 },
        aavsop87::VSOP87Coefficient { a: 171.0, b: 5.417, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 131.0, b: 0.626, c: 728.763 },
        aavsop87::VSOP87Coefficient { a: 115.0, b: 0.680, c: 846.083 },
        aavsop87::VSOP87Coefficient { a: 115.0, b: 5.286, c: 2118.764 },
        aavsop87::VSOP87Coefficient { a: 108.0, b: 4.493, c: 956.289 },
        aavsop87::VSOP87Coefficient { a: 80.0, b: 5.82, c: 1045.15 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 5.34, c: 942.06 },
        aavsop87::VSOP87Coefficient { a: 70.0, b: 5.97, c: 532.87 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 5.73, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 66.0, b: 0.13, c: 526.51 },
        aavsop87::VSOP87Coefficient { a: 65.0, b: 6.09, c: 1581.96 },
        aavsop87::VSOP87Coefficient { a: 59.0, b: 0.59, c: 1155.36 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 0.99, c: 1596.19 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 5.97, c: 1169.59 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 1.41, c: 533.62 },
        aavsop87::VSOP87Coefficient { a: 55.0, b: 5.43, c: 10.29 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 5.73, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 0.23, c: 1368.66 },
        aavsop87::VSOP87Coefficient { a: 50.0, b: 6.08, c: 525.76 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 3.63, c: 1478.87 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 0.51, c: 1265.57 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 4.16, c: 1692.17 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 0.10, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 33.0, b: 5.04, c: 220.41 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 5.37, c: 508.35 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 5.42, c: 1272.68 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 3.36, c: 4.67 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 0.76, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 1.61, c: 831.86 }
    ];

const G_L2JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 57] =
    [
        aavsop87::VSOP87Coefficient { a: 47234.0, b: 4.32148, c: 7.11355 },
        aavsop87::VSOP87Coefficient { a: 38966.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 30629.0, b: 2.93021, c: 529.69097 },
        aavsop87::VSOP87Coefficient { a: 3189.0, b: 1.0550, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 2729.0, b: 4.8455, c: 536.8045 },
        aavsop87::VSOP87Coefficient { a: 2723.0, b: 3.4141, c: 1059.3819 },
        aavsop87::VSOP87Coefficient { a: 1721.0, b: 4.1873, c: 14.2271 },
        aavsop87::VSOP87Coefficient { a: 383.0, b: 5.768, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 378.0, b: 0.760, c: 515.464 },
        aavsop87::VSOP87Coefficient { a: 367.0, b: 6.055, c: 103.093 },
        aavsop87::VSOP87Coefficient { a: 337.0, b: 3.786, c: 3.181 },
        aavsop87::VSOP87Coefficient { a: 308.0, b: 0.694, c: 206.186 },
        aavsop87::VSOP87Coefficient { a: 218.0, b: 3.814, c: 1589.073 },
        aavsop87::VSOP87Coefficient { a: 199.0, b: 5.340, c: 1066.495 },
        aavsop87::VSOP87Coefficient { a: 197.0, b: 2.484, c: 3.932 },
        aavsop87::VSOP87Coefficient { a: 156.0, b: 1.406, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 146.0, b: 3.814, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 142.0, b: 1.634, c: 426.598 },
        aavsop87::VSOP87Coefficient { a: 130.0, b: 5.837, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 117.0, b: 1.414, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 97.0, b: 4.03, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 91.0, b: 1.11, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 87.0, b: 2.52, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 79.0, b: 4.64, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 72.0, b: 2.22, c: 735.88 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 0.83, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 57.0, b: 3.12, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 1.67, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 4.02, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 0.62, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 2.33, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 3.61, c: 10.29 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 3.24, c: 838.97 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 4.50, c: 742.99 },
        aavsop87::VSOP87Coefficient { a: 26.0, b: 2.51, c: 1162.47 },
        aavsop87::VSOP87Coefficient { a: 25.0, b: 1.22, c: 1045.15 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 3.01, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 4.29, c: 532.87 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 0.81, c: 508.35 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 4.20, c: 2118.76 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 1.83, c: 526.51 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 5.81, c: 1596.19 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 0.68, c: 942.06 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 4.00, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 5.95, c: 316.39 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 1.80, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 2.52, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 4.37, c: 1169.59 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 4.44, c: 525.76 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 1.72, c: 1581.96 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.18, c: 1155.36 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.29, c: 220.41 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.32, c: 831.86 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.76, c: 846.08 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.71, c: 533.62 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 2.18, c: 1265.57 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 0.50, c: 949.18 }
    ];

const G_L3JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 39] =
    [
        aavsop87::VSOP87Coefficient { a: 6502.0, b: 2.5986, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 1357.0, b: 1.3464, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 471.0, b: 2.475, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 417.0, b: 3.245, c: 536.805 },
        aavsop87::VSOP87Coefficient { a: 353.0, b: 2.974, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 155.0, b: 2.076, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 87.0, b: 2.51, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 3.83, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 28.0, b: 2.45, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 24.0, b: 1.28, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 23.0, b: 2.98, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 2.10, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 20.0, b: 1.40, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 19.0, b: 1.59, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.30, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 17.0, b: 2.60, c: 1589.07 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 3.15, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 3.36, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 2.76, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 2.54, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 6.27, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 1.76, c: 10.29 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 2.27, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 3.43, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 4.04, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 2.52, c: 508.35 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 2.91, c: 1045.15 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 5.25, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 4.30, c: 88.87 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 3.52, c: 302.16 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 4.09, c: 735.88 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 1.43, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.36, c: 1596.19 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 1.25, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 5.02, c: 838.97 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.24, c: 117.32 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.90, c: 742.99 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 2.36, c: 942.06 }
    ];

const G_L4JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 19] =
    [
        aavsop87::VSOP87Coefficient { a: 669.0, b: 0.853, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 100.0, b: 0.743, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 50.0, b: 1.65, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 5.82, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.86, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 4.29, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.71, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 1.30, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 2.32, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 0.48, c: 21.34 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.00, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 0.40, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.26, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.91, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.26, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 5.26, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.72, c: 95.98 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 1.29, c: 1589.07 }
    ];

const G_L5JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 5] =
    [
        aavsop87::VSOP87Coefficient { a: 50.0, b: 5.26, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 16.0, b: 5.25, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 0.01, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 1.10, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 3.14, c: 0.0 }
    ];

const G_B0JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 26] =
    [
        aavsop87::VSOP87Coefficient { a: 2268616.0, b: 3.5585261, c: 529.6909651 },
        aavsop87::VSOP87Coefficient { a: 110090.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 109972.0, b: 3.908093, c: 1059.381930 },
        aavsop87::VSOP87Coefficient { a: 8101.0, b: 3.6051, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 6438.0, b: 0.3063, c: 536.8045 },
        aavsop87::VSOP87Coefficient { a: 6044.0, b: 4.2588, c: 1589.0729 },
        aavsop87::VSOP87Coefficient { a: 1107.0, b: 2.9853, c: 1162.4747 },
        aavsop87::VSOP87Coefficient { a: 944.0, b: 1.675, c: 426.598 },
        aavsop87::VSOP87Coefficient { a: 942.0, b: 2.936, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 894.0, b: 1.754, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 836.0, b: 5.179, c: 103.093 },
        aavsop87::VSOP87Coefficient { a: 767.0, b: 2.155, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 684.0, b: 3.678, c: 213.299 },
        aavsop87::VSOP87Coefficient { a: 629.0, b: 0.643, c: 1066.495 },
        aavsop87::VSOP87Coefficient { a: 559.0, b: 0.014, c: 846.083 },
        aavsop87::VSOP87Coefficient { a: 532.0, b: 2.703, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 464.0, b: 1.173, c: 949.176 },
        aavsop87::VSOP87Coefficient { a: 431.0, b: 2.608, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 351.0, b: 4.611, c: 2118.764 },
        aavsop87::VSOP87Coefficient { a: 132.0, b: 4.778, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 123.0, b: 3.350, c: 1692.166 },
        aavsop87::VSOP87Coefficient { a: 116.0, b: 1.387, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 115.0, b: 5.049, c: 316.392 },
        aavsop87::VSOP87Coefficient { a: 104.0, b: 3.701, c: 515.464 },
        aavsop87::VSOP87Coefficient { a: 103.0, b: 2.319, c: 1478.867 },
        aavsop87::VSOP87Coefficient { a: 102.0, b: 3.153, c: 1581.959 }
    ];

const G_B1JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 22] =
    [
        aavsop87::VSOP87Coefficient { a: 177352.0, b: 5.701665, c: 529.690965 },
        aavsop87::VSOP87Coefficient { a: 3230.0, b: 5.7794, c: 1059.3819 },
        aavsop87::VSOP87Coefficient { a: 3081.0, b: 5.4746, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 2212.0, b: 4.7348, c: 536.8045 },
        aavsop87::VSOP87Coefficient { a: 1694.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 346.0, b: 4.746, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 234.0, b: 5.189, c: 1066.495 },
        aavsop87::VSOP87Coefficient { a: 196.0, b: 6.186, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 150.0, b: 3.927, c: 1589.073 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 3.439, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 97.0, b: 2.91, c: 949.18 },
        aavsop87::VSOP87Coefficient { a: 82.0, b: 5.08, c: 1162.47 },
        aavsop87::VSOP87Coefficient { a: 77.0, b: 2.51, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 77.0, b: 0.61, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 74.0, b: 5.50, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 61.0, b: 5.45, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 50.0, b: 3.95, c: 735.88 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 0.54, c: 110.21 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 1.90, c: 846.08 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 4.70, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 36.0, b: 6.11, c: 316.39 },
        aavsop87::VSOP87Coefficient { a: 32.0, b: 4.92, c: 1581.96 }
    ];

const G_B2JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 14] =
    [
        aavsop87::VSOP87Coefficient { a: 8094.0, b: 1.4632, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 813.0, b: 3.1416, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 742.0, b: 0.957, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 399.0, b: 2.899, c: 536.805 },
        aavsop87::VSOP87Coefficient { a: 342.0, b: 1.447, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 74.0, b: 0.41, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 46.0, b: 3.48, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 1.93, c: 1589.07 },
        aavsop87::VSOP87Coefficient { a: 29.0, b: 0.99, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 23.0, b: 4.27, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 2.92, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 5.22, c: 632.78 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 4.88, c: 949.18 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 6.21, c: 1045.15 }
    ];

const G_B3JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 9] =
    [
        aavsop87::VSOP87Coefficient { a: 252.0, b: 3.381, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 122.0, b: 2.733, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 49.0, b: 1.04, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 2.31, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 2.77, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 4.25, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 1.78, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 1.13, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.14, c: 0.0 }
    ];

const G_B4JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 6] =
    [
        aavsop87::VSOP87Coefficient { a: 15.0, b: 4.53, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 4.47, c: 529.69 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 5.44, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.52, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 1.0, b: 4.20, c: 1052.27 }
    ];

const G_B5JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 1] =
    [
        aavsop87::VSOP87Coefficient { a: 1.0, b: 0.09, c: 522.58 }
    ];

const G_R0JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 46] =
    [
        aavsop87::VSOP87Coefficient { a: 520887429.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 25209327.0, b: 3.49108640, c: 529.69096509 },
        aavsop87::VSOP87Coefficient { a: 610600.0, b: 3.841154, c: 1059.381930 },
        aavsop87::VSOP87Coefficient { a: 282029.0, b: 2.574199, c: 632.783739 },
        aavsop87::VSOP87Coefficient { a: 187647.0, b: 2.075904, c: 522.577418 },
        aavsop87::VSOP87Coefficient { a: 86793.0, b: 0.71001, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 72063.0, b: 0.21466, c: 536.80451 },
        aavsop87::VSOP87Coefficient { a: 65517.0, b: 5.97996, c: 316.39187 },
        aavsop87::VSOP87Coefficient { a: 30135.0, b: 2.16132, c: 949.17561 },
        aavsop87::VSOP87Coefficient { a: 29135.0, b: 1.67759, c: 103.09277 },
        aavsop87::VSOP87Coefficient { a: 23947.0, b: 0.27458, c: 7.11355 },
        aavsop87::VSOP87Coefficient { a: 23453.0, b: 3.54023, c: 735.87651 },
        aavsop87::VSOP87Coefficient { a: 22284.0, b: 4.19363, c: 1589.07290 },
        aavsop87::VSOP87Coefficient { a: 13033.0, b: 2.96043, c: 1162.47470 },
        aavsop87::VSOP87Coefficient { a: 12749.0, b: 2.71550, c: 1052.26838 },
        aavsop87::VSOP87Coefficient { a: 9703.0, b: 1.9067, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 9161.0, b: 4.4135, c: 213.2991 },
        aavsop87::VSOP87Coefficient { a: 7895.0, b: 2.4791, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 7058.0, b: 2.1818, c: 1265.5675 },
        aavsop87::VSOP87Coefficient { a: 6138.0, b: 6.2642, c: 846.0828 },
        aavsop87::VSOP87Coefficient { a: 5477.0, b: 5.6573, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 4170.0, b: 2.0161, c: 515.4639 },
        aavsop87::VSOP87Coefficient { a: 4137.0, b: 2.7222, c: 625.6702 },
        aavsop87::VSOP87Coefficient { a: 3503.0, b: 0.5653, c: 1066.4955 },
        aavsop87::VSOP87Coefficient { a: 2617.0, b: 2.0099, c: 1581.9593 },
        aavsop87::VSOP87Coefficient { a: 2500.0, b: 4.5518, c: 838.9693 },
        aavsop87::VSOP87Coefficient { a: 2128.0, b: 6.1275, c: 742.9901 },
        aavsop87::VSOP87Coefficient { a: 1912.0, b: 0.8562, c: 412.3711 },
        aavsop87::VSOP87Coefficient { a: 1611.0, b: 3.0887, c: 1368.6603 },
        aavsop87::VSOP87Coefficient { a: 1479.0, b: 2.6803, c: 1478.8666 },
        aavsop87::VSOP87Coefficient { a: 1231.0, b: 1.8904, c: 323.5054 },
        aavsop87::VSOP87Coefficient { a: 1217.0, b: 1.8017, c: 110.2063 },
        aavsop87::VSOP87Coefficient { a: 1015.0, b: 1.3867, c: 454.9094 },
        aavsop87::VSOP87Coefficient { a: 999.0, b: 2.872, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 961.0, b: 4.549, c: 2118.764 },
        aavsop87::VSOP87Coefficient { a: 886.0, b: 4.148, c: 533.623 },
        aavsop87::VSOP87Coefficient { a: 821.0, b: 1.593, c: 1898.351 },
        aavsop87::VSOP87Coefficient { a: 812.0, b: 5.941, c: 909.819 },
        aavsop87::VSOP87Coefficient { a: 777.0, b: 3.677, c: 728.763 },
        aavsop87::VSOP87Coefficient { a: 727.0, b: 3.988, c: 1155.361 },
        aavsop87::VSOP87Coefficient { a: 655.0, b: 2.791, c: 1685.052 },
        aavsop87::VSOP87Coefficient { a: 654.0, b: 3.382, c: 1692.166 },
        aavsop87::VSOP87Coefficient { a: 621.0, b: 4.823, c: 956.289 },
        aavsop87::VSOP87Coefficient { a: 615.0, b: 2.276, c: 942.062 },
        aavsop87::VSOP87Coefficient { a: 562.0, b: 0.081, c: 543.918 },
        aavsop87::VSOP87Coefficient { a: 542.0, b: 0.284, c: 525.759 }
    ];

const G_R1JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 43] =
    [
        aavsop87::VSOP87Coefficient { a: 1271802.0, b: 2.6493751, c: 529.6909651 },
        aavsop87::VSOP87Coefficient { a: 61662.0, b: 3.00076, c: 1059.38193 },
        aavsop87::VSOP87Coefficient { a: 53444.0, b: 3.89718, c: 522.57742 },
        aavsop87::VSOP87Coefficient { a: 41390.0, b: 0.0, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 31185.0, b: 4.88277, c: 536.80451 },
        aavsop87::VSOP87Coefficient { a: 11847.0, b: 2.41330, c: 419.48464 },
        aavsop87::VSOP87Coefficient { a: 9166.0, b: 4.7598, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 3404.0, b: 3.3469, c: 1589.0729 },
        aavsop87::VSOP87Coefficient { a: 3203.0, b: 5.2108, c: 735.8765 },
        aavsop87::VSOP87Coefficient { a: 3176.0, b: 2.7930, c: 103.0928 },
        aavsop87::VSOP87Coefficient { a: 2806.0, b: 3.7422, c: 515.4639 },
        aavsop87::VSOP87Coefficient { a: 2677.0, b: 4.3305, c: 1052.2684 },
        aavsop87::VSOP87Coefficient { a: 2600.0, b: 3.6344, c: 206.1855 },
        aavsop87::VSOP87Coefficient { a: 2412.0, b: 1.4695, c: 426.5982 },
        aavsop87::VSOP87Coefficient { a: 2101.0, b: 3.9276, c: 639.8973 },
        aavsop87::VSOP87Coefficient { a: 1646.0, b: 5.3095, c: 1066.4955 },
        aavsop87::VSOP87Coefficient { a: 1641.0, b: 4.4163, c: 625.6702 },
        aavsop87::VSOP87Coefficient { a: 1050.0, b: 3.1611, c: 213.2991 },
        aavsop87::VSOP87Coefficient { a: 1025.0, b: 2.5543, c: 412.3711 },
        aavsop87::VSOP87Coefficient { a: 806.0, b: 2.678, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 741.0, b: 2.171, c: 1162.475 },
        aavsop87::VSOP87Coefficient { a: 677.0, b: 6.250, c: 838.969 },
        aavsop87::VSOP87Coefficient { a: 567.0, b: 4.577, c: 742.990 },
        aavsop87::VSOP87Coefficient { a: 485.0, b: 2.469, c: 949.176 },
        aavsop87::VSOP87Coefficient { a: 469.0, b: 4.710, c: 543.918 },
        aavsop87::VSOP87Coefficient { a: 445.0, b: 0.403, c: 323.505 },
        aavsop87::VSOP87Coefficient { a: 416.0, b: 5.368, c: 728.763 },
        aavsop87::VSOP87Coefficient { a: 402.0, b: 4.605, c: 309.278 },
        aavsop87::VSOP87Coefficient { a: 347.0, b: 4.681, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 338.0, b: 3.168, c: 956.289 },
        aavsop87::VSOP87Coefficient { a: 261.0, b: 5.343, c: 846.083 },
        aavsop87::VSOP87Coefficient { a: 247.0, b: 3.923, c: 942.062 },
        aavsop87::VSOP87Coefficient { a: 220.0, b: 4.842, c: 1368.660 },
        aavsop87::VSOP87Coefficient { a: 203.0, b: 5.600, c: 1155.361 },
        aavsop87::VSOP87Coefficient { a: 200.0, b: 4.439, c: 1045.155 },
        aavsop87::VSOP87Coefficient { a: 197.0, b: 3.706, c: 2118.764 },
        aavsop87::VSOP87Coefficient { a: 196.0, b: 3.759, c: 199.072 },
        aavsop87::VSOP87Coefficient { a: 184.0, b: 4.265, c: 95.979 },
        aavsop87::VSOP87Coefficient { a: 180.0, b: 4.402, c: 532.872 },
        aavsop87::VSOP87Coefficient { a: 170.0, b: 4.846, c: 526.510 },
        aavsop87::VSOP87Coefficient { a: 146.0, b: 6.130, c: 533.623 },
        aavsop87::VSOP87Coefficient { a: 133.0, b: 1.322, c: 110.206 },
        aavsop87::VSOP87Coefficient { a: 132.0, b: 4.512, c: 525.759 }
    ];

const G_R2JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 36] =
    [
        aavsop87::VSOP87Coefficient { a: 79645.0, b: 1.35866, c: 529.69097 },
        aavsop87::VSOP87Coefficient { a: 8252.0, b: 5.7777, c: 522.5774 },
        aavsop87::VSOP87Coefficient { a: 7030.0, b: 3.2748, c: 536.8045 },
        aavsop87::VSOP87Coefficient { a: 5314.0, b: 1.8384, c: 1059.3819 },
        aavsop87::VSOP87Coefficient { a: 1861.0, b: 2.9768, c: 7.1135 },
        aavsop87::VSOP87Coefficient { a: 964.0, b: 5.480, c: 515.464 },
        aavsop87::VSOP87Coefficient { a: 836.0, b: 4.199, c: 419.485 },
        aavsop87::VSOP87Coefficient { a: 498.0, b: 3.142, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 427.0, b: 2.228, c: 639.897 },
        aavsop87::VSOP87Coefficient { a: 406.0, b: 3.783, c: 1066.495 },
        aavsop87::VSOP87Coefficient { a: 377.0, b: 2.242, c: 1589.073 },
        aavsop87::VSOP87Coefficient { a: 363.0, b: 5.368, c: 206.186 },
        aavsop87::VSOP87Coefficient { a: 342.0, b: 6.099, c: 1052.268 },
        aavsop87::VSOP87Coefficient { a: 339.0, b: 6.127, c: 625.670 },
        aavsop87::VSOP87Coefficient { a: 333.0, b: 0.003, c: 426.598 },
        aavsop87::VSOP87Coefficient { a: 280.0, b: 4.262, c: 412.371 },
        aavsop87::VSOP87Coefficient { a: 257.0, b: 0.963, c: 632.784 },
        aavsop87::VSOP87Coefficient { a: 230.0, b: 0.705, c: 735.877 },
        aavsop87::VSOP87Coefficient { a: 201.0, b: 3.069, c: 543.918 },
        aavsop87::VSOP87Coefficient { a: 200.0, b: 4.429, c: 103.093 },
        aavsop87::VSOP87Coefficient { a: 139.0, b: 2.932, c: 14.227 },
        aavsop87::VSOP87Coefficient { a: 114.0, b: 0.787, c: 728.763 },
        aavsop87::VSOP87Coefficient { a: 95.0, b: 1.70, c: 838.97 },
        aavsop87::VSOP87Coefficient { a: 86.0, b: 5.14, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 0.06, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 80.0, b: 2.98, c: 742.99 },
        aavsop87::VSOP87Coefficient { a: 75.0, b: 1.60, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 70.0, b: 1.51, c: 213.30 },
        aavsop87::VSOP87Coefficient { a: 67.0, b: 5.47, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 62.0, b: 6.10, c: 1045.15 },
        aavsop87::VSOP87Coefficient { a: 56.0, b: 0.96, c: 1162.47 },
        aavsop87::VSOP87Coefficient { a: 52.0, b: 5.58, c: 942.06 },
        aavsop87::VSOP87Coefficient { a: 50.0, b: 2.72, c: 532.87 },
        aavsop87::VSOP87Coefficient { a: 45.0, b: 5.52, c: 508.35 },
        aavsop87::VSOP87Coefficient { a: 44.0, b: 0.27, c: 526.51 },
        aavsop87::VSOP87Coefficient { a: 40.0, b: 5.95, c: 95.98 }
    ];

const G_R3JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 28] =
    [
        aavsop87::VSOP87Coefficient { a: 3519.0, b: 6.0580, c: 529.6910 },
        aavsop87::VSOP87Coefficient { a: 1073.0, b: 1.6732, c: 536.8045 },
        aavsop87::VSOP87Coefficient { a: 916.0, b: 1.413, c: 522.577 },
        aavsop87::VSOP87Coefficient { a: 342.0, b: 0.523, c: 1059.382 },
        aavsop87::VSOP87Coefficient { a: 255.0, b: 1.196, c: 7.114 },
        aavsop87::VSOP87Coefficient { a: 222.0, b: 0.952, c: 515.464 },
        aavsop87::VSOP87Coefficient { a: 90.0, b: 3.14, c: 0.0 },
        aavsop87::VSOP87Coefficient { a: 69.0, b: 2.27, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 1.41, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 58.0, b: 0.53, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 51.0, b: 5.98, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 47.0, b: 1.58, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 43.0, b: 6.12, c: 419.48 },
        aavsop87::VSOP87Coefficient { a: 37.0, b: 1.18, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 1.67, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 34.0, b: 0.85, c: 206.19 },
        aavsop87::VSOP87Coefficient { a: 31.0, b: 1.04, c: 1589.07 },
        aavsop87::VSOP87Coefficient { a: 30.0, b: 4.63, c: 426.60 },
        aavsop87::VSOP87Coefficient { a: 21.0, b: 2.50, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 15.0, b: 0.89, c: 199.07 },
        aavsop87::VSOP87Coefficient { a: 14.0, b: 0.96, c: 508.35 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 1.50, c: 1045.15 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 2.61, c: 735.88 },
        aavsop87::VSOP87Coefficient { a: 12.0, b: 3.56, c: 323.51 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 1.79, c: 309.28 },
        aavsop87::VSOP87Coefficient { a: 11.0, b: 6.28, c: 956.29 },
        aavsop87::VSOP87Coefficient { a: 10.0, b: 6.26, c: 103.09 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 3.45, c: 838.97 }
    ];

const G_R4JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 15] =
    [
        aavsop87::VSOP87Coefficient { a: 129.0, b: 0.084, c: 536.805 },
        aavsop87::VSOP87Coefficient { a: 113.0, b: 4.249, c: 529.691 },
        aavsop87::VSOP87Coefficient { a: 83.0, b: 3.30, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 38.0, b: 2.73, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 27.0, b: 5.69, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 18.0, b: 5.40, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 13.0, b: 6.02, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 9.0, b: 0.77, c: 1066.50 },
        aavsop87::VSOP87Coefficient { a: 8.0, b: 5.68, c: 14.23 },
        aavsop87::VSOP87Coefficient { a: 7.0, b: 1.43, c: 412.37 },
        aavsop87::VSOP87Coefficient { a: 6.0, b: 5.12, c: 639.90 },
        aavsop87::VSOP87Coefficient { a: 5.0, b: 3.34, c: 625.67 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 3.40, c: 1052.27 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 4.16, c: 728.76 },
        aavsop87::VSOP87Coefficient { a: 3.0, b: 2.90, c: 426.60 }
    ];

const G_R5JUPITER_COEFFICIENTS: [aavsop87::VSOP87Coefficient; 7] =
    [
        aavsop87::VSOP87Coefficient { a: 11.0, b: 4.75, c: 536.80 },
        aavsop87::VSOP87Coefficient { a: 4.0, b: 5.92, c: 522.58 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.57, c: 515.46 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.30, c: 543.92 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 3.69, c: 7.11 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 4.13, c: 1059.38 },
        aavsop87::VSOP87Coefficient { a: 2.0, b: 5.49, c: 1066.50 }
    ];


pub fn ecliptic_longitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_jup::l(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate l0
    let n_l0coefficients = G_L0JUPITER_COEFFICIENTS.len();
    let mut l0 = 0.0f64;
    for i in 0..n_l0coefficients {
        l0 += G_L0JUPITER_COEFFICIENTS[i].a * f64::cos(G_L0JUPITER_COEFFICIENTS[i].b + G_L0JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate l1
    let n_l1coefficients = G_L1JUPITER_COEFFICIENTS.len();
    let mut l1 = 0.0f64;
    for i in 0..n_l1coefficients {
        l1 += G_L1JUPITER_COEFFICIENTS[i].a * f64::cos(G_L1JUPITER_COEFFICIENTS[i].b + G_L1JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate l2
    let n_l2coefficients = G_L2JUPITER_COEFFICIENTS.len();
    let mut l2 = 0.0f64;
    for i in 0..n_l2coefficients {
        l2 += G_L2JUPITER_COEFFICIENTS[i].a * f64::cos(G_L2JUPITER_COEFFICIENTS[i].b + G_L2JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate l3
    let n_l3coefficients = G_L3JUPITER_COEFFICIENTS.len();
    let mut l3 = 0.0f64;
    for i in 0..n_l3coefficients {
        l3 += G_L3JUPITER_COEFFICIENTS[i].a * f64::cos(G_L3JUPITER_COEFFICIENTS[i].b + G_L3JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate l4
    let n_l4coefficients = G_L4JUPITER_COEFFICIENTS.len();
    let mut l4 = 0.0f64;
    for i in 0..n_l4coefficients {
        l4 += G_L4JUPITER_COEFFICIENTS[i].a * f64::cos(G_L4JUPITER_COEFFICIENTS[i].b + G_L4JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate l5
    let n_l5coefficients = G_L5JUPITER_COEFFICIENTS.len();
    let mut l5 = 0.0f64;
    for i in 0..n_l5coefficients {
        l5 += G_L5JUPITER_COEFFICIENTS[i].a * f64::cos(G_L5JUPITER_COEFFICIENTS[i].b + G_L5JUPITER_COEFFICIENTS[i].c * rho);
    }

    let value = (l0 + l1 * rho + l2 * rhosquared + l3 * rhocubed + l4 * rho4 + l5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_0to360_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn ecliptic_latitude(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(aavsop87d_jup::b(jd)));
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate b0
    let n_b0coefficients = G_B0JUPITER_COEFFICIENTS.len();
    let mut b0 = 0.0f64;
    for i in 0..n_b0coefficients {
        b0 += G_B0JUPITER_COEFFICIENTS[i].a * f64::cos(G_B0JUPITER_COEFFICIENTS[i].b + G_B0JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate b1
    let n_b1coefficients = G_B1JUPITER_COEFFICIENTS.len();
    let mut b1 = 0.0f64;
    for i in 0..n_b1coefficients {
        b1 += G_B1JUPITER_COEFFICIENTS[i].a * f64::cos(G_B1JUPITER_COEFFICIENTS[i].b + G_B1JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate b2
    let n_b2coefficients = G_B2JUPITER_COEFFICIENTS.len();
    let mut b2 = 0.0f64;
    for i in 0..n_b2coefficients {
        b2 += G_B2JUPITER_COEFFICIENTS[i].a * f64::cos(G_B2JUPITER_COEFFICIENTS[i].b + G_B2JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate b3
    let n_b3coefficients = G_B3JUPITER_COEFFICIENTS.len();
    let mut b3 = 0.0f64;
    for i in 0..n_b3coefficients {
        b3 += G_B3JUPITER_COEFFICIENTS[i].a * f64::cos(G_B3JUPITER_COEFFICIENTS[i].b + G_B3JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate b4
    let n_b4coefficients = G_B4JUPITER_COEFFICIENTS.len();
    let mut b4 = 0.0f64;
    for i in 0..n_b4coefficients {
        b4 += G_B4JUPITER_COEFFICIENTS[i].a * f64::cos(G_B4JUPITER_COEFFICIENTS[i].b + G_B4JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate b5
    let n_b5coefficients = G_B5JUPITER_COEFFICIENTS.len();
    let mut b5 = 0.0f64;
    for i in 0..n_b5coefficients {
        b5 += G_B5JUPITER_COEFFICIENTS[i].a * f64::cos(G_B5JUPITER_COEFFICIENTS[i].b + G_B5JUPITER_COEFFICIENTS[i].c * rho);
    }

    let value = (b0 + b1 * rho + b2 * rhosquared + b3 * rhocubed + b4 * rho4 + b5 * rho5) / 100000000.0;

    //convert results back to degrees
    aacoordinatetransformation::map_to_minus90to90_range(aacoordinatetransformation::radians_to_degrees(value))
}

pub fn radius_vector(jd: f64, b_high_precision: bool) -> f64
{
    if b_high_precision {
        return aavsop87d_jup::r(jd);
    }

    let rho = (jd - 2451545.0) / 365250.0;
    let rhosquared = rho * rho;
    let rhocubed = rhosquared * rho;
    let rho4 = rhocubed * rho;
    let rho5 = rho4 * rho;

    //Calculate r0
    let n_r0coefficients = G_R0JUPITER_COEFFICIENTS.len();
    let mut r0 = 0.0f64;
    for i in 0..n_r0coefficients {
        r0 += G_R0JUPITER_COEFFICIENTS[i].a * f64::cos(G_R0JUPITER_COEFFICIENTS[i].b + G_R0JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate r1
    let n_r1coefficients = G_R1JUPITER_COEFFICIENTS.len();
    let mut r1 = 0.0f64;
    for i in 0..n_r1coefficients {
        r1 += G_R1JUPITER_COEFFICIENTS[i].a * f64::cos(G_R1JUPITER_COEFFICIENTS[i].b + G_R1JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate r2
    let n_r2coefficients = G_R2JUPITER_COEFFICIENTS.len();
    let mut r2 = 0.0f64;
    for i in 0..n_r2coefficients {
        r2 += G_R2JUPITER_COEFFICIENTS[i].a * f64::cos(G_R2JUPITER_COEFFICIENTS[i].b + G_R2JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate r3
    let n_r3coefficients = G_R3JUPITER_COEFFICIENTS.len();
    let mut r3 = 0.0f64;
    for i in 0..n_r3coefficients {
        r3 += G_R3JUPITER_COEFFICIENTS[i].a * f64::cos(G_R3JUPITER_COEFFICIENTS[i].b + G_R3JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate r4
    let n_r4coefficients = G_R4JUPITER_COEFFICIENTS.len();
    let mut r4 = 0.0f64;
    for i in 0..n_r4coefficients {
        r4 += G_R4JUPITER_COEFFICIENTS[i].a * f64::cos(G_R4JUPITER_COEFFICIENTS[i].b + G_R4JUPITER_COEFFICIENTS[i].c * rho);
    }

    //Calculate r5
    let n_r5coefficients = G_R5JUPITER_COEFFICIENTS.len();
    let mut r5 = 0.0f64;
    for i in 0..n_r5coefficients {
        r5 += G_R5JUPITER_COEFFICIENTS[i].a * f64::cos(G_R5JUPITER_COEFFICIENTS[i].b + G_R5JUPITER_COEFFICIENTS[i].c * rho);
    }

    (r0 + r1 * rho + r2 * rhosquared + r3 * rhocubed + r4 * rho4 + r5 * rho5) / 100000000.0
}
