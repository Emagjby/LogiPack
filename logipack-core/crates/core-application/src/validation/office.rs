use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OfficeValidationError {
    #[error("name too short")]
    NameTooShort,
    #[error("name too long")]
    NameTooLong,
    #[error("city too short")]
    CityTooShort,
    #[error("city too long")]
    CityTooLong,
    #[error("address too short")]
    AddressTooShort,
    #[error("address too long")]
    AddressTooLong,
}

pub fn validate_office(name: &str, city: &str, address: &str) -> Result<(), OfficeValidationError> {
    validate_name(name)?;
    validate_city(city)?;
    validate_address(address)?;
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), OfficeValidationError> {
    let trimmed = name.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(OfficeValidationError::NameTooShort);
    }

    if len > 100 {
        return Err(OfficeValidationError::NameTooLong);
    }

    Ok(())
}

pub fn validate_city(city: &str) -> Result<(), OfficeValidationError> {
    let trimmed = city.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(OfficeValidationError::CityTooShort);
    }

    if len > 100 {
        return Err(OfficeValidationError::CityTooLong);
    }

    Ok(())
}

pub fn validate_address(address: &str) -> Result<(), OfficeValidationError> {
    let trimmed = address.trim();
    let len = trimmed.chars().count();

    if len < 2 {
        return Err(OfficeValidationError::AddressTooShort);
    }

    if len > 200 {
        return Err(OfficeValidationError::AddressTooLong);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_office() {
        let result = validate_office("Main Office", "Sofia", "1 Test Street");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn empty_name_rejected() {
        let result = validate_name("   ");
        assert_eq!(result, Err(OfficeValidationError::NameTooShort));
    }

    #[test]
    fn short_name_rejected() {
        let result = validate_name("A");
        assert_eq!(result, Err(OfficeValidationError::NameTooShort));
    }

    #[test]
    fn long_name_rejected() {
        let name = "a".repeat(101);
        let result = validate_name(&name);
        assert_eq!(result, Err(OfficeValidationError::NameTooLong));
    }

    #[test]
    fn empty_city_rejected() {
        let result = validate_city("   ");
        assert_eq!(result, Err(OfficeValidationError::CityTooShort));
    }

    #[test]
    fn short_city_rejected() {
        let result = validate_city("A");
        assert_eq!(result, Err(OfficeValidationError::CityTooShort));
    }

    #[test]
    fn empty_address_rejected() {
        let result = validate_address("   ");
        assert_eq!(result, Err(OfficeValidationError::AddressTooShort));
    }

    #[test]
    fn short_address_rejected() {
        let result = validate_address("A");
        assert_eq!(result, Err(OfficeValidationError::AddressTooShort));
    }
}
