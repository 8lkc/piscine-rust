use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug)]
pub struct Flag {pub short_hand: String, pub long_hand: String, pub desc: String}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: String::from(format!("-{}", l_h.chars().next().unwrap())),
            long_hand: String::from(format!("--{}", l_h)),
            desc: String::from(d),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

#[derive(Debug)]
pub struct FlagsHandler {pub flags: HashMap<(String, String), Callback>}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {self.flags.insert(flag, func);}

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        if let Some(Callback) = self.flags.get(&flag) {
            if argv.len() == 2 {
                match Callback(argv[0], argv[1]) {
                    Ok(result) => result,
                    Err(_) => "invalid float literal".to_string(),
                }
            } else {"Error: Expected exactly two arguments.".to_string()}
        } else {"Error: Flag not found.".to_string()}
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f32 = a.parse()?;
    let num2: f32 = b.parse()?;
    let result = num1 / num2;
    Ok(result.to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f32 = a.parse()?;
    let num2: f32 = b.parse()?;
    let result = num1 % num2;
    Ok(result.to_string())
}
