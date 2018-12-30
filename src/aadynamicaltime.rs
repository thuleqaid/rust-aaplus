use super::aadate;

pub fn tt2tai(jd: f64) -> f64
{
    jd - (32.184 / 86400.0)
}

pub fn tai2tt(jd: f64) -> f64
{
    jd + (32.184 / 86400.0)
}

struct DeltaTValue {
    pub jd: f64,
    pub delta_t: f64,
}

const G_DELTA_T_VALUES: [DeltaTValue; 552] =
    [
//All the initial values are observed values from 1 February 1973 to 1 June 2017 as taken from http://maia.usno.navy.mil/ser7/deltat.data
        DeltaTValue { jd: 2441714.5, delta_t: 43.4724 },
        DeltaTValue { jd: 2441742.5, delta_t: 43.5648 },
        DeltaTValue { jd: 2441773.5, delta_t: 43.6737 },
        DeltaTValue { jd: 2441803.5, delta_t: 43.7782 },
        DeltaTValue { jd: 2441834.5, delta_t: 43.8763 },
        DeltaTValue { jd: 2441864.5, delta_t: 43.9562 },
        DeltaTValue { jd: 2441895.5, delta_t: 44.0315 },
        DeltaTValue { jd: 2441926.5, delta_t: 44.1132 },
        DeltaTValue { jd: 2441956.5, delta_t: 44.1982 },
        DeltaTValue { jd: 2441987.5, delta_t: 44.2952 },
        DeltaTValue { jd: 2442017.5, delta_t: 44.3936 },
        DeltaTValue { jd: 2442048.5, delta_t: 44.4841 },
        DeltaTValue { jd: 2442079.5, delta_t: 44.5646 },
        DeltaTValue { jd: 2442107.5, delta_t: 44.6425 },
        DeltaTValue { jd: 2442138.5, delta_t: 44.7386 },
        DeltaTValue { jd: 2442168.5, delta_t: 44.8370 },
        DeltaTValue { jd: 2442199.5, delta_t: 44.9302 },
        DeltaTValue { jd: 2442229.5, delta_t: 44.9986 },
        DeltaTValue { jd: 2442260.5, delta_t: 45.0584 },
        DeltaTValue { jd: 2442291.5, delta_t: 45.1284 },
        DeltaTValue { jd: 2442321.5, delta_t: 45.2064 },
        DeltaTValue { jd: 2442352.5, delta_t: 45.2980 },
        DeltaTValue { jd: 2442382.5, delta_t: 45.3897 },
        DeltaTValue { jd: 2442413.5, delta_t: 45.4761 },
        DeltaTValue { jd: 2442444.5, delta_t: 45.5633 },
        DeltaTValue { jd: 2442472.5, delta_t: 45.6450 },
        DeltaTValue { jd: 2442503.5, delta_t: 45.7375 },
        DeltaTValue { jd: 2442533.5, delta_t: 45.8284 },
        DeltaTValue { jd: 2442564.5, delta_t: 45.9133 },
        DeltaTValue { jd: 2442594.5, delta_t: 45.9820 },
        DeltaTValue { jd: 2442625.5, delta_t: 46.0408 },
        DeltaTValue { jd: 2442656.5, delta_t: 46.1067 },
        DeltaTValue { jd: 2442686.5, delta_t: 46.1825 },
        DeltaTValue { jd: 2442717.5, delta_t: 46.2789 },
        DeltaTValue { jd: 2442747.5, delta_t: 46.3713 },
        DeltaTValue { jd: 2442778.5, delta_t: 46.4567 },
        DeltaTValue { jd: 2442809.5, delta_t: 46.5445 },
        DeltaTValue { jd: 2442838.5, delta_t: 46.6311 },
        DeltaTValue { jd: 2442869.5, delta_t: 46.7302 },
        DeltaTValue { jd: 2442899.5, delta_t: 46.8284 },
        DeltaTValue { jd: 2442930.5, delta_t: 46.9247 },
        DeltaTValue { jd: 2442960.5, delta_t: 46.9970 },
        DeltaTValue { jd: 2442991.5, delta_t: 47.0709 },
        DeltaTValue { jd: 2443022.5, delta_t: 47.1451 },
        DeltaTValue { jd: 2443052.5, delta_t: 47.2362 },
        DeltaTValue { jd: 2443083.5, delta_t: 47.3413 },
        DeltaTValue { jd: 2443113.5, delta_t: 47.4319 },
        DeltaTValue { jd: 2443144.5, delta_t: 47.5214 },
        DeltaTValue { jd: 2443175.5, delta_t: 47.6049 },
        DeltaTValue { jd: 2443203.5, delta_t: 47.6837 },
        DeltaTValue { jd: 2443234.5, delta_t: 47.7781 },
        DeltaTValue { jd: 2443264.5, delta_t: 47.8771 },
        DeltaTValue { jd: 2443295.5, delta_t: 47.9687 },
        DeltaTValue { jd: 2443325.5, delta_t: 48.0348 },
        DeltaTValue { jd: 2443356.5, delta_t: 48.0942 },
        DeltaTValue { jd: 2443387.5, delta_t: 48.1608 },
        DeltaTValue { jd: 2443417.5, delta_t: 48.2460 },
        DeltaTValue { jd: 2443448.5, delta_t: 48.3439 },
        DeltaTValue { jd: 2443478.5, delta_t: 48.4355 },
        DeltaTValue { jd: 2443509.5, delta_t: 48.5344 },
        DeltaTValue { jd: 2443540.5, delta_t: 48.6325 },
        DeltaTValue { jd: 2443568.5, delta_t: 48.7294 },
        DeltaTValue { jd: 2443599.5, delta_t: 48.8365 },
        DeltaTValue { jd: 2443629.5, delta_t: 48.9353 },
        DeltaTValue { jd: 2443660.5, delta_t: 49.0319 },
        DeltaTValue { jd: 2443690.5, delta_t: 49.1013 },
        DeltaTValue { jd: 2443721.5, delta_t: 49.1591 },
        DeltaTValue { jd: 2443752.5, delta_t: 49.2286 },
        DeltaTValue { jd: 2443782.5, delta_t: 49.3070 },
        DeltaTValue { jd: 2443813.5, delta_t: 49.4018 },
        DeltaTValue { jd: 2443843.5, delta_t: 49.4945 },
        DeltaTValue { jd: 2443874.5, delta_t: 49.5862 },
        DeltaTValue { jd: 2443905.5, delta_t: 49.6805 },
        DeltaTValue { jd: 2443933.5, delta_t: 49.7602 },
        DeltaTValue { jd: 2443964.5, delta_t: 49.8556 },
        DeltaTValue { jd: 2443994.5, delta_t: 49.9489 },
        DeltaTValue { jd: 2444025.5, delta_t: 50.0347 },
        DeltaTValue { jd: 2444055.5, delta_t: 50.1019 },
        DeltaTValue { jd: 2444086.5, delta_t: 50.1622 },
        DeltaTValue { jd: 2444117.5, delta_t: 50.2260 },
        DeltaTValue { jd: 2444147.5, delta_t: 50.2968 },
        DeltaTValue { jd: 2444178.5, delta_t: 50.3831 },
        DeltaTValue { jd: 2444208.5, delta_t: 50.4599 },
        DeltaTValue { jd: 2444239.5, delta_t: 50.5387 },
        DeltaTValue { jd: 2444270.5, delta_t: 50.6161 },
        DeltaTValue { jd: 2444299.5, delta_t: 50.6866 },
        DeltaTValue { jd: 2444330.5, delta_t: 50.7658 },
        DeltaTValue { jd: 2444360.5, delta_t: 50.8454 },
        DeltaTValue { jd: 2444391.5, delta_t: 50.9187 },
        DeltaTValue { jd: 2444421.5, delta_t: 50.9761 },
        DeltaTValue { jd: 2444452.5, delta_t: 51.0278 },
        DeltaTValue { jd: 2444483.5, delta_t: 51.0843 },
        DeltaTValue { jd: 2444513.5, delta_t: 51.1538 },
        DeltaTValue { jd: 2444544.5, delta_t: 51.2319 },
        DeltaTValue { jd: 2444574.5, delta_t: 51.3063 },
        DeltaTValue { jd: 2444605.5, delta_t: 51.3808 },
        DeltaTValue { jd: 2444636.5, delta_t: 51.4526 },
        DeltaTValue { jd: 2444664.5, delta_t: 51.5160 },
        DeltaTValue { jd: 2444695.5, delta_t: 51.5985 },
        DeltaTValue { jd: 2444725.5, delta_t: 51.6809 },
        DeltaTValue { jd: 2444756.5, delta_t: 51.7573 },
        DeltaTValue { jd: 2444786.5, delta_t: 51.8133 },
        DeltaTValue { jd: 2444817.5, delta_t: 51.8532 },
        DeltaTValue { jd: 2444848.5, delta_t: 51.9014 },
        DeltaTValue { jd: 2444878.5, delta_t: 51.9603 },
        DeltaTValue { jd: 2444909.5, delta_t: 52.0328 },
        DeltaTValue { jd: 2444939.5, delta_t: 52.0985 },
        DeltaTValue { jd: 2444970.5, delta_t: 52.1668 },
        DeltaTValue { jd: 2445001.5, delta_t: 52.2316 },
        DeltaTValue { jd: 2445029.5, delta_t: 52.2938 },
        DeltaTValue { jd: 2445060.5, delta_t: 52.3680 },
        DeltaTValue { jd: 2445090.5, delta_t: 52.4465 },
        DeltaTValue { jd: 2445121.5, delta_t: 52.5180 },
        DeltaTValue { jd: 2445151.5, delta_t: 52.5752 },
        DeltaTValue { jd: 2445182.5, delta_t: 52.6178 },
        DeltaTValue { jd: 2445213.5, delta_t: 52.6668 },
        DeltaTValue { jd: 2445243.5, delta_t: 52.7340 },
        DeltaTValue { jd: 2445274.5, delta_t: 52.8056 },
        DeltaTValue { jd: 2445304.5, delta_t: 52.8792 },
        DeltaTValue { jd: 2445335.5, delta_t: 52.9565 },
        DeltaTValue { jd: 2445366.5, delta_t: 53.0445 },
        DeltaTValue { jd: 2445394.5, delta_t: 53.1268 },
        DeltaTValue { jd: 2445425.5, delta_t: 53.2197 },
        DeltaTValue { jd: 2445455.5, delta_t: 53.3024 },
        DeltaTValue { jd: 2445486.5, delta_t: 53.3747 },
        DeltaTValue { jd: 2445516.5, delta_t: 53.4335 },
        DeltaTValue { jd: 2445547.5, delta_t: 53.4778 },
        DeltaTValue { jd: 2445578.5, delta_t: 53.5300 },
        DeltaTValue { jd: 2445608.5, delta_t: 53.5845 },
        DeltaTValue { jd: 2445639.5, delta_t: 53.6523 },
        DeltaTValue { jd: 2445669.5, delta_t: 53.7256 },
        DeltaTValue { jd: 2445700.5, delta_t: 53.7882 },
        DeltaTValue { jd: 2445731.5, delta_t: 53.8367 },
        DeltaTValue { jd: 2445760.5, delta_t: 53.8830 },
        DeltaTValue { jd: 2445791.5, delta_t: 53.9443 },
        DeltaTValue { jd: 2445821.5, delta_t: 54.0042 },
        DeltaTValue { jd: 2445852.5, delta_t: 54.0536 },
        DeltaTValue { jd: 2445882.5, delta_t: 54.0856 },
        DeltaTValue { jd: 2445913.5, delta_t: 54.1084 },
        DeltaTValue { jd: 2445944.5, delta_t: 54.1463 },
        DeltaTValue { jd: 2445974.5, delta_t: 54.1914 },
        DeltaTValue { jd: 2446005.5, delta_t: 54.2452 },
        DeltaTValue { jd: 2446035.5, delta_t: 54.2958 },
        DeltaTValue { jd: 2446066.5, delta_t: 54.3427 },
        DeltaTValue { jd: 2446097.5, delta_t: 54.3911 },
        DeltaTValue { jd: 2446125.5, delta_t: 54.4320 },
        DeltaTValue { jd: 2446156.5, delta_t: 54.4898 },
        DeltaTValue { jd: 2446186.5, delta_t: 54.5456 },
        DeltaTValue { jd: 2446217.5, delta_t: 54.5977 },
        DeltaTValue { jd: 2446247.5, delta_t: 54.6355 },
        DeltaTValue { jd: 2446278.5, delta_t: 54.6532 },
        DeltaTValue { jd: 2446309.5, delta_t: 54.6776 },
        DeltaTValue { jd: 2446339.5, delta_t: 54.7174 },
        DeltaTValue { jd: 2446370.5, delta_t: 54.7741 },
        DeltaTValue { jd: 2446400.5, delta_t: 54.8253 },
        DeltaTValue { jd: 2446431.5, delta_t: 54.8713 },
        DeltaTValue { jd: 2446462.5, delta_t: 54.9161 },
        DeltaTValue { jd: 2446490.5, delta_t: 54.9581 },
        DeltaTValue { jd: 2446521.5, delta_t: 54.9997 },
        DeltaTValue { jd: 2446551.5, delta_t: 55.0476 },
        DeltaTValue { jd: 2446582.5, delta_t: 55.0912 },
        DeltaTValue { jd: 2446612.5, delta_t: 55.1132 },
        DeltaTValue { jd: 2446643.5, delta_t: 55.1328 },
        DeltaTValue { jd: 2446674.5, delta_t: 55.1532 },
        DeltaTValue { jd: 2446704.5, delta_t: 55.1898 },
        DeltaTValue { jd: 2446735.5, delta_t: 55.2416 },
        DeltaTValue { jd: 2446765.5, delta_t: 55.2838 },
        DeltaTValue { jd: 2446796.5, delta_t: 55.3222 },
        DeltaTValue { jd: 2446827.5, delta_t: 55.3613 },
        DeltaTValue { jd: 2446855.5, delta_t: 55.4063 },
        DeltaTValue { jd: 2446886.5, delta_t: 55.4629 },
        DeltaTValue { jd: 2446916.5, delta_t: 55.5111 },
        DeltaTValue { jd: 2446947.5, delta_t: 55.5524 },
        DeltaTValue { jd: 2446977.5, delta_t: 55.5812 },
        DeltaTValue { jd: 2447008.5, delta_t: 55.6004 },
        DeltaTValue { jd: 2447039.5, delta_t: 55.6262 },
        DeltaTValue { jd: 2447069.5, delta_t: 55.6656 },
        DeltaTValue { jd: 2447100.5, delta_t: 55.7168 },
        DeltaTValue { jd: 2447130.5, delta_t: 55.7698 },
        DeltaTValue { jd: 2447161.5, delta_t: 55.8197 },
        DeltaTValue { jd: 2447192.5, delta_t: 55.8615 },
        DeltaTValue { jd: 2447221.5, delta_t: 55.9130 },
        DeltaTValue { jd: 2447252.5, delta_t: 55.9663 },
        DeltaTValue { jd: 2447282.5, delta_t: 56.0220 },
        DeltaTValue { jd: 2447313.5, delta_t: 56.0700 },
        DeltaTValue { jd: 2447343.5, delta_t: 56.0939 },
        DeltaTValue { jd: 2447374.5, delta_t: 56.1105 },
        DeltaTValue { jd: 2447405.5, delta_t: 56.1314 },
        DeltaTValue { jd: 2447435.5, delta_t: 56.1611 },
        DeltaTValue { jd: 2447466.5, delta_t: 56.2068 },
        DeltaTValue { jd: 2447496.5, delta_t: 56.2583 },
        DeltaTValue { jd: 2447527.5, delta_t: 56.3000 },
        DeltaTValue { jd: 2447558.5, delta_t: 56.3399 },
        DeltaTValue { jd: 2447586.5, delta_t: 56.3790 },
        DeltaTValue { jd: 2447617.5, delta_t: 56.4283 },
        DeltaTValue { jd: 2447647.5, delta_t: 56.4804 },
        DeltaTValue { jd: 2447678.5, delta_t: 56.5352 },
        DeltaTValue { jd: 2447708.5, delta_t: 56.5697 },
        DeltaTValue { jd: 2447739.5, delta_t: 56.5983 },
        DeltaTValue { jd: 2447770.5, delta_t: 56.6328 },
        DeltaTValue { jd: 2447800.5, delta_t: 56.6739 },
        DeltaTValue { jd: 2447831.5, delta_t: 56.7332 },
        DeltaTValue { jd: 2447861.5, delta_t: 56.7972 },
        DeltaTValue { jd: 2447892.5, delta_t: 56.8553 },
        DeltaTValue { jd: 2447923.5, delta_t: 56.9111 },
        DeltaTValue { jd: 2447951.5, delta_t: 56.9755 },
        DeltaTValue { jd: 2447982.5, delta_t: 57.0471 },
        DeltaTValue { jd: 2448012.5, delta_t: 57.1136 },
        DeltaTValue { jd: 2448043.5, delta_t: 57.1738 },
        DeltaTValue { jd: 2448073.5, delta_t: 57.2226 },
        DeltaTValue { jd: 2448104.5, delta_t: 57.2597 },
        DeltaTValue { jd: 2448135.5, delta_t: 57.3073 },
        DeltaTValue { jd: 2448165.5, delta_t: 57.3643 },
        DeltaTValue { jd: 2448196.5, delta_t: 57.4334 },
        DeltaTValue { jd: 2448226.5, delta_t: 57.5016 },
        DeltaTValue { jd: 2448257.5, delta_t: 57.5653 },
        DeltaTValue { jd: 2448288.5, delta_t: 57.6333 },
        DeltaTValue { jd: 2448316.5, delta_t: 57.6973 },
        DeltaTValue { jd: 2448347.5, delta_t: 57.7711 },
        DeltaTValue { jd: 2448377.5, delta_t: 57.8407 },
        DeltaTValue { jd: 2448408.5, delta_t: 57.9058 },
        DeltaTValue { jd: 2448438.5, delta_t: 57.9576 },
        DeltaTValue { jd: 2448469.5, delta_t: 57.9975 },
        DeltaTValue { jd: 2448500.5, delta_t: 58.0426 },
        DeltaTValue { jd: 2448530.5, delta_t: 58.1043 },
        DeltaTValue { jd: 2448561.5, delta_t: 58.1679 },
        DeltaTValue { jd: 2448591.5, delta_t: 58.2389 },
        DeltaTValue { jd: 2448622.5, delta_t: 58.3092 },
        DeltaTValue { jd: 2448653.5, delta_t: 58.3833 },
        DeltaTValue { jd: 2448682.5, delta_t: 58.4537 },
        DeltaTValue { jd: 2448713.5, delta_t: 58.5401 },
        DeltaTValue { jd: 2448743.5, delta_t: 58.6228 },
        DeltaTValue { jd: 2448774.5, delta_t: 58.6917 },
        DeltaTValue { jd: 2448804.5, delta_t: 58.7410 },
        DeltaTValue { jd: 2448835.5, delta_t: 58.7836 },
        DeltaTValue { jd: 2448866.5, delta_t: 58.8406 },
        DeltaTValue { jd: 2448896.5, delta_t: 58.8986 },
        DeltaTValue { jd: 2448927.5, delta_t: 58.9714 },
        DeltaTValue { jd: 2448957.5, delta_t: 59.0438 },
        DeltaTValue { jd: 2448988.5, delta_t: 59.1218 },
        DeltaTValue { jd: 2449019.5, delta_t: 59.2003 },
        DeltaTValue { jd: 2449047.5, delta_t: 59.2747 },
        DeltaTValue { jd: 2449078.5, delta_t: 59.3574 },
        DeltaTValue { jd: 2449108.5, delta_t: 59.4434 },
        DeltaTValue { jd: 2449139.5, delta_t: 59.5242 },
        DeltaTValue { jd: 2449169.5, delta_t: 59.5850 },
        DeltaTValue { jd: 2449200.5, delta_t: 59.6344 },
        DeltaTValue { jd: 2449231.5, delta_t: 59.6928 },
        DeltaTValue { jd: 2449261.5, delta_t: 59.7588 },
        DeltaTValue { jd: 2449292.5, delta_t: 59.8386 },
        DeltaTValue { jd: 2449322.5, delta_t: 59.9111 },
        DeltaTValue { jd: 2449353.5, delta_t: 59.9845 },
        DeltaTValue { jd: 2449384.5, delta_t: 60.0564 },
        DeltaTValue { jd: 2449412.5, delta_t: 60.1231 },
        DeltaTValue { jd: 2449443.5, delta_t: 60.2042 },
        DeltaTValue { jd: 2449473.5, delta_t: 60.2804 },
        DeltaTValue { jd: 2449504.5, delta_t: 60.3530 },
        DeltaTValue { jd: 2449534.5, delta_t: 60.4012 },
        DeltaTValue { jd: 2449565.5, delta_t: 60.4440 },
        DeltaTValue { jd: 2449596.5, delta_t: 60.4900 },
        DeltaTValue { jd: 2449626.5, delta_t: 60.5578 },
        DeltaTValue { jd: 2449657.5, delta_t: 60.6324 },
        DeltaTValue { jd: 2449687.5, delta_t: 60.7059 },
        DeltaTValue { jd: 2449718.5, delta_t: 60.7853 },
        DeltaTValue { jd: 2449749.5, delta_t: 60.8664 },
        DeltaTValue { jd: 2449777.5, delta_t: 60.9387 },
        DeltaTValue { jd: 2449808.5, delta_t: 61.0277 },
        DeltaTValue { jd: 2449838.5, delta_t: 61.1103 },
        DeltaTValue { jd: 2449869.5, delta_t: 61.1870 },
        DeltaTValue { jd: 2449899.5, delta_t: 61.2454 },
        DeltaTValue { jd: 2449930.5, delta_t: 61.2881 },
        DeltaTValue { jd: 2449961.5, delta_t: 61.3378 },
        DeltaTValue { jd: 2449991.5, delta_t: 61.4036 },
        DeltaTValue { jd: 2450022.5, delta_t: 61.4760 },
        DeltaTValue { jd: 2450052.5, delta_t: 61.5525 },
        DeltaTValue { jd: 2450083.5, delta_t: 61.6287 },
        DeltaTValue { jd: 2450114.5, delta_t: 61.6846 },
        DeltaTValue { jd: 2450143.5, delta_t: 61.7433 },
        DeltaTValue { jd: 2450174.5, delta_t: 61.8132 },
        DeltaTValue { jd: 2450204.5, delta_t: 61.8823 },
        DeltaTValue { jd: 2450235.5, delta_t: 61.9497 },
        DeltaTValue { jd: 2450265.5, delta_t: 61.9969 },
        DeltaTValue { jd: 2450296.5, delta_t: 62.0343 },
        DeltaTValue { jd: 2450327.5, delta_t: 62.0714 },
        DeltaTValue { jd: 2450357.5, delta_t: 62.1202 },
        DeltaTValue { jd: 2450388.5, delta_t: 62.1810 },
        DeltaTValue { jd: 2450418.5, delta_t: 62.2382 },
        DeltaTValue { jd: 2450449.5, delta_t: 62.2950 },
        DeltaTValue { jd: 2450480.5, delta_t: 62.3506 },
        DeltaTValue { jd: 2450508.5, delta_t: 62.3995 },
        DeltaTValue { jd: 2450539.5, delta_t: 62.4754 },
        DeltaTValue { jd: 2450569.5, delta_t: 62.5463 },
        DeltaTValue { jd: 2450600.5, delta_t: 62.6136 },
        DeltaTValue { jd: 2450630.5, delta_t: 62.6571 },
        DeltaTValue { jd: 2450661.5, delta_t: 62.6942 },
        DeltaTValue { jd: 2450692.5, delta_t: 62.7383 },
        DeltaTValue { jd: 2450722.5, delta_t: 62.7926 },
        DeltaTValue { jd: 2450753.5, delta_t: 62.8567 },
        DeltaTValue { jd: 2450783.5, delta_t: 62.9146 },
        DeltaTValue { jd: 2450814.5, delta_t: 62.9659 },
        DeltaTValue { jd: 2450845.5, delta_t: 63.0217 },
        DeltaTValue { jd: 2450873.5, delta_t: 63.0807 },
        DeltaTValue { jd: 2450904.5, delta_t: 63.1462 },
        DeltaTValue { jd: 2450934.5, delta_t: 63.2053 },
        DeltaTValue { jd: 2450965.5, delta_t: 63.2599 },
        DeltaTValue { jd: 2450995.5, delta_t: 63.2844 },
        DeltaTValue { jd: 2451026.5, delta_t: 63.2961 },
        DeltaTValue { jd: 2451057.5, delta_t: 63.3126 },
        DeltaTValue { jd: 2451087.5, delta_t: 63.3422 },
        DeltaTValue { jd: 2451118.5, delta_t: 63.3871 },
        DeltaTValue { jd: 2451148.5, delta_t: 63.4339 },
        DeltaTValue { jd: 2451179.5, delta_t: 63.4673 },
        DeltaTValue { jd: 2451210.5, delta_t: 63.4979 },
        DeltaTValue { jd: 2451238.5, delta_t: 63.5319 },
        DeltaTValue { jd: 2451269.5, delta_t: 63.5679 },
        DeltaTValue { jd: 2451299.5, delta_t: 63.6104 },
        DeltaTValue { jd: 2451330.5, delta_t: 63.6444 },
        DeltaTValue { jd: 2451360.5, delta_t: 63.6642 },
        DeltaTValue { jd: 2451391.5, delta_t: 63.6739 },
        DeltaTValue { jd: 2451422.5, delta_t: 63.6926 },
        DeltaTValue { jd: 2451452.5, delta_t: 63.7147 },
        DeltaTValue { jd: 2451483.5, delta_t: 63.7518 },
        DeltaTValue { jd: 2451513.5, delta_t: 63.7927 },
        DeltaTValue { jd: 2451544.5, delta_t: 63.8285 },
        DeltaTValue { jd: 2451575.5, delta_t: 63.8557 },
        DeltaTValue { jd: 2451604.5, delta_t: 63.8804 },
        DeltaTValue { jd: 2451635.5, delta_t: 63.9075 },
        DeltaTValue { jd: 2451665.5, delta_t: 63.9393 },
        DeltaTValue { jd: 2451696.5, delta_t: 63.9691 },
        DeltaTValue { jd: 2451726.5, delta_t: 63.9799 },
        DeltaTValue { jd: 2451757.5, delta_t: 63.9833 },
        DeltaTValue { jd: 2451788.5, delta_t: 63.9938 },
        DeltaTValue { jd: 2451818.5, delta_t: 64.0093 },
        DeltaTValue { jd: 2451849.5, delta_t: 64.0400 },
        DeltaTValue { jd: 2451879.5, delta_t: 64.0670 },
        DeltaTValue { jd: 2451910.5, delta_t: 64.0908 },
        DeltaTValue { jd: 2451941.5, delta_t: 64.1068 },
        DeltaTValue { jd: 2451969.5, delta_t: 64.1282 },
        DeltaTValue { jd: 2452000.5, delta_t: 64.1584 },
        DeltaTValue { jd: 2452030.5, delta_t: 64.1833 },
        DeltaTValue { jd: 2452061.5, delta_t: 64.2094 },
        DeltaTValue { jd: 2452091.5, delta_t: 64.2117 },
        DeltaTValue { jd: 2452122.5, delta_t: 64.2073 },
        DeltaTValue { jd: 2452153.5, delta_t: 64.2116 },
        DeltaTValue { jd: 2452183.5, delta_t: 64.2223 },
        DeltaTValue { jd: 2452214.5, delta_t: 64.2500 },
        DeltaTValue { jd: 2452244.5, delta_t: 64.2761 },
        DeltaTValue { jd: 2452275.5, delta_t: 64.2998 },
        DeltaTValue { jd: 2452306.5, delta_t: 64.3192 },
        DeltaTValue { jd: 2452334.5, delta_t: 64.3450 },
        DeltaTValue { jd: 2452365.5, delta_t: 64.3735 },
        DeltaTValue { jd: 2452395.5, delta_t: 64.3943 },
        DeltaTValue { jd: 2452426.5, delta_t: 64.4151 },
        DeltaTValue { jd: 2452456.5, delta_t: 64.4132 },
        DeltaTValue { jd: 2452487.5, delta_t: 64.4118 },
        DeltaTValue { jd: 2452518.5, delta_t: 64.4097 },
        DeltaTValue { jd: 2452548.5, delta_t: 64.4168 },
        DeltaTValue { jd: 2452579.5, delta_t: 64.4329 },
        DeltaTValue { jd: 2452609.5, delta_t: 64.4511 },
        DeltaTValue { jd: 2452640.5, delta_t: 64.4734 },
        DeltaTValue { jd: 2452671.5, delta_t: 64.4893 },
        DeltaTValue { jd: 2452699.5, delta_t: 64.5053 },
        DeltaTValue { jd: 2452730.5, delta_t: 64.5269 },
        DeltaTValue { jd: 2452760.5, delta_t: 64.5471 },
        DeltaTValue { jd: 2452791.5, delta_t: 64.5597 },
        DeltaTValue { jd: 2452821.5, delta_t: 64.5512 },
        DeltaTValue { jd: 2452852.5, delta_t: 64.5371 },
        DeltaTValue { jd: 2452883.5, delta_t: 64.5359 },
        DeltaTValue { jd: 2452913.5, delta_t: 64.5415 },
        DeltaTValue { jd: 2452944.5, delta_t: 64.5544 },
        DeltaTValue { jd: 2452974.5, delta_t: 64.5654 },
        DeltaTValue { jd: 2453005.5, delta_t: 64.5736 },
        DeltaTValue { jd: 2453036.5, delta_t: 64.5891 },
        DeltaTValue { jd: 2453065.5, delta_t: 64.6015 },
        DeltaTValue { jd: 2453096.5, delta_t: 64.6176 },
        DeltaTValue { jd: 2453126.5, delta_t: 64.6374 },
        DeltaTValue { jd: 2453157.5, delta_t: 64.6549 },
        DeltaTValue { jd: 2453187.5, delta_t: 64.6530 },
        DeltaTValue { jd: 2453218.5, delta_t: 64.6379 },
        DeltaTValue { jd: 2453249.5, delta_t: 64.6372 },
        DeltaTValue { jd: 2453279.5, delta_t: 64.6400 },
        DeltaTValue { jd: 2453310.5, delta_t: 64.6543 },
        DeltaTValue { jd: 2453340.5, delta_t: 64.6723 },
        DeltaTValue { jd: 2453371.5, delta_t: 64.6876 },
        DeltaTValue { jd: 2453402.5, delta_t: 64.7052 },
        DeltaTValue { jd: 2453430.5, delta_t: 64.7313 },
        DeltaTValue { jd: 2453461.5, delta_t: 64.7575 },
        DeltaTValue { jd: 2453491.5, delta_t: 64.7811 },
        DeltaTValue { jd: 2453522.5, delta_t: 64.8001 },
        DeltaTValue { jd: 2453552.5, delta_t: 64.7995 },
        DeltaTValue { jd: 2453583.5, delta_t: 64.7876 },
        DeltaTValue { jd: 2453614.5, delta_t: 64.7831 },
        DeltaTValue { jd: 2453644.5, delta_t: 64.7921 },
        DeltaTValue { jd: 2453675.5, delta_t: 64.8096 },
        DeltaTValue { jd: 2453705.5, delta_t: 64.8311 },
        DeltaTValue { jd: 2453736.5, delta_t: 64.8452 },
        DeltaTValue { jd: 2453767.5, delta_t: 64.8597 },
        DeltaTValue { jd: 2453795.5, delta_t: 64.8850 },
        DeltaTValue { jd: 2453826.5, delta_t: 64.9175 },
        DeltaTValue { jd: 2453856.5, delta_t: 64.9480 },
        DeltaTValue { jd: 2453887.5, delta_t: 64.9794 },
        DeltaTValue { jd: 2453917.5, delta_t: 64.9895 },
        DeltaTValue { jd: 2453948.5, delta_t: 65.0028 },
        DeltaTValue { jd: 2453979.5, delta_t: 65.0138 },
        DeltaTValue { jd: 2454009.5, delta_t: 65.0371 },
        DeltaTValue { jd: 2454040.5, delta_t: 65.0773 },
        DeltaTValue { jd: 2454070.5, delta_t: 65.1122 },
        DeltaTValue { jd: 2454101.5, delta_t: 65.1464 },
        DeltaTValue { jd: 2454132.5, delta_t: 65.1833 },
        DeltaTValue { jd: 2454160.5, delta_t: 65.2145 },
        DeltaTValue { jd: 2454191.5, delta_t: 65.2494 },
        DeltaTValue { jd: 2454221.5, delta_t: 65.2921 },
        DeltaTValue { jd: 2454252.5, delta_t: 65.3279 },
        DeltaTValue { jd: 2454282.5, delta_t: 65.3413 },
        DeltaTValue { jd: 2454313.5, delta_t: 65.3452 },
        DeltaTValue { jd: 2454344.5, delta_t: 65.3496 },
        DeltaTValue { jd: 2454374.5, delta_t: 65.3711 },
        DeltaTValue { jd: 2454405.5, delta_t: 65.3972 },
        DeltaTValue { jd: 2454435.5, delta_t: 65.4296 },
        DeltaTValue { jd: 2454466.5, delta_t: 65.4573 },
        DeltaTValue { jd: 2454497.5, delta_t: 65.4868 },
        DeltaTValue { jd: 2454526.5, delta_t: 65.5152 },
        DeltaTValue { jd: 2454557.5, delta_t: 65.5450 },
        DeltaTValue { jd: 2454587.5, delta_t: 65.5781 },
        DeltaTValue { jd: 2454618.5, delta_t: 65.6127 },
        DeltaTValue { jd: 2454648.5, delta_t: 65.6288 },
        DeltaTValue { jd: 2454679.5, delta_t: 65.6370 },
        DeltaTValue { jd: 2454710.5, delta_t: 65.6493 },
        DeltaTValue { jd: 2454740.5, delta_t: 65.6760 },
        DeltaTValue { jd: 2454771.5, delta_t: 65.7097 },
        DeltaTValue { jd: 2454801.5, delta_t: 65.7461 },
        DeltaTValue { jd: 2454832.5, delta_t: 65.7768 },
        DeltaTValue { jd: 2454863.5, delta_t: 65.8025 },
        DeltaTValue { jd: 2454891.5, delta_t: 65.8237 },
        DeltaTValue { jd: 2454922.5, delta_t: 65.8595 },
        DeltaTValue { jd: 2454952.5, delta_t: 65.8973 },
        DeltaTValue { jd: 2454983.5, delta_t: 65.9323 },
        DeltaTValue { jd: 2455013.5, delta_t: 65.9509 },
        DeltaTValue { jd: 2455044.5, delta_t: 65.9534 },
        DeltaTValue { jd: 2455075.5, delta_t: 65.9628 },
        DeltaTValue { jd: 2455105.5, delta_t: 65.9839 },
        DeltaTValue { jd: 2455136.5, delta_t: 66.0147 },
        DeltaTValue { jd: 2455166.5, delta_t: 66.0420 },
        DeltaTValue { jd: 2455197.5, delta_t: 66.0699 },
        DeltaTValue { jd: 2455228.5, delta_t: 66.0961 },
        DeltaTValue { jd: 2455256.5, delta_t: 66.1310 },
        DeltaTValue { jd: 2455287.5, delta_t: 66.1683 },
        DeltaTValue { jd: 2455317.5, delta_t: 66.2072 },
        DeltaTValue { jd: 2455348.5, delta_t: 66.2356 },
        DeltaTValue { jd: 2455378.5, delta_t: 66.2409 },
        DeltaTValue { jd: 2455409.5, delta_t: 66.2335 },
        DeltaTValue { jd: 2455440.5, delta_t: 66.2349 },
        DeltaTValue { jd: 2455470.5, delta_t: 66.2441 },
        DeltaTValue { jd: 2455501.5, delta_t: 66.2751 },
        DeltaTValue { jd: 2455531.5, delta_t: 66.3054 },
        DeltaTValue { jd: 2455562.5, delta_t: 66.3246 },
        DeltaTValue { jd: 2455593.5, delta_t: 66.3406 },
        DeltaTValue { jd: 2455621.5, delta_t: 66.3624 },
        DeltaTValue { jd: 2455652.5, delta_t: 66.3957 },
        DeltaTValue { jd: 2455682.5, delta_t: 66.4289 },
        DeltaTValue { jd: 2455713.5, delta_t: 66.4619 },
        DeltaTValue { jd: 2455743.5, delta_t: 66.4749 },
        DeltaTValue { jd: 2455774.5, delta_t: 66.4751 },
        DeltaTValue { jd: 2455805.5, delta_t: 66.4829 },
        DeltaTValue { jd: 2455835.5, delta_t: 66.5056 },
        DeltaTValue { jd: 2455866.5, delta_t: 66.5383 },
        DeltaTValue { jd: 2455896.5, delta_t: 66.5706 },
        DeltaTValue { jd: 2455927.5, delta_t: 66.6030 },
        DeltaTValue { jd: 2455958.5, delta_t: 66.6340 },
        DeltaTValue { jd: 2455987.5, delta_t: 66.6569 },
        DeltaTValue { jd: 2456018.5, delta_t: 66.6925 }, //1 April 2012
        DeltaTValue { jd: 2456048.5, delta_t: 66.7289 },
        DeltaTValue { jd: 2456079.5, delta_t: 66.7579 },
        DeltaTValue { jd: 2456109.5, delta_t: 66.7708 },
        DeltaTValue { jd: 2456140.5, delta_t: 66.7740 },
        DeltaTValue { jd: 2456171.5, delta_t: 66.7846 },
        DeltaTValue { jd: 2456201.5, delta_t: 66.8103 },
        DeltaTValue { jd: 2456232.5, delta_t: 66.8400 },
        DeltaTValue { jd: 2456262.5, delta_t: 66.8779 },
        DeltaTValue { jd: 2456293.5, delta_t: 66.9069 }, //1 January 2013
        DeltaTValue { jd: 2456324.5, delta_t: 66.9443 }, //1 Februrary 2013
        DeltaTValue { jd: 2456352.5, delta_t: 66.9763 }, //1 March 2013
        DeltaTValue { jd: 2456383.5, delta_t: 67.0258 }, //1 April 2013
        DeltaTValue { jd: 2456413.5, delta_t: 67.0716 }, //1 May 2013
        DeltaTValue { jd: 2456444.5, delta_t: 67.1100 }, //1 June 2013
        DeltaTValue { jd: 2456474.5, delta_t: 67.1266 }, //1 July 2013
        DeltaTValue { jd: 2456505.5, delta_t: 67.1331 }, //1 August 2013
        DeltaTValue { jd: 2456536.5, delta_t: 67.1458 }, //1 September 2013
        DeltaTValue { jd: 2456566.5, delta_t: 67.1717 }, //1 October 2013
        DeltaTValue { jd: 2456597.5, delta_t: 67.2091 }, //1 November 2013
        DeltaTValue { jd: 2456627.5, delta_t: 67.2460 }, //1 December 2013
        DeltaTValue { jd: 2456658.5, delta_t: 67.2810 }, //1 January 2014
        DeltaTValue { jd: 2456689.5, delta_t: 67.3136 }, //1 February 2014
        DeltaTValue { jd: 2456717.5, delta_t: 67.3457 }, //1 March 2014
        DeltaTValue { jd: 2456748.5, delta_t: 67.3890 }, //1 April 2014
        DeltaTValue { jd: 2456778.5, delta_t: 67.4318 }, //1 May 2014
        DeltaTValue { jd: 2456809.5, delta_t: 67.4666 }, //1 June 2014
        DeltaTValue { jd: 2456839.5, delta_t: 67.4858 }, //1 July 2014
        DeltaTValue { jd: 2456870.5, delta_t: 67.4989 }, //1 August 2014
        DeltaTValue { jd: 2456901.5, delta_t: 67.5111 }, //1 September 2014
        DeltaTValue { jd: 2456931.5, delta_t: 67.5353 }, //1 October 2014
        DeltaTValue { jd: 2456962.5, delta_t: 67.5711 }, //1 November 2014
        DeltaTValue { jd: 2456992.5, delta_t: 67.6070 }, //1 December 2014
        DeltaTValue { jd: 2457023.5, delta_t: 67.6439 }, //1 January 2015
        DeltaTValue { jd: 2457054.5, delta_t: 67.6765 }, //1 February 2015
        DeltaTValue { jd: 2457082.5, delta_t: 67.7117 }, //1 March 2015
        DeltaTValue { jd: 2457113.5, delta_t: 67.7591 }, //1 April 2015
        DeltaTValue { jd: 2457143.5, delta_t: 67.8012 }, //1 May 2015
        DeltaTValue { jd: 2457174.5, delta_t: 67.8402 }, //1 June 2015
        DeltaTValue { jd: 2457204.5, delta_t: 67.8606 }, //1 July 2015
        DeltaTValue { jd: 2457235.5, delta_t: 67.8822 }, //1 August 2015
        DeltaTValue { jd: 2457266.5, delta_t: 67.9120 }, //1 September 2015
        DeltaTValue { jd: 2457296.5, delta_t: 67.9546 }, //1 October 2015
        DeltaTValue { jd: 2457327.5, delta_t: 68.0055 }, //1 November 2015
        DeltaTValue { jd: 2457357.5, delta_t: 68.0514 }, //1 December 2015
        DeltaTValue { jd: 2457388.5, delta_t: 68.1024 }, //1 January 2016
        DeltaTValue { jd: 2457419.5, delta_t: 68.1577 }, //1 February 2016
        DeltaTValue { jd: 2457448.5, delta_t: 68.2044 }, //1 March 2016
        DeltaTValue { jd: 2457479.5, delta_t: 68.2665 }, //1 April 2016
        DeltaTValue { jd: 2457509.5, delta_t: 68.3188 }, //1 May 2016
        DeltaTValue { jd: 2457540.5, delta_t: 68.3703 }, //1 June 2016
        DeltaTValue { jd: 2457570.5, delta_t: 68.3964 }, //1 July 2016
        DeltaTValue { jd: 2457601.5, delta_t: 68.4094 }, //1 August 2016
        DeltaTValue { jd: 2457632.5, delta_t: 68.4305 }, //1 September 2016
        DeltaTValue { jd: 2457662.5, delta_t: 68.4630 }, //1 October 2016
        DeltaTValue { jd: 2457693.5, delta_t: 68.5078 }, //1 November 2016
        DeltaTValue { jd: 2457723.5, delta_t: 68.5537 }, //1 December 2016
        DeltaTValue { jd: 2457754.5, delta_t: 68.5928 }, //1 January 2017
        DeltaTValue { jd: 2457785.5, delta_t: 68.6298 }, //1 February 2017
        DeltaTValue { jd: 2457813.5, delta_t: 68.6671 }, //1 March 2017
        DeltaTValue { jd: 2457844.5, delta_t: 68.7135 }, //1 April 2017
        DeltaTValue { jd: 2457874.5, delta_t: 68.7623 }, //1 May 2017
        DeltaTValue { jd: 2457905.5, delta_t: 68.8033 }, //1 June 2017
        DeltaTValue { jd: 2457935.5, delta_t: 68.8245 }, //1 July 2017
        DeltaTValue { jd: 2457966.5, delta_t: 68.8373 }, //1 August 2017
        DeltaTValue { jd: 2457997.5, delta_t: 68.8477 }, //1 September 2017
        DeltaTValue { jd: 2458027.5, delta_t: 68.8689 }, //1 October 2017
        DeltaTValue { jd: 2458058.5, delta_t: 68.9006 }, //1 November 2017
        DeltaTValue { jd: 2458088.5, delta_t: 68.9355 }, //1 December 2017
        DeltaTValue { jd: 2458119.5, delta_t: 68.9676 }, //1 January 2018
        DeltaTValue { jd: 2458150.5, delta_t: 68.9875 }, //1 February 2018
        DeltaTValue { jd: 2458178.5, delta_t: 69.0175 }, //1 March 2018
        DeltaTValue { jd: 2458209.5, delta_t: 69.0499 }, //1 April 2018

//All these final values are predicted values from Year 2018.5 to 2026.0 are taken from http://maia.usno.navy.mil/ser7/deltat.preds
        DeltaTValue { jd: 2458302.0, delta_t: 69.3 }, //2018.5
        DeltaTValue { jd: 2458484.5, delta_t: 69.5 }, //2019.0
        DeltaTValue { jd: 2458575.75, delta_t: 69.6 }, //2019.25
        DeltaTValue { jd: 2458667.0, delta_t: 69.7 }, //2019.5
        DeltaTValue { jd: 2458758.25, delta_t: 69.8 }, //2019.75
        DeltaTValue { jd: 2458849.5, delta_t: 69.9 }, //2020.0
        DeltaTValue { jd: 2458941.0, delta_t: 70.0 }, //2020.25
        DeltaTValue { jd: 2459763.0, delta_t: 71.0 }, //2022.5
        DeltaTValue { jd: 2461041.5, delta_t: 72.0 }, //2026.0

//Note as currently coded there is a single discontinuity of c. 2.074 seconds on 1 January 2026. At this point http://maia.usno.navy.mil/ser7/deltat.preds indicates an error value for delta_t of about 5 seconds anyway.
    ];

struct LeapSecondCoefficient {
    jd: f64,
    leap_seconds: f64,
    base_mjd: f64,
    coefficient: f64,
}

const G_LEAP_SECOND_COEFFICIENTS: [LeapSecondCoefficient; 41] = //Cumulative leap second values from 1 Jan 1961 to 1 January 2017 as taken from http://maia.usno.navy.mil/ser7/tai-utc.dat
    [
        LeapSecondCoefficient { jd: 2437300.5, leap_seconds: 1.4228180, base_mjd: 37300.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2437512.5, leap_seconds: 1.3728180, base_mjd: 37300.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2437665.5, leap_seconds: 1.8458580, base_mjd: 37665.0, coefficient: 0.0011232 },
        LeapSecondCoefficient { jd: 2438334.5, leap_seconds: 1.9458580, base_mjd: 37665.0, coefficient: 0.0011232 },
        LeapSecondCoefficient { jd: 2438395.5, leap_seconds: 3.2401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2438486.5, leap_seconds: 3.3401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2438639.5, leap_seconds: 3.4401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2438761.5, leap_seconds: 3.5401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2438820.5, leap_seconds: 3.6401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2438942.5, leap_seconds: 3.7401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2439004.5, leap_seconds: 3.8401300, base_mjd: 38761.0, coefficient: 0.001296 },
        LeapSecondCoefficient { jd: 2439126.5, leap_seconds: 4.3131700, base_mjd: 39126.0, coefficient: 0.002592 },
        LeapSecondCoefficient { jd: 2439887.5, leap_seconds: 4.2131700, base_mjd: 39126.0, coefficient: 0.002592 },
        LeapSecondCoefficient { jd: 2441317.5, leap_seconds: 10.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2441499.5, leap_seconds: 11.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2441683.5, leap_seconds: 12.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2442048.5, leap_seconds: 13.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2442413.5, leap_seconds: 14.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2442778.5, leap_seconds: 15.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2443144.5, leap_seconds: 16.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2443509.5, leap_seconds: 17.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2443874.5, leap_seconds: 18.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2444239.5, leap_seconds: 19.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2444786.5, leap_seconds: 20.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2445151.5, leap_seconds: 21.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2445516.5, leap_seconds: 22.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2446247.5, leap_seconds: 23.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2447161.5, leap_seconds: 24.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2447892.5, leap_seconds: 25.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2448257.5, leap_seconds: 26.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2448804.5, leap_seconds: 27.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2449169.5, leap_seconds: 28.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2449534.5, leap_seconds: 29.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2450083.5, leap_seconds: 30.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2450630.5, leap_seconds: 31.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2451179.5, leap_seconds: 32.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2453736.5, leap_seconds: 33.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2454832.5, leap_seconds: 34.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2456109.5, leap_seconds: 35.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2457204.5, leap_seconds: 36.0, base_mjd: 41317.0, coefficient: 0.0 },
        LeapSecondCoefficient { jd: 2457754.5, leap_seconds: 37.0, base_mjd: 41317.0, coefficient: 0.0 }
    ];

pub fn delta_t(jd: f64) -> f64
{
    //What will be the return value from the method
    let mut delta = 0.0_f64;

    //Determine if we can use the lookup table
    let n_lookup_elements = G_DELTA_T_VALUES.len();
    if (jd >= G_DELTA_T_VALUES[0].jd) && (jd < G_DELTA_T_VALUES[n_lookup_elements - 1].jd) {
        //Find the index in the lookup table which contains the jd value closest to the jd input parameter
        let mut b_found = false;
        let mut n_found_index = 0;
        while !b_found {
            assert!(n_found_index < n_lookup_elements);
            b_found = G_DELTA_T_VALUES[n_found_index].jd > jd;

            //Prepare for the next loop
            if !b_found {
                n_found_index += 1;
            } else {
                //Now do a simple linear interpolation of the delta_t values from the lookup table
                delta = (jd - G_DELTA_T_VALUES[n_found_index - 1].jd) / (G_DELTA_T_VALUES[n_found_index].jd - G_DELTA_T_VALUES[n_found_index - 1].jd) * (G_DELTA_T_VALUES[n_found_index].delta_t - G_DELTA_T_VALUES[n_found_index - 1].delta_t) + G_DELTA_T_VALUES[n_found_index - 1].delta_t;
            }
        }
    } else {
        let date = aadate::AADate::from_jd(jd, aadate::after_papal_reform_jd(jd));
        let y = date.fractional_year();

        //Use the polynomial expressions from Espenak & Meeus 2006. References: http://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html and
        //http://www.staff.science.uu.nl/~gent0113/deltat/deltat_old.htm (Espenak & Meeus 2006 section)
        if y < -500.0 {
            let u = (y - 1820.0) / 100.0;
            let u2 = u * u;
            delta = -20.0 + 32.0 * u2;
        } else if y < 500.0 {
            let u = y / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            delta = 10583.6 + (-1014.41 * u) + (33.78311 * u2) + (-5.952053 * u3) + (-0.1798452 * u4) + (0.022174192 * u5) + (0.0090316521 * u6);
        } else if y < 1600.0 {
            let u = (y - 1000.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            delta = 1574.2 + (-556.01 * u) + (71.23472 * u2) + (0.319781 * u3) + (-0.8503463 * u4) + (-0.005050998 * u5) + (0.0083572073 * u6);
        } else if y < 1700.0 {
            let u = (y - 1600.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta = 120.0 + (-98.08 * u) + (-153.2 * u2) + (u3 / 0.007129);
        } else if y < 1800.0 {
            let u = (y - 1700.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            delta = 8.83 + (16.03 * u) + (-59.285 * u2) + (133.36 * u3) + (-u4 / 0.01174);
        } else if y < 1860.0 {
            let u = (y - 1800.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            let u6 = u5 * u;
            let u7 = u6 * u;
            delta = 13.72 + (-33.2447 * u) + (68.612 * u2) + (4111.6 * u3) + (-37436.0 * u4) + (121272.0 * u5) + (-169900.0 * u6) + (87500.0 * u7);
        } else if y < 1900.0 {
            let u = (y - 1860.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            delta = 7.62 + (57.37 * u) + (-2517.54 * u2) + (16806.68 * u3) + (-44736.24 * u4) + (u5 / 0.0000233174);
        } else if y < 1920.0 {
            let u = (y - 1900.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            delta = -2.79 + (149.4119 * u) + (-598.939 * u2) + (6196.6 * u3) + (-19700.0 * u4);
        } else if y < 1941.0 {
            let u = (y - 1920.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta = 21.20 + (84.493 * u) + (-761.00 * u2) + (2093.6 * u3);
        } else if y < 1961.0 {
            let u = (y - 1950.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta = 29.07 + (40.7 * u) + (-u2 / 0.0233) + (u3 / 0.002547);
        } else if y < 1986.0 {
            let u = (y - 1975.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            delta = 45.45 + 106.7 * u - u2 / 0.026 - u3 / 0.000718;
        } else if y < 2005.0 {
            let u = (y - 2000.0) / 100.0;
            let u2 = u * u;
            let u3 = u2 * u;
            let u4 = u3 * u;
            let u5 = u4 * u;
            delta = 63.86 + (33.45 * u) + (-603.74 * u2) + (1727.5 * u3) + (65181.4 * u4) + (237359.9 * u5);
        } else if y < 2050.0 {
            let u = (y - 2000.0) / 100.0;
            let u2 = u * u;
            delta = 62.92 + (32.217 * u) + (55.89 * u2);
        } else if y < 2150.0 {
            let u = (y - 1820.0) / 100.0;
            let u2 = u * u;
            delta = -205.72 + (56.28 * u) + (32.0 * u2);
        } else {
            let u = (y - 1820.0) / 100.0;
            let u2 = u * u;
            delta = -20.0 + (32.0 * u2);
        }
    }
    delta
}

pub fn cumulative_leap_seconds(jd: f64) -> f64
{
    //What will be the return value from the method
    let mut leap_seconds = 0.0_f64;

    let n_lookup_elements = G_LEAP_SECOND_COEFFICIENTS.len();
    if jd >= G_LEAP_SECOND_COEFFICIENTS[0].jd {
        //Find the index in the lookup table which contains the jd value closest to the jd input parameter
        let mut b_continue = true;
        let mut n_index = 1;
        while b_continue {
            if n_index >= n_lookup_elements {
                leap_seconds = G_LEAP_SECOND_COEFFICIENTS[n_lookup_elements - 1].leap_seconds + (jd - 2400000.5 - G_LEAP_SECOND_COEFFICIENTS[n_lookup_elements - 1].base_mjd) * G_LEAP_SECOND_COEFFICIENTS[n_lookup_elements - 1].coefficient;
                b_continue = false;
            } else if jd < G_LEAP_SECOND_COEFFICIENTS[n_index].jd {
                leap_seconds = G_LEAP_SECOND_COEFFICIENTS[n_index - 1].leap_seconds + (jd - 2400000.5 - G_LEAP_SECOND_COEFFICIENTS[n_index - 1].base_mjd) * G_LEAP_SECOND_COEFFICIENTS[n_index - 1].coefficient;
                b_continue = false;
            }

            //Prepare for the next loop
            if b_continue {
                n_index += 1;
            }
        }
    }
    leap_seconds
}

pub fn tt2utc(jd: f64) -> f64
{
    //Outside of the range 1 January 1961 to 500 days after the last leap second,
    //we implement tt2utc as tt2ut1
    let n_lookup_elements = G_LEAP_SECOND_COEFFICIENTS.len();
    if (jd < G_LEAP_SECOND_COEFFICIENTS[0].jd) || (jd > (G_LEAP_SECOND_COEFFICIENTS[n_lookup_elements - 1].jd + 500.0)) {
        tt2ut1(jd)
    } else {
        let dt = delta_t(jd);
        let ut1 = jd - (dt / 86400.0);
        let leap_seconds = cumulative_leap_seconds(jd);
        ((dt - leap_seconds - 32.184) / 86400.0) + ut1
    }
}

pub fn utc2tt(jd: f64) -> f64
{
    //Outside of the range 1 January 1961 to 500 days after the last leap second,
    //we implement tt2utc as tt2ut1
    let n_lookup_elements = G_LEAP_SECOND_COEFFICIENTS.len();
    if (jd < G_LEAP_SECOND_COEFFICIENTS[0].jd) || (jd > (G_LEAP_SECOND_COEFFICIENTS[n_lookup_elements - 1].jd + 500.0)) {
        ut12tt(jd)
    } else {
        let dt = delta_t(jd);
        let leap_seconds = cumulative_leap_seconds(jd);
        let ut1 = jd - ((dt - leap_seconds - 32.184) / 86400.0);
        ut1 + (dt / 86400.0)
    }
}

pub fn tt2ut1(jd: f64) -> f64
{
    jd - (delta_t(jd) / 86400.0)
}

pub fn ut12tt(jd: f64) -> f64
{
    jd + (delta_t(jd) / 86400.0)
}

pub fn ut1_minus_utc(jd: f64) -> f64
{
    let jd_utc = jd + ((delta_t(jd) - cumulative_leap_seconds(jd) - 32.184) / 86400.0);
    (jd - jd_utc) * 86400.0
}
