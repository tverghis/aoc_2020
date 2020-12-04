use lazy_static::lazy_static;
use regex::Regex;
use std::convert::From;

lazy_static! {
    static ref HCL_RE: Regex = Regex::new(r"^\#([0-9]|[a-f]){6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

#[derive(Default)]
pub struct Passport {
    byr: Option<&'static str>,
    iyr: Option<&'static str>,
    eyr: Option<&'static str>,
    hgt: Option<&'static str>,
    hcl: Option<&'static str>,
    ecl: Option<&'static str>,
    pid: Option<&'static str>,
    cid: Option<&'static str>,
}

impl Passport {
    pub fn has_all_req_fields(&self) -> bool {
        self.byr.is_some()
            & self.iyr.is_some()
            & self.eyr.is_some()
            & self.hgt.is_some()
            & self.hcl.is_some()
            & self.ecl.is_some()
            & self.pid.is_some()
    }

    pub fn validate_fields(&self) -> bool {
        self.validate_byr()
            & self.validate_iyr()
            & self.validate_eyr()
            & self.validate_hgt()
            & self.validate_hcl()
            & self.validate_ecl()
            & self.validate_pid()
    }

    fn validate_byr(&self) -> bool {
        match self.byr {
            None => false,
            Some(byr) => {
                let parsed_byr = byr.parse::<u32>().unwrap_or(0);
                parsed_byr >= 1920 && parsed_byr <= 2002
            }
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr {
            None => false,
            Some(iyr) => {
                let parsed_iyr = iyr.parse::<u32>().unwrap_or(0);
                parsed_iyr >= 2010 && parsed_iyr <= 2020
            }
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr {
            None => false,
            Some(eyr) => {
                let parsed_eyr = eyr.parse::<u32>().unwrap_or(0);
                parsed_eyr >= 2020 && parsed_eyr <= 2030
            }
        }
    }

    fn validate_hgt(&self) -> bool {
        match self.hgt {
            None => false,
            Some(hgt) => {
                let unit = &hgt[hgt.len() - 2..];
                let value = hgt[..hgt.len() - 2].parse::<u32>().unwrap_or(0);
                match unit {
                    "cm" => value >= 150 && value <= 193,
                    "in" => value >= 59 && value <= 76,
                    _ => false,
                }
            }
        }
    }

    fn validate_hcl(&self) -> bool {
        match self.hcl {
            None => false,
            Some(hcl) => HCL_RE.is_match(hcl),
        }
    }

    fn validate_ecl(&self) -> bool {
        match self.ecl {
            None => false,
            Some(ecl) => ECL_RE.is_match(ecl),
        }
    }

    fn validate_pid(&self) -> bool {
        match self.pid {
            None => false,
            Some(pid) => PID_RE.is_match(pid),
        }
    }
}

impl From<Vec<&'static str>> for Passport {
    fn from(input: Vec<&'static str>) -> Self {
        let mut passport = Passport::default();

        for line in input {
            line.split_whitespace().for_each(|f| {
                let field_split = f.split(':').collect::<Vec<_>>();

                let name = field_split[0];
                let value = field_split[1];

                match name {
                    "byr" => passport.byr = Some(value),
                    "iyr" => passport.iyr = Some(value),
                    "eyr" => passport.eyr = Some(value),
                    "hgt" => passport.hgt = Some(value),
                    "hcl" => passport.hcl = Some(value),
                    "ecl" => passport.ecl = Some(value),
                    "pid" => passport.pid = Some(value),
                    "cid" => passport.cid = Some(value),
                    _ => unreachable!(),
                }
            });
        }

        passport
    }
}
