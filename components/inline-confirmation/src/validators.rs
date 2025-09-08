use crate::types::*;
use regex::Regex;

/// Validate input based on rules
pub fn validate_input(value: &str, rules: &[ValidationRule]) -> ValidationState {
    for rule in rules {
        match rule {
            ValidationRule::Required(msg) => {
                if value.trim().is_empty() {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::MinLength(min, msg) => {
                if value.len() < *min {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::MaxLength(max, msg) => {
                if value.len() > *max {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::Pattern(pattern, msg) => {
                if let Ok(re) = Regex::new(pattern) {
                    if !re.is_match(value) {
                        return ValidationState::Invalid(msg.clone());
                    }
                }
            }
            ValidationRule::Email(msg) => {
                if !is_valid_email(value) {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::Url(msg) => {
                if !is_valid_url(value) {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::Range(min, max, msg) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num < *min || num > *max {
                        return ValidationState::Invalid(msg.clone());
                    }
                } else {
                    return ValidationState::Invalid("Invalid number".to_string());
                }
            }
            ValidationRule::PhilippineTIN(msg) => {
                if !is_valid_philippine_tin(value) {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::PhilippinePhone(msg) => {
                if !is_valid_philippine_phone(value) {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::PhilippineZip(msg) => {
                if !is_valid_philippine_zip(value) {
                    return ValidationState::Invalid(msg.clone());
                }
            }
            ValidationRule::Custom(_, _) => {
                // Custom validation handled externally
            }
        }
    }
    
    ValidationState::Valid(None)
}

/// Email validation
fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex.is_match(email)
}

/// URL validation
fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(
        r"^(https?://)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(/.*)?$"
    ).unwrap();
    url_regex.is_match(url)
}

/// Philippine TIN validation
fn is_valid_philippine_tin(tin: &str) -> bool {
    // Remove non-digits
    let digits: String = tin.chars().filter(|c| c.is_ascii_digit()).collect();
    
    // TIN should be 9 or 12 digits
    if digits.len() != 9 && digits.len() != 12 {
        return false;
    }
    
    // Basic check digit validation (simplified)
    // Real TIN validation uses modulo 11 algorithm
    if digits.len() == 9 {
        // 9-digit TIN validation
        let check_digit = calculate_tin_check_digit(&digits[..8]);
        digits.chars().nth(8).unwrap().to_digit(10).unwrap() == check_digit
    } else {
        // 12-digit TIN (with branch code)
        let check_digit = calculate_tin_check_digit(&digits[..8]);
        digits.chars().nth(8).unwrap().to_digit(10).unwrap() == check_digit
    }
}

/// Calculate TIN check digit using modulo 11
fn calculate_tin_check_digit(tin_base: &str) -> u32 {
    let weights = [2, 7, 6, 5, 4, 3, 2, 7];
    let mut sum = 0;
    
    for (i, ch) in tin_base.chars().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            sum += digit * weights[i];
        }
    }
    
    let remainder = sum % 11;
    if remainder == 0 || remainder == 1 {
        0
    } else {
        11 - remainder
    }
}

/// Philippine phone number validation
fn is_valid_philippine_phone(phone: &str) -> bool {
    // Remove non-digits except +
    let cleaned: String = phone.chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();
    
    // Check patterns
    if cleaned.starts_with("+639") && cleaned.len() == 13 {
        return true;
    }
    if cleaned.starts_with("639") && cleaned.len() == 12 {
        return true;
    }
    if cleaned.starts_with("09") && cleaned.len() == 11 {
        return true;
    }
    
    false
}

/// Philippine ZIP code validation
fn is_valid_philippine_zip(zip: &str) -> bool {
    // Philippine ZIP codes are 4 digits
    if zip.len() != 4 {
        return false;
    }
    
    // Must be all digits
    if !zip.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // Check if it's a valid range (1000-9999)
    if let Ok(code) = zip.parse::<u32>() {
        code >= 1000 && code <= 9999
    } else {
        false
    }
}

/// Format currency for Philippines
pub fn format_philippine_currency(amount: f64) -> String {
    let formatted = format!("{:.2}", amount);
    let parts: Vec<&str> = formatted.split('.').collect();
    let whole = parts[0];
    let decimal = parts.get(1).unwrap_or(&"00");
    
    // Add thousand separators
    let mut result = String::new();
    for (i, c) in whole.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    
    format!("₱{}.{}", result, decimal)
}

/// Format percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> ValidationState {
    let mut strength_score = 0;
    let mut issues = Vec::new();
    
    // Length check
    if password.len() >= 8 {
        strength_score += 1;
    } else {
        issues.push("at least 8 characters");
    }
    
    // Uppercase check
    if password.chars().any(|c| c.is_uppercase()) {
        strength_score += 1;
    } else {
        issues.push("one uppercase letter");
    }
    
    // Lowercase check
    if password.chars().any(|c| c.is_lowercase()) {
        strength_score += 1;
    } else {
        issues.push("one lowercase letter");
    }
    
    // Number check
    if password.chars().any(|c| c.is_ascii_digit()) {
        strength_score += 1;
    } else {
        issues.push("one number");
    }
    
    // Special character check
    if password.chars().any(|c| !c.is_alphanumeric()) {
        strength_score += 1;
    } else {
        issues.push("one special character");
    }
    
    match strength_score {
        5 => ValidationState::Valid(Some("Strong password".to_string())),
        3..=4 => ValidationState::Warning(format!("Medium strength. Consider adding: {}", issues.join(", "))),
        _ => ValidationState::Invalid(format!("Weak password. Must have: {}", issues.join(", "))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@company.co.uk"));
        assert!(!is_valid_email("invalid.email"));
        assert!(!is_valid_email("@example.com"));
    }
    
    #[test]
    fn test_philippine_phone() {
        assert!(is_valid_philippine_phone("09123456789"));
        assert!(is_valid_philippine_phone("+639123456789"));
        assert!(is_valid_philippine_phone("639123456789"));
        assert!(!is_valid_philippine_phone("08123456789"));
        assert!(!is_valid_philippine_phone("123456789"));
    }
    
    #[test]
    fn test_philippine_zip() {
        assert!(is_valid_philippine_zip("1000"));
        assert!(is_valid_philippine_zip("1234"));
        assert!(is_valid_philippine_zip("9999"));
        assert!(!is_valid_philippine_zip("999"));
        assert!(!is_valid_philippine_zip("10000"));
        assert!(!is_valid_philippine_zip("abcd"));
    }
}