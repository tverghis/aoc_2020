use std::convert::From;
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
