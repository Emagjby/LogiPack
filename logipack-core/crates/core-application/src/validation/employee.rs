use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmployeeValidationError {
    #[error("full name too short")]
    FullNameTooShort,
    #[error("full name too long")]
    FullNameTooLong,
}

pub fn validate_full_name(full_name: &str) -> Result<(), EmployeeValidationError> {
    let trimmed = full_name.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(EmployeeValidationError::FullNameTooShort);
    }

    if len > 100 {
        return Err(EmployeeValidationError::FullNameTooLong);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_full_name() {
        let result = validate_full_name("John Doe");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn empty_full_name_rejected() {
        let result = validate_full_name("   ");
        assert_eq!(result, Err(EmployeeValidationError::FullNameTooShort));
    }

    #[test]
    fn short_full_name_rejected() {
        let result = validate_full_name("A");
        assert_eq!(result, Err(EmployeeValidationError::FullNameTooShort));
    }

    #[test]
    fn long_full_name_rejected() {
        let name = "a".repeat(101);
        let result = validate_full_name(&name);
        assert_eq!(result, Err(EmployeeValidationError::FullNameTooLong));
    }
}
