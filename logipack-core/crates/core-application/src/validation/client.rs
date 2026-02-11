use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ClientValidationError {
    #[error("name too short")]
    NameTooShort,
    #[error("name too long")]
    NameTooLong,
    #[error("invalid email")]
    InvalidEmail,
    #[error("invalid phone")]
    InvalidPhone,
}

pub fn validate_client(
    name: &str,
    email: &str,
    phone: Option<&str>,
) -> Result<(), ClientValidationError> {
    validate_name(name)?;
    validate_email(email)?;
    validate_phone(phone)?;
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), ClientValidationError> {
    let trimmed = name.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(ClientValidationError::NameTooShort);
    }

    if len > 100 {
        return Err(ClientValidationError::NameTooLong);
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), ClientValidationError> {
    let trimmed = email.trim();

    if trimmed.is_empty() {
        return Err(ClientValidationError::InvalidEmail);
    }

    if trimmed.len() > 254 {
        return Err(ClientValidationError::InvalidEmail);
    }

    if trimmed.contains(' ') {
        return Err(ClientValidationError::InvalidEmail);
    }

    let mut split = trimmed.split('@');
    let local = split.next().unwrap_or("");
    let domain = split.next().unwrap_or("");

    if local.is_empty() || domain.is_empty() {
        return Err(ClientValidationError::InvalidEmail);
    }

    if split.next().is_some() {
        return Err(ClientValidationError::InvalidEmail);
    }

    if !domain.contains('.') {
        return Err(ClientValidationError::InvalidEmail);
    }

    Ok(())
}

pub fn validate_phone(phone: Option<&str>) -> Result<(), ClientValidationError> {
    let Some(value) = phone else {
        return Ok(());
    };

    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(ClientValidationError::InvalidPhone);
    }

    let len = trimmed.chars().count();
    if !(8..=15).contains(&len) {
        return Err(ClientValidationError::InvalidPhone);
    }

    let mut chars = trimmed.chars();
    if chars.next() != Some('+') {
        return Err(ClientValidationError::InvalidPhone);
    }

    if !chars.all(|ch| ch.is_ascii_digit()) {
        return Err(ClientValidationError::InvalidPhone);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_client() {
        let result = validate_client("Jane Doe", "jane.doe@example.com", Some("+12025550123"));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn empty_name_rejected() {
        let result = validate_name("   ");
        assert_eq!(result, Err(ClientValidationError::NameTooShort));
    }

    #[test]
    fn short_name_rejected() {
        let result = validate_name("A");
        assert_eq!(result, Err(ClientValidationError::NameTooShort));
    }

    #[test]
    fn invalid_email_cases() {
        let cases = [
            "",
            "john",
            "john@",
            "@example.com",
            "john@example",
            "john@@example.com",
            "john example@example.com",
        ];

        for email in cases {
            let result = validate_email(email);
            assert_eq!(result, Err(ClientValidationError::InvalidEmail));
        }
    }

    #[test]
    fn invalid_phone_cases() {
        let cases = [
            "",
            "12345678",
            "+123",
            "+1234567890123456",
            "+12 3456789",
            "+12345678x",
        ];

        for phone in cases {
            let result = validate_phone(Some(phone));
            assert_eq!(result, Err(ClientValidationError::InvalidPhone));
        }
    }

    #[test]
    fn phone_optional_case() {
        let result = validate_phone(None);
        assert_eq!(result, Ok(()));
    }
}
