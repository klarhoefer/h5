
macro_rules! int {
    ($c:ident) => {
        ($c as u32 - '0' as u32)
    }
}

pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8
}

impl DateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        DateTime { year, month, day, hour, minute, second }
    }

    pub fn parse(s: &str) -> Result<Self, ()> {
        let mut year: u16 = 0;
        let mut month: u8 = 0;
        let mut day: u8 = 0;
        let mut hour: u8 = 0;
        let mut minute: u8 = 0;
        let mut second: u8 = 0;

        for (i, c) in s.chars().enumerate() {
            match i {
                8 | 15 => continue,
                j if j < 4 => year = year * 10 + int!(c) as u16,
                j if j < 6 => month = month * 10 + int!(c) as u8,
                j if j < 8 => day = day * 10 + int!(c) as u8,
                j if j < 11 => hour = hour * 10 + int!(c) as u8,
                j if j < 13 => minute = minute * 10 + int!(c) as u8,
                j if j < 15 => second = second * 10 + int!(c) as u8,
                _ => return Err(()),
            }
        }
        Ok(DateTime { year, month, day, hour, minute, second })
    }

    pub fn to_string(&self) -> String {
        format!("{:04}{:02}{:02}T{:02}{:02}{:02}z", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}


#[cfg(test)]
mod tests {

    use super::{DateTime};

    #[test]
    fn new() {
        let dt = DateTime::new(2020, 6, 17, 1, 23, 45);
        assert_eq!("20200617T012345z", dt.to_string().as_str());
    }

    #[test]
    fn parse() {
        let dt = DateTime::parse("20200617T012345z").unwrap();
        assert_eq!("20200617T012345z", dt.to_string().as_str());
    }
}
