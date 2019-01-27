use super::aacoordinatetransformation;

struct PlutoCoefficient1
{
  j:i64,
  s:i64,
  p:i64,
}

struct PlutoCoefficient2
{
  a:f64,
  b:f64,
}

const G_PLUTO_ARGUMENT_COEFFICIENTS:[PlutoCoefficient1;43] =
[ 
  PlutoCoefficient1{ j: 0, s:   0, p:    1 },
  PlutoCoefficient1{ j: 0, s:   0, p:    2 },
  PlutoCoefficient1{ j: 0, s:   0, p:    3 },
  PlutoCoefficient1{ j: 0, s:   0, p:    4 },
  PlutoCoefficient1{ j: 0, s:   0, p:    5 },
  PlutoCoefficient1{ j: 0, s:   0, p:    6 },
  PlutoCoefficient1{ j: 0, s:   1, p:   -1 },
  PlutoCoefficient1{ j: 0, s:   1, p:    0 },
  PlutoCoefficient1{ j: 0, s:   1, p:    1 },
  PlutoCoefficient1{ j: 0, s:   1, p:    2 },
  PlutoCoefficient1{ j: 0, s:   1, p:    3 },
  PlutoCoefficient1{ j: 0, s:   2, p:   -2 },
  PlutoCoefficient1{ j: 0, s:   2, p:   -1 },
  PlutoCoefficient1{ j: 0, s:   2, p:    0 },
  PlutoCoefficient1{ j: 1, s:  -1, p:    0 },
  PlutoCoefficient1{ j: 1, s:  -1, p:    1 },
  PlutoCoefficient1{ j: 1, s:   0, p:   -3 },
  PlutoCoefficient1{ j: 1, s:   0, p:   -2 },
  PlutoCoefficient1{ j: 1, s:   0, p:   -1 },
  PlutoCoefficient1{ j: 1, s:   0, p:    0 },
  PlutoCoefficient1{ j: 1, s:   0, p:    1 },
  PlutoCoefficient1{ j: 1, s:   0, p:    2 },
  PlutoCoefficient1{ j: 1, s:   0, p:    3 },
  PlutoCoefficient1{ j: 1, s:   0, p:    4 },
  PlutoCoefficient1{ j: 1, s:   1, p:   -3 },
  PlutoCoefficient1{ j: 1, s:   1, p:   -2 },
  PlutoCoefficient1{ j: 1, s:   1, p:   -1 },
  PlutoCoefficient1{ j: 1, s:   1, p:    0 },
  PlutoCoefficient1{ j: 1, s:   1, p:    1 },
  PlutoCoefficient1{ j: 1, s:   1, p:    3 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -6 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -5 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -4 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -3 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -2 },
  PlutoCoefficient1{ j: 2, s:   0, p:   -1 },
  PlutoCoefficient1{ j: 2, s:   0, p:    0 },
  PlutoCoefficient1{ j: 2, s:   0, p:    1 },
  PlutoCoefficient1{ j: 2, s:   0, p:    2 },
  PlutoCoefficient1{ j: 2, s:   0, p:    3 },
  PlutoCoefficient1{ j: 3, s:   0, p:   -2 },
  PlutoCoefficient1{ j: 3, s:   0, p:   -1 },
  PlutoCoefficient1{ j: 3, s:   0, p:    0 }
];

const G_PLUTO_LONGITUDE_COEFFICIENTS:[PlutoCoefficient2;43] =
[ 
  PlutoCoefficient2{ a: -19799805.0, b: 19850055.0 },
  PlutoCoefficient2{ a:  897144.0,  b:-4954829.0  },
  PlutoCoefficient2{ a:  611149.0,  b: 1211027.0  },
  PlutoCoefficient2{ a: -341243.0,  b:-189585.0   },
  PlutoCoefficient2{ a:  129287.0,  b:-34992.0    },
  PlutoCoefficient2{ a: -38164.0,   b: 30893.0    },
  PlutoCoefficient2{ a:  20442.0,   b:-9987.0     },
  PlutoCoefficient2{ a: -4063.0,    b:-5071.0     },
  PlutoCoefficient2{ a: -6016.0,    b:-3336.0     },
  PlutoCoefficient2{ a: -3956.0,    b: 3039.0     },
  PlutoCoefficient2{ a: -667.0,     b: 3572.0     },
  PlutoCoefficient2{ a:  1276.0,    b: 501.0      },
  PlutoCoefficient2{ a:  1152.0,    b:-917.0      },
  PlutoCoefficient2{ a:  630.0,     b:-1277.0     },
  PlutoCoefficient2{ a:  2571.0,    b:-459.0      },
  PlutoCoefficient2{ a:  899.0,     b:-1449.0     },
  PlutoCoefficient2{ a: -1016.0,    b: 1043.0     },
  PlutoCoefficient2{ a: -2343.0,    b:-1012.0     },
  PlutoCoefficient2{ a:  7042.0,    b: 788.0      },
  PlutoCoefficient2{ a:  1199.0,    b:-338.0      },
  PlutoCoefficient2{ a:  418.0,     b:-67.0       },
  PlutoCoefficient2{ a:  120.0,     b:-274.0      },
  PlutoCoefficient2{ a: -60.0,      b:-159.0      },
  PlutoCoefficient2{ a: -82.0,      b:-29.0       },
  PlutoCoefficient2{ a: -36.0,      b:-29.0       },
  PlutoCoefficient2{ a: -40.0,      b: 7.0        },
  PlutoCoefficient2{ a: -14.0,      b: 22.0       },
  PlutoCoefficient2{ a:  4.0,       b: 13.0       },
  PlutoCoefficient2{ a:  5.0,       b: 2.0        },
  PlutoCoefficient2{ a: -1.0,       b: 0.0        },
  PlutoCoefficient2{ a:  2.0,       b: 0.0        },
  PlutoCoefficient2{ a: -4.0,       b: 5.0        },
  PlutoCoefficient2{ a:  4.0,       b:-7.0        },
  PlutoCoefficient2{ a:  14.0,      b: 24.0       },
  PlutoCoefficient2{ a: -49.0,      b:-34.0       },
  PlutoCoefficient2{ a:  163.0,     b:-48.0       },
  PlutoCoefficient2{ a:  9.0,       b:-24.0       },
  PlutoCoefficient2{ a: -4.0,       b: 1.0        },
  PlutoCoefficient2{ a: -3.0,       b: 1.0        },
  PlutoCoefficient2{ a:  1.0,       b: 3.0        },
  PlutoCoefficient2{ a: -3.0,       b:-1.0        },
  PlutoCoefficient2{ a:  5.0,       b:-3.0        },
  PlutoCoefficient2{ a:  0.0,       b: 0.0        }
];

const G_PLUTO_LATITUDE_COEFFICIENTS:[PlutoCoefficient2;43] =
[ 
  PlutoCoefficient2{ a: -5452852.0, b: -14974862.0 },
  PlutoCoefficient2{ a:  3527812.0, b:  1672790.0  },
  PlutoCoefficient2{ a: -1050748.0, b:  327647.0   },
  PlutoCoefficient2{ a:  178690.0, b: -292153.0   },
  PlutoCoefficient2{ a:  18650.0,  b:  100340.0   },
  PlutoCoefficient2{ a: -30697.0,  b: -25823.0    },
  PlutoCoefficient2{ a:  4878.0,   b:  11248.0    },
  PlutoCoefficient2{ a:  226.0,    b: -64.0       },
  PlutoCoefficient2{ a:  2030.0,   b: -836.0      },
  PlutoCoefficient2{ a:  69.0,     b: -604.0      },
  PlutoCoefficient2{ a: -247.0,    b: -567.0      },
  PlutoCoefficient2{ a: -57.0,     b:  1.0        },
  PlutoCoefficient2{ a: -122.0,    b:  175.0      },
  PlutoCoefficient2{ a: -49.0,     b: -164.0      },
  PlutoCoefficient2{ a: -197.0,    b:  199.0      },
  PlutoCoefficient2{ a: -25.0,     b:  217.0      },
  PlutoCoefficient2{ a:  589.0,    b: -248.0      },
  PlutoCoefficient2{ a: -269.0,    b:  711.0      },
  PlutoCoefficient2{ a:  185.0,    b:  193.0      },
  PlutoCoefficient2{ a:  315.0,    b:  807.0      },
  PlutoCoefficient2{ a: -130.0,    b: -43.0       },
  PlutoCoefficient2{ a:  5.0,      b:  3.0        },
  PlutoCoefficient2{ a:  2.0,      b:  17.0       },
  PlutoCoefficient2{ a:  2.0,      b:  5.0        },
  PlutoCoefficient2{ a:  2.0,      b:  3.0        },
  PlutoCoefficient2{ a:  3.0,      b:  1.0        },
  PlutoCoefficient2{ a:  2.0,      b: -1.0        },
  PlutoCoefficient2{ a:  1.0,      b: -1.0        },
  PlutoCoefficient2{ a:  0.0,      b: -1.0        },
  PlutoCoefficient2{ a:  0.0,      b:  0.0        },
  PlutoCoefficient2{ a:  0.0,      b: -2.0        },
  PlutoCoefficient2{ a:  2.0,      b:  2.0        },
  PlutoCoefficient2{ a: -7.0,      b:  0.0        },
  PlutoCoefficient2{ a:  10.0,     b: -8.0        },
  PlutoCoefficient2{ a: -3.0,      b:  20.0       },
  PlutoCoefficient2{ a:  6.0,      b:  5.0        },
  PlutoCoefficient2{ a:  14.0,     b:  17.0       },
  PlutoCoefficient2{ a: -2.0,      b:  0.0        },
  PlutoCoefficient2{ a:  0.0,      b:  0.0        },
  PlutoCoefficient2{ a:  0.0,      b:  0.0        },
  PlutoCoefficient2{ a:  0.0,      b:  1.0        },
  PlutoCoefficient2{ a:  0.0,      b:  0.0        },
  PlutoCoefficient2{ a:  1.0,      b:  0.0        }
];

const G_PLUTO_RADIUS_COEFFICIENTS:[PlutoCoefficient2;43] =
[ 
  PlutoCoefficient2{ a:  66865439.0, b:   68951812.0 },
  PlutoCoefficient2{ a: -11827535.0, b:  -332538.0   },
  PlutoCoefficient2{ a:  1593179.0, b:  -1438890.0  },
  PlutoCoefficient2{ a: -18444.0,   b:   483220.0   },
  PlutoCoefficient2{ a: -65977.0,   b:  -85431.0    },
  PlutoCoefficient2{ a:  31174.0,   b:  -6032.0     },
  PlutoCoefficient2{ a: -5794.0,    b:   22161.0    },
  PlutoCoefficient2{ a:  4601.0,    b:   4032.0     },
  PlutoCoefficient2{ a: -1729.0,    b:   234.0      },
  PlutoCoefficient2{ a: -415.0,     b:   702.0      },
  PlutoCoefficient2{ a:  239.0,     b:   723.0      },
  PlutoCoefficient2{ a:  67.0,      b:  -67.0       },
  PlutoCoefficient2{ a:  1034.0,    b:  -451.0      },
  PlutoCoefficient2{ a: -129.0,     b:   504.0      },
  PlutoCoefficient2{ a:  480.0,     b:  -231.0      },
  PlutoCoefficient2{ a:  2.0,       b:  -441.0      },
  PlutoCoefficient2{ a: -3359.0,    b:   265.0      },
  PlutoCoefficient2{ a:  7856.0,    b:  -7832.0     },
  PlutoCoefficient2{ a:  36.0,      b:   45763.0    },
  PlutoCoefficient2{ a:  8663.0,    b:   8547.0     },
  PlutoCoefficient2{ a: -809.0,     b:  -769.0      },
  PlutoCoefficient2{ a:  263.0,     b:  -144.0      },
  PlutoCoefficient2{ a: -126.0,     b:   32.0       },
  PlutoCoefficient2{ a: -35.0,      b:  -16.0       },
  PlutoCoefficient2{ a: -19.0,      b:  -4.0        },
  PlutoCoefficient2{ a: -15.0,      b:   8.0        },
  PlutoCoefficient2{ a: -4.0,       b:   12.0       },
  PlutoCoefficient2{ a:  5.0,       b:   6.0        },
  PlutoCoefficient2{ a:  3.0,       b:   1.0        },
  PlutoCoefficient2{ a:  6.0,       b:  -2.0        },
  PlutoCoefficient2{ a:  2.0,       b:   2.0        },
  PlutoCoefficient2{ a: -2.0,       b:  -2.0        },
  PlutoCoefficient2{ a:  14.0,      b:   13.0       },
  PlutoCoefficient2{ a: -63.0,      b:   13.0       },
  PlutoCoefficient2{ a:  136.0,     b:  -236.0      },
  PlutoCoefficient2{ a:  273.0,     b:   1065.0     },
  PlutoCoefficient2{ a:  251.0,     b:   149.0      },
  PlutoCoefficient2{ a: -25.0,      b:  -9.0        },
  PlutoCoefficient2{ a:  9.0,       b:  -2.0        },
  PlutoCoefficient2{ a: -8.0,       b:   7.0        },
  PlutoCoefficient2{ a:  2.0,       b:  -10.0       },
  PlutoCoefficient2{ a:  19.0,      b:   35.0       },
  PlutoCoefficient2{ a:  10.0,      b:   3.0        }
];


pub fn ecliptic_longitude(jd:f64) -> f64
{
  let t = (jd - 2451545.0) / 36525.0;
  let j = 34.35 + 3034.9057* t;
  let s = 50.08 + 1222.1138* t;
  let p = 238.96 + 144.9600* t;

  //Calculate Longitude
  let mut l = 0.0f64;
  let n_pluto_coefficients = G_PLUTO_ARGUMENT_COEFFICIENTS.len();
  for i in 0..n_pluto_coefficients {
    let mut alpha = G_PLUTO_ARGUMENT_COEFFICIENTS[i].j as f64 * j +  G_PLUTO_ARGUMENT_COEFFICIENTS[i].s as f64 * s + G_PLUTO_ARGUMENT_COEFFICIENTS[i].p as f64 * p;
    alpha = aacoordinatetransformation::degrees_to_radians(alpha);
    l += G_PLUTO_LONGITUDE_COEFFICIENTS[i].a * f64::sin(alpha) + G_PLUTO_LONGITUDE_COEFFICIENTS[i].b * f64::cos(alpha);
  }
  l = l / 1000000.0;
  l += 238.958116 + 144.96* t;
  aacoordinatetransformation::map_to_0to360_range(l)
}

pub fn ecliptic_latitude(jd:f64) -> f64
{
  let t = (jd - 2451545.0) / 36525.0;
  let j = 34.35 + 3034.9057* t;
  let s = 50.08 + 1222.1138* t;
  let p = 238.96 + 144.9600* t;

  //Calculate Latitude
  let mut l = 0.0f64;
  let n_pluto_coefficients = G_PLUTO_ARGUMENT_COEFFICIENTS.len();
  for i in 0..n_pluto_coefficients {
    let mut alpha = G_PLUTO_ARGUMENT_COEFFICIENTS[i].j as f64 * j +  G_PLUTO_ARGUMENT_COEFFICIENTS[i].s as f64 * s + G_PLUTO_ARGUMENT_COEFFICIENTS[i].p as f64 * p;
    alpha = aacoordinatetransformation::degrees_to_radians(alpha);
    l += G_PLUTO_LATITUDE_COEFFICIENTS[i].a * f64::sin(alpha) + G_PLUTO_LATITUDE_COEFFICIENTS[i].b * f64::cos(alpha);
  }
  l = l / 1000000.0;
  l += -3.908239;

  aacoordinatetransformation::map_to_minus90to90_range(l)
}

pub fn radius_vector(jd:f64) -> f64
{
  let t = (jd - 2451545.0) / 36525.0;
  let j = 34.35 + 3034.9057* t;
  let s = 50.08 + 1222.1138* t;
  let p = 238.96 + 144.9600* t;

  //Calculate Radius
  let mut r = 0.0f64;
  let n_pluto_coefficients = G_PLUTO_ARGUMENT_COEFFICIENTS.len();
  for i in 0..n_pluto_coefficients {
    let mut alpha = G_PLUTO_ARGUMENT_COEFFICIENTS[i].j as f64 * j +  G_PLUTO_ARGUMENT_COEFFICIENTS[i].s as f64 * s + G_PLUTO_ARGUMENT_COEFFICIENTS[i].p as f64 * p;
    alpha = aacoordinatetransformation::degrees_to_radians(alpha);
    r += G_PLUTO_RADIUS_COEFFICIENTS[i].a * f64::sin(alpha) + G_PLUTO_RADIUS_COEFFICIENTS[i].b * f64::cos(alpha);
  }
  r = r / 10000000.0;
  r + 40.7241346
}
