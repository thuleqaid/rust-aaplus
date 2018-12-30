use super::aacoordinatetransformation;

pub struct VSOP87Coefficient
{
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct VSOP87Coefficient2<'a>
{
    pub p_coefficients: &'a [VSOP87Coefficient],
}

pub fn calculate(jd: f64, table: &[VSOP87Coefficient2], table_size: usize, b_angle: bool) -> f64
{
//Validate our parameters
    assert!(table.len() > 0);

    let t = (jd - 2451545.0) / 365250.0;
    let mut tterm = t;
    let mut result = 0.0;
    for i in 0..table_size {
        let mut temp_result = 0.0;
        for j in 0..table[i].p_coefficients.len() {
            temp_result += table[i].p_coefficients[j].a * f64::cos(table[i].p_coefficients[j].b + table[i].p_coefficients[j].c * t);
        }
        if i > 0 {
            temp_result *= tterm;
            tterm *= t;
        }
        result += temp_result;
    }

    if b_angle {
        result = aacoordinatetransformation::map_to_0to2pi_range(result);
    }

    result
}

pub fn calculate_dash(jd: f64, table: &[VSOP87Coefficient2], table_size: usize) -> f64
{
//Validate our parameters
    assert!(table.len() > 0);

    let t = (jd - 2451545.0) / 365250.0;
    let mut tterm1 = 1.0;
    let mut tterm2 = t;
    let mut result = 0.0;
    for i in 0..table_size {
        let mut temp_part1 = 0.0;
        let mut temp_part2 = 0.0;
        for j in 0..table[i].p_coefficients.len() {
            let b_ct: f64 = table[i].p_coefficients[j].b + table[i].p_coefficients[j].c * t;
            temp_part1 += i as f64 * table[i].p_coefficients[j].a * f64::cos(b_ct);
            temp_part2 += table[i].p_coefficients[j].a * table[i].p_coefficients[j].c * f64::sin(b_ct);
        }
        if i > 0 {
            temp_part1 *= tterm1;
            temp_part2 *= tterm2;
            tterm1 *= t;
            tterm2 *= t;
        }
        result += temp_part1 - temp_part2;
    }

    //The value returned is in per days
    result / 365250.0
}

