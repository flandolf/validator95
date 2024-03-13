pub enum ValidationError {
    InvalidInput,
}

pub fn validate_algo(key: &str) -> Result<bool, ValidationError> {
    let key_parts: Vec<&str> = key.split("-").collect();
    if key_parts.len() < 3 {
        return Err(ValidationError::InvalidInput);
    }
    let algo_digits = key_parts[2];
    let mut sum = 0;
    for c in algo_digits.chars() {
        sum += c.to_digit(10).ok_or(ValidationError::InvalidInput)?;
    }
    Ok(sum % 7 == 0)
}

pub fn validate_day(key: &str) -> Result<bool, ValidationError> {
    let parts: Vec<char> = key.chars().collect();
    if parts.len() < 3 {
        return Err(ValidationError::InvalidInput);
    }
    if let Ok(day) = parts[0..2].iter().collect::<String>().parse::<u32>() {
        Ok(day > 0 && day < 366)
    } else {
        Err(ValidationError::InvalidInput)
    }
}

pub fn validate_year(key: &str) -> Result<bool, ValidationError> {
    let allowed_years: [&str; 9] = ["95", "96", "97", "98", "99", "00", "01", "02", "03"];
    let year = get_year(key);
    Ok(allowed_years.contains(&year.as_str()))
}

pub fn get_year(key: &str) -> String {
    let parts: Vec<&str> = key.split("").collect();
    let year: String = parts.get(4..6).unwrap_or(&[""]).join("");
    year
}

pub fn get_day(key: &str) -> Result<String, ValidationError> {
    let parts: Vec<&str> = key.split("").collect();
    let day: String = parts
        .get(1..4)
        .ok_or(ValidationError::InvalidInput)?
        .join("");
    Ok(day)
}

pub fn get_algo(key: &str) -> Result<String, ValidationError> {
    let key_parts: Vec<&str> = key.split("-").collect();
    let algo_digits = key_parts.get(2).ok_or(ValidationError::InvalidInput)?;
    Ok(algo_digits.to_string())
}

pub fn validate_random(key: &str) -> Result<bool, ValidationError> {
    let key_parts: Vec<&str> = key.split("-").collect();
    if key_parts.len() < 4 {
        return Err(ValidationError::InvalidInput);
    }
    let random = key_parts[3];
    Ok(random.len() == 5)
}

pub fn get_random(key: &str) -> Result<String, ValidationError> {
    let key_parts: Vec<&str> = key.split("-").collect();
    let random = key_parts.get(3).ok_or(ValidationError::InvalidInput)?;
    Ok(random.to_string())
}