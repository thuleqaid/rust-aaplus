#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AACalendarDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AADate {
    m_dbl_julian: f64,
    m_b_gregorian_calendar: bool,
}

impl AADate {
    pub fn new() -> Self
    {
        Default::default()
    }
    pub fn from_date(year: i64, month: i64, day: f64, b_gregorian_calendar: bool) -> Self
    {
        let mut obj = Self::new();
        obj.set_datetime(year, month, day, 0.0, 0.0, 0.0, b_gregorian_calendar);
        obj
    }
    pub fn from_datetime(year: i64, month: i64, day: f64, hour: f64, minute: f64, second: f64, b_gregorian_calendar: bool) -> Self
    {
        let mut obj = Self::new();
        obj.set_datetime(year, month, day, hour, minute, second, b_gregorian_calendar);
        obj
    }
    pub fn from_jd(jd: f64, b_gregorian_calendar: bool) -> Self
    {
        let mut obj = Self::new();
        obj.set_jd(jd, b_gregorian_calendar);
        obj
    }
    pub fn julian(&self) -> f64
    {
        self.m_dbl_julian
    }
    pub fn day(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        day
    }
    pub fn month(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        month
    }
    pub fn year(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        year
    }
    pub fn hour(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        hour
    }
    pub fn minute(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        minute
    }
    pub fn second(&self) -> f64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        second
    }
    pub fn set_datetime(&mut self, year: i64, month: i64, day: f64, hour: f64, minute: f64, second: f64, b_gregorian_calendar: bool)
    {
        let dbl_day = day + (hour / 24.0) + (minute / 1440.0) + (second / 86400.0);
        self.set_jd(date_to_jd(year, month, dbl_day, b_gregorian_calendar), b_gregorian_calendar);
    }
    pub fn set_jd(&mut self, jd: f64, b_gregorian_calendar: bool)
    {
        self.m_dbl_julian = jd;
        self.set_in_gregorian_calendar(b_gregorian_calendar);
    }
    pub fn set_in_gregorian_calendar(&mut self, b_gregorian_calendar: bool)
    {
        let b_after_papal_reform = after_papal_reform_jd(self.m_dbl_julian);
        self.m_b_gregorian_calendar = b_gregorian_calendar && b_after_papal_reform;
    }
    pub fn get(&self, year: &mut i64, month: &mut i64, day: &mut i64, hour: &mut i64, minute: &mut i64, second: &mut f64)
    {
        let jd = self.m_dbl_julian + 0.5;
        let tempz = f64::trunc(jd);
        let mut f = f64::fract(jd);
        let z = tempz as i64;
        let a: i64;
        //There is a difference here between the Meeus implementation and this one
        //if (Z >= 2299161)       //The Meeus implementation automatically assumes the Gregorian Calendar
        //came into effect on 15 October 1582 (jd: 2299161), while the CAADate
        //implementation has a "m_bGregorianCalendar" value to decide if the date
        //was specified in the Gregorian or Julian Calendars. This difference
        //means in effect that CAADate fully supports a propalactive version of the
        //Julian calendar. This allows you to construct Julian dates after the Papal
        //reform in 1582. This is useful if you want to construct dates in countries
        //which did not immediately adapt the Gregorian calendar
        if self.m_b_gregorian_calendar {
            let alpha = int((z as f64 - 1867216.25) / 36524.25);
            a = z + 1 + alpha - int(alpha as f64 / 4.0);
        } else {
            a = z;
        }

        let b = a + 1524;
        let c = int((b as f64 - 122.1) / 365.25);
        let d = int(365.25 * c as f64);
        let e = int((0.0 + b as f64 - d as f64) / 30.6001);

        let dbl_day = 0.0 + b as f64 - d as f64 - int(30.6001 * e as f64) as f64 + f as f64;
        *day = f64::trunc(dbl_day) as i64;

        if e < 14 {
            *month = e - 1;
        } else {
            *month = e - 13;
        }

        if *month > 2 {
            *year = c - 4716;
        } else {
            *year = c - 4715;
        }

        f = f64::fract(dbl_day);
        *hour = int(f * 24.0);
        *minute = int((f - *hour as f64 / 24.0) * 1440.0);
        *second = (f - (*hour as f64 / 24.0) - (*minute as f64 / 1440.0)) * 86400.0;
    }
    pub fn day_of_week(&self) -> i64
    {
        (self.m_dbl_julian + 1.5) as i64 % 7
    }
    pub fn day_of_year(&self) -> f64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        day_of_year(self.m_dbl_julian, year, after_papal_reform_date(year, 1, 1.0))
    }
    pub fn days_in_month(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        days_in_month(month, is_leap(year, self.m_b_gregorian_calendar))
    }
    pub fn days_in_year(&self) -> i64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        if is_leap(year, self.m_b_gregorian_calendar) {
            366
        } else {
            365
        }
    }
    pub fn leap(&self) -> bool
    {
        is_leap(self.year(), self.m_b_gregorian_calendar)
    }
    pub fn in_gregorian_calendar(&self) -> bool
    {
        self.m_b_gregorian_calendar
    }
    pub fn fractional_year(&self) -> f64
    {
        let mut year = 0_i64;
        let mut month = 0_i64;
        let mut day = 0_i64;
        let mut hour = 0_i64;
        let mut minute = 0_i64;
        let mut second = 0.0_f64;
        self.get(&mut year, &mut month, &mut day, &mut hour, &mut minute, &mut second);
        let days_in_year: i64 = if is_leap(year, self.m_b_gregorian_calendar) { 366 } else { 365 };
        year as f64 + (self.m_dbl_julian - date_to_jd(year, 1, 1.0, after_papal_reform_date(year, 1, 1.0))) / days_in_year as f64
    }
}

pub fn after_papal_reform_date(year: i64, month: i64, day: f64) -> bool
{
    (year > 1582) || ((year == 1582) && (month > 10)) || ((year == 1582) && (month == 10) && (day > 15.0))
}

pub fn after_papal_reform_jd(jd: f64) -> bool
{
    jd >= 2299160.5
}

pub fn date_to_jd(year: i64, month: i64, day: f64, b_gregorian_calendar: bool) -> f64
{
    let mut y = year;
    let mut m = month;
    if m < 3 {
        y = y - 1;
        m = m + 12;
    }

    let mut b = 0i64;
    if b_gregorian_calendar {
        let a = int(y as f64 / 100.0);
        b = 2 - a + int(a as f64 / 4.0);
    }

    return 0.0 + int(365.25 * (y as f64 + 4716.0)) as f64 + int(30.6001 * (m as f64 + 1.0)) as f64 + day + b as f64 - 1524.5;
}

pub fn is_leap(year: i64, b_gregorian_calendar: bool) -> bool
{
    if b_gregorian_calendar {
        if (year % 100) == 0 {
            (year % 400) == 0
        } else {
            (year % 4) == 0
        }
    } else {
        (year % 4) == 0
    }
}

pub fn day_of_year_to_day_and_month(day_of_year: i64, b_leap: bool, day_of_month: &mut i64, month: &mut i64)
{
    let k = if b_leap { 1 } else { 2 };

    *month = int(9.0 * (0.0 + k as f64 + day_of_year as f64) / 275.0 + 0.98);
    if day_of_year < 32 {
        *month = 1;
    }

    *day_of_month = day_of_year - int((275.0 * (*month) as f64) / 9.0) + (k * int(((*month) as f64 + 9.0) / 12.0)) + 30;
}

pub fn julian_to_gregorian(year: i64, month: i64, day: i64) -> AACalendarDate
{
    let mut date = AADate::from_date(year, month, day as f64, false);
    date.set_in_gregorian_calendar(true);

    let mut gregorian_date: AACalendarDate = Default::default();
    let mut hour = 0_i64;
    let mut minute = 0_i64;
    let mut second = 0.0_f64;
    date.get(&mut gregorian_date.year, &mut gregorian_date.month, &mut gregorian_date.day, &mut hour, &mut minute, &mut second);
    gregorian_date
}

pub fn gregorian_to_julian(year: i64, month: i64, day: i64) -> AACalendarDate
{
    let mut date = AADate::from_date(year, month, day as f64, true);
    date.set_in_gregorian_calendar(false);

    let mut julian_date: AACalendarDate = Default::default();
    let mut hour = 0_i64;
    let mut minute = 0_i64;
    let mut second = 0.0_f64;
    date.get(&mut julian_date.year, &mut julian_date.month, &mut julian_date.day, &mut hour, &mut minute, &mut second);
    julian_date
}

pub fn int(value: f64) -> i64 {
    f64::floor(value) as i64
}

pub fn day_of_year(jd: f64, year: i64, b_gregorian_calendar: bool) -> f64
{
    jd - date_to_jd(year, 1, 1.0, b_gregorian_calendar) + 1.0
}

pub fn days_in_month(month: i64, b_leap: bool) -> i64
{
    //Validate our parameters
    assert!(month >= 1 && month <= 12);

    let non_leap_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_months = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if b_leap {
        leap_months[month as usize - 1]
    } else {
        non_leap_months[month as usize - 1]
    }
}

