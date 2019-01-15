use super::aacoordinatetransformation;

pub fn mercury_semimajor_axis(_jd: f64) -> f64
{
    0.387098310
}

pub fn mercury_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.20563175 + 0.000020407 * t - 0.0000000283 * tsquared - 0.00000000018 * tcubed
}

pub fn venus_semimajor_axis(_jd: f64) -> f64
{
    0.723329820
}

pub fn venus_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.00677192 - 0.000047765 * t + 0.0000000981 * tsquared + 0.00000000046 * tcubed
}

pub fn earth_semimajor_axis(_jd: f64) -> f64
{
    1.000001018
}

pub fn earth_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.01670863 - 0.000042037 * t - 0.0000001267 * tsquared + 0.00000000014 * tcubed
}

pub fn earth_inclination(_jd: f64) -> f64
{
    0.0
}

pub fn mars_semimajor_axis(_jd: f64) -> f64
{
    1.523679342
}

pub fn mars_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.09340065 + 0.000090484 * t - 0.0000000806 * tsquared - 0.00000000025 * tcubed
}

pub fn jupiter_semimajor_axis(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;

    5.202603209 + 0.0000001913 * t
}

pub fn jupiter_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.04849793 + 0.000163225 * t - 0.0000004714 * tsquared - 0.00000000201 * tcubed
}

pub fn saturn_semimajor_axis(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;

    9.554909192 - 0.0000021390 * t + 0.000000004 * tsquared
}

pub fn saturn_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.05554814 - 0.0003446641 * t - 0.0000006436 * tsquared + 0.00000000340 * tcubed
}

pub fn uranus_semimajor_axis(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;

    19.218446062 - 0.0000000372 * t + 0.00000000098 * tsquared
}

pub fn uranus_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.04638122 - 0.000027293 * t + 0.0000000789 * tsquared + 0.00000000024 * tcubed
}

pub fn neptune_semimajor_axis(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;

    30.110386869 - 0.0000001663 * t + 0.00000000069 * tsquared
}

pub fn neptune_eccentricity(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tcubed = t * t * t;

    0.00945575 + 0.000006033 * t - 0.00000000005 * tcubed
}

pub fn earth_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    0.0130548 * t - 0.00000931 * tsquared - 0.000000034 * tcubed
}

pub fn mercury_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(252.250906 + 149474.0722491 * t + 0.00030350 * tsquared + 0.000000018 * tcubed)
}

pub fn mercury_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(7.004986 + 0.0018215 * t - 0.00001810 * tsquared + 0.000000056 * tcubed)
}

pub fn mercury_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(48.330893 + 1.1861883 * t + 0.00017542 * tsquared + 0.000000215 * tcubed)
}

pub fn mercury_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(77.456119 + 1.5564776 * t + 0.00029544 * tsquared + 0.000000009 * tcubed)
}

pub fn venus_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(181.979801 + 58519.2130302 * t + 0.00031014 * tsquared + 0.000000015 * tcubed)
}

pub fn venus_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(3.394662 + 0.0010037 * t - 0.00000088 * tsquared - 0.000000007 * tcubed)
}

pub fn venus_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(76.679920 + 0.9011206 * t + 0.00040618 * tsquared - 0.000000093 * tcubed)
}

pub fn venus_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(131.563703 + 1.4022288 * t - 0.00107618 * tsquared - 0.000005678 * tcubed)
}

pub fn earth_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(100.466457 + 36000.7698278 * t + 0.00030322 * tsquared + 0.000000020 * tcubed)
}


pub fn earth_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(102.937348 + 1.17195366 * t + 0.00045688 * tsquared - 0.000000018 * tcubed)
}

pub fn mars_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(355.433000 + 19141.6964471 * t + 0.00031052 * tsquared + 0.000000016 * tcubed)
}

pub fn mars_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(1.849726 - 0.0006011 * t + 0.00001276 * tsquared - 0.000000007 * tcubed)
}

pub fn mars_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(49.588093 + 0.7720959 * t + 0.00001557 * tsquared + 0.000002267 * tcubed)
}

pub fn mars_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(336.060234 + 1.8410449 * t + 0.00013477 * tsquared + 0.000000536 * tcubed)
}

pub fn jupiter_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(34.351519 + 3036.3027748 * t + 0.00022330 * tsquared + 0.000000037 * tcubed)
}

pub fn jupiter_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(1.303267 - 0.0054965 * t + 0.00000466 * tsquared - 0.000000002 * tcubed)
}

pub fn jupiter_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(100.464407 + 1.0209774 * t + 0.00040315 * tsquared + 0.000000404 * tcubed)
}

pub fn jupiter_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(14.331207 + 1.6126352 * t + 0.00103042 * tsquared - 0.000004464 * tcubed)
}

pub fn saturn_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(50.077444 + 1223.5110686 * t + 0.00051908 * tsquared - 0.000000030 * tcubed)
}

pub fn saturn_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(2.488879 - 0.0037362 * t - 0.00001519 * tsquared + 0.000000087 * tcubed)
}

pub fn saturn_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(113.665503 + 0.8770880 * t - 0.00012176 * tsquared - 0.000002249 * tcubed)
}

pub fn saturn_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(93.057237 + 1.9637613 * t + 0.00083753 * tsquared + 0.000004928 * tcubed)
}

pub fn uranus_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(314.055005 + 429.8640561 * t + 0.00030390 * tsquared + 0.000000026 * tcubed)
}

pub fn uranus_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(0.773197 + 0.0007744 * t + 0.00003749 * tsquared - 0.000000092 * tcubed)
}

pub fn uranus_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(74.005957 + 0.5211278 * t + 0.00133947 * tsquared + 0.000018484 * tcubed)
}

pub fn uranus_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(173.005291 + 1.4863790 * t + 0.00021406 * tsquared + 0.000000434 * tcubed)
}

pub fn neptune_mean_longitude(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(304.348665 + 219.8833092 * t + 0.00030882 * tsquared + 0.000000018 * tcubed)
}

pub fn neptune_inclination(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(1.769953 - 0.0093082 * t - 0.00000708 * tsquared + 0.000000027 * tcubed)
}

pub fn neptune_longitude_ascending_node(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(131.784057 + 1.1022039 * t + 0.00025952 * tsquared - 0.000000637 * tcubed)
}

pub fn neptune_longitude_perihelion(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(48.120276 + 1.4262957 * t + 0.00038434 * tsquared + 0.000000020 * tcubed)
}

pub fn mercury_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(252.250906 + 149472.6746358 * t - 0.00000536 * tsquared + 0.000000002 * tcubed)
}

pub fn mercury_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(7.004986 - 0.0059516 * t + 0.00000080 * tsquared + 0.000000043 * tcubed)
}

pub fn mercury_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(48.330893 - 0.1254227 * t - 0.00008833 * tsquared - 0.000000200 * tcubed)
}

pub fn mercury_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(77.456119 + 0.1588643 * t - 0.00001342 * tsquared - 0.000000007 * tcubed)
}

pub fn venus_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(181.979801 + 58517.8156760 * t + 0.00000165 * tsquared - 0.000000002 * tcubed)
}

pub fn venus_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(3.394662 - 0.0008568 * t - 0.00003244 * tsquared + 0.000000009 * tcubed)
}

pub fn venus_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(76.679920 - 0.2780134 * t - 0.00014257 * tsquared - 0.000000164 * tcubed)
}

pub fn venus_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(131.563703 + 0.0048746 * t - 0.00138467 * tsquared - 0.000005695 * tcubed)
}

pub fn earth_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(100.466457 + 35999.3728565 * t - 0.00000568 * tsquared - 0.000000001 * tcubed)
}

pub fn earth_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(174.873176 - 0.241098 * t + 0.00004262 * tsquared + 0.000000001 * tcubed)
}

pub fn earth_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(102.937348 + 0.3225654 * t + 0.00014799 * tsquared - 0.000000039 * tcubed)
}

pub fn mars_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(355.433000 + 19140.2993039 * t + 0.00000262 * tsquared - 0.000000003 * tcubed)
}

pub fn mars_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(1.849726 - 0.0081477 * t - 0.00002255 * tsquared - 0.000000029 * tcubed)
}

pub fn mars_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(49.588093 - 0.2950250 * t - 0.00064048 * tsquared - 0.000001964 * tcubed)
}

pub fn mars_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(336.060234 + 0.4439016 * t - 0.00017313 * tsquared + 0.000000518 * tcubed)
}

pub fn jupiter_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(34.351519 + 3034.9056606 * t - 0.00008501 * tsquared + 0.000000016 * tcubed)
}

pub fn jupiter_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(1.303267 - 0.0019877 * t + 0.00003320 * tsquared + 0.000000097 * tcubed)
}

pub fn jupiter_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(100.464407 + 0.1767232 * t + 0.00090700 * tsquared - 0.000007272 * tcubed)
}

pub fn jupiter_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(14.331207 + 0.2155209 * t + 0.00072211 * tsquared - 0.000004485 * tcubed)
}

pub fn saturn_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(50.077444 + 1222.1138488 * t + 0.00021004 * tsquared - 0.000000046 * tcubed)
}

pub fn saturn_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(2.488879 + 0.0025514 * t - 0.00004906 * tsquared + 0.000000017 * tcubed)
}

pub fn saturn_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(113.665503 - 0.2566722 * t - 0.00018399 * tsquared + 0.000000480 * tcubed)
}

pub fn saturn_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(93.057237 + 0.5665415 * t + 0.00052850 * tsquared + 0.000004912 * tcubed)
}

pub fn uranus_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(314.055005 + 428.4669983 * t - 0.00000486 * tsquared + 0.000000006 * tcubed)
}

pub fn uranus_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(0.773197 - 0.0016869 * t + 0.00000349 * tsquared + 0.000000016 * tcubed)
}

pub fn uranus_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(74.005957 + 0.0741431 * t + 0.00040539 * tsquared + 0.000000119 * tcubed)
}

pub fn uranus_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(173.005291 + 0.0893212 * t - 0.00009470 * tsquared + 0.000000414 * tcubed)
}

pub fn neptune_mean_longitude_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(304.348665 + 218.4862002 * t + 0.00000059 * tsquared - 0.000000002 * tcubed)
}

pub fn neptune_inclination_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;

    aacoordinatetransformation::map_to_0to360_range(1.769953 + 0.0002256 * t + 0.00000023 * tsquared)
}

pub fn neptune_longitude_ascending_node_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;
    let tcubed = tsquared * t;

    aacoordinatetransformation::map_to_0to360_range(131.784057 - 0.0061651 * t - 0.00000219 * tsquared - 0.000000078 * tcubed)
}

pub fn neptune_longitude_perihelion_j2000(jd: f64) -> f64
{
    let t = (jd - 2451545.0) / 36525.0;
    let tsquared = t * t;

    aacoordinatetransformation::map_to_0to360_range(48.120276 + 0.0291866 * t + 0.00007610 * tsquared)
}

