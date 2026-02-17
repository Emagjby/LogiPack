use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserValidationError {
    #[error("name too short")]
    NameTooShort,
    #[error("name too long")]
    NameTooLong,
    #[error("invalid email")]
    InvalidEmail,
}

pub fn validate_name(name: &str) -> Result<(), UserValidationError> {
    let trimmed = name.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(UserValidationError::NameTooShort);
    }

    if len > 100 {
        return Err(UserValidationError::NameTooLong);
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), UserValidationError> {
    let trimmed = email.trim();

    if trimmed.is_empty() {
        return Err(UserValidationError::InvalidEmail);
    }

    if trimmed.len() > 254 {
        return Err(UserValidationError::InvalidEmail);
    }

    if trimmed.contains(' ') {
        return Err(UserValidationError::InvalidEmail);
    }

    let mut split = trimmed.split('@');
    let local = split.next().unwrap_or("");
    let domain = split.next().unwrap_or("");

    if local.is_empty() || domain.is_empty() {
        return Err(UserValidationError::InvalidEmail);
    }

    if split.next().is_some() {
        return Err(UserValidationError::InvalidEmail);
    }

    if !domain.contains('.') {
        return Err(UserValidationError::InvalidEmail);
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(UserValidationError::InvalidEmail);
    }

    if domain.contains("..") {
        return Err(UserValidationError::InvalidEmail);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert_eq!(validate_name("Jane Doe"), Ok(()));
    }

    #[test]
    fn empty_name_rejected() {
        assert_eq!(validate_name("   "), Err(UserValidationError::NameTooShort));
    }

    #[test]
    fn short_name_rejected() {
        assert_eq!(validate_name("A"), Err(UserValidationError::NameTooShort));
    }

    #[test]
    fn valid_email() {
        assert_eq!(validate_email("jane@example.com"), Ok(()));
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
            assert_eq!(
                validate_email(email),
                Err(UserValidationError::InvalidEmail)
            );
        }
    }
}
