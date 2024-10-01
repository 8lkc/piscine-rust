pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        let date = Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
        FormError {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors = Vec::new();

        // Validate first name
        if self.first_name.is_empty() {
            return Err(FormError::new(
                "first_name".to_string(),
                self.first_name.clone(),
                "No user name".to_string(),
            ));
        } else {
            errors.push("Valid first name");
        }

        // Validate password
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "At least 8 characters".to_string(),
            ));
        }

        let has_alpha = self.password.chars().any(|c| c.is_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_digit(10));
        let has_special = self.password.chars().any(|c| !c.is_alphanumeric());

        if !(has_alpha && has_digit && has_special) {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            ));
        } else {
            errors.push("Valid password");
        }

        Ok(errors)
    }
}

// Helper function to create a NaiveDate from a string
pub fn create_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}
