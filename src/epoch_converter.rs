use crate::error::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct DateTime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    second: u64,
}

impl DateTime {
    pub fn new(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) -> Result<DateTime, Error> {
        if year >= 1970
            && month >= 1
            && month <= 12
            && day >= 1
            && day * 86400 <= length_of_month(month, year)
            && hour <= 23
            && minute <= 59
            && second <= 59
        {
            Ok(DateTime { year, month, day, hour, minute, second })
        } else {
            Err(Error::new(ErrorKind::InvalidDate))
        }
    }

    pub fn from_epoch(mut epoch: u64) -> DateTime {
        let mut year = 1970;

        while epoch >= length_of_year(year) {
            epoch -= length_of_year(year);
            year += 1;
        }

        let mut month = 1;

        while epoch >= length_of_month(month, year) {
            epoch -= length_of_month(month, year);
            month += 1;
        }

        let day = epoch / 86400 + 1;
        epoch %= 86400;

        let hour = epoch / 3600;
        epoch %= 3600;

        let minute = epoch / 60;
        epoch %= 60;

        let second = epoch;

        DateTime::new(year, month, day, hour, minute, second).unwrap()
    }

    pub fn to_epoch(&self) -> u64 {
        let mut epoch = 0;

        for i in 1970..self.year {
            epoch += length_of_year(i);
        }

        for i in 1..self.month {
            epoch += length_of_month(i, self.year);
        }

        epoch += (self.day - 1) * 86400 + self.hour * 3600 + self.minute * 60 + self.second;

        epoch
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{year}-{month:0>2}-{day:0>2}T{hour:0>2}:{minute:0>2}:{second:0>2}Z",
            year = self.year,
            month = self.month,
            day = self.day,
            hour = self.hour,
            minute = self.minute,
            second = self.second,
        )
    }
}

fn is_leap_year(year: u64) -> bool {
    (year % 100 == 0 && year % 400 == 0) || (year % 100 != 0 && year % 4 == 0)
}

fn length_of_year(year: u64) -> u64 {
    if is_leap_year(year) {
        31622400
    } else {
        31536000
    }
}

fn length_of_month(month: u64, year: u64) -> u64 {
    if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
        2678400
    } else if [4, 6, 9, 11].contains(&month) {
        2592000
    } else if is_leap_year(year) {
        2505600
    } else {
        2419200
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_to_date_test() {
        assert_eq!(DateTime::new(2001, 9, 9, 1, 46, 40).unwrap(), DateTime::from_epoch(1000000000));
    }

    #[test]
    fn date_to_epoch_test() {
        assert_eq!(1000000000, DateTime::new(2001, 9, 9, 1, 46, 40).unwrap().to_epoch(),);
    }
}
