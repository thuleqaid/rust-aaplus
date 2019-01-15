#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AAEasterDetails
{
    pub month: i64,
    pub day: i64,
}

pub fn calculate(n_year: i64, gregorian_calendar: bool) -> AAEasterDetails
{
    let mut details: AAEasterDetails = Default::default();

    if gregorian_calendar {
        let a = n_year % 19;
        let b = n_year / 100;
        let c = n_year % 100;
        let d = b / 4;
        let e = b % 4;
        let f = (b + 8) / 25;
        let g = (b - f + 1) / 3;
        let h = (19 * a + b - d - g + 15) % 30;
        let i = c / 4;
        let k = c % 4;
        let l = (32 + 2 * e + 2 * i - h - k) % 7;
        let m = (a + 11 * h + 22 * l) / 451;
        let n = (h + l - 7 * m + 114) / 31;
        let p = (h + l - 7 * m + 114) % 31;
        details.month = n;
        details.day = p + 1;
    } else {
        let a = n_year % 4;
        let b = n_year % 7;
        let c = n_year % 19;
        let d = (19 * c + 15) % 30;
        let e = (2 * a + 4 * b - d + 34) % 7;
        let f = (d + e + 114) / 31;
        let g = (d + e + 114) % 31;
        details.month = f;
        details.day = g + 1;
    }

    details
}

