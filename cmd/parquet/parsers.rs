#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Duration {
    Year(u16),
    Month(u16),
}

pub fn duration_parser(duration_string: &str) -> Result<Duration, String> {
    let is_month_suffixed = duration_string.to_ascii_lowercase().ends_with("m");
    let is_year_suffixed = duration_string.to_ascii_lowercase().ends_with("y");

    let duration_value: u16 = {
        match (is_month_suffixed, is_year_suffixed) {
            (true, false) => Ok(duration_string.strip_suffix("m").unwrap()),
            (false, true) => Ok(duration_string.strip_suffix("y").unwrap()),
            (_, _) => Err(format!(
                "`{duration_string}` not recognizable prefixed. Only `y` and `m` are valids."
            )),
        }
    }?
    .parse()
    .map_err(|_| format!("`{duration_string}` isn't a valid number"))?;

    match (is_month_suffixed, is_year_suffixed) {
        (true, false) => Ok(Duration::Month(duration_value)),
        (false, true) => Ok(Duration::Year(duration_value)),
        _ => Err(format!(
            "`{duration_string}` not recognizable prefixed. Only `y` and `m` are valids."
        )),
    }
}

pub fn positive_integer_parser(customer_number: &str) -> Result<u16, String> {
    let customer_amount: u16 = customer_number
        .parse()
        .map_err(|_| format!("`{customer_number}` isn't a valid number"))?;

    if customer_amount == 0 {
        return Err("Only values higher than 0 are valid".to_string());
    }

    Ok(customer_amount)
}

#[cfg(test)]
mod parser_duration_arg_value_test {
    use super::*;

    macro_rules! valid_duration_parametrized_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;

                    let result = duration_parser(&input);

                    assert!(result.is_ok());
                    assert_eq!(result.ok().unwrap(), expected);
                }
            )*
        };
    }

    macro_rules! invalid_duration_parametrized_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;

                    let result = duration_parser(&input);

                    assert!(result.is_err());
                    assert_eq!(result.err().unwrap(), expected);
                }
            )*
        };
    }

    valid_duration_parametrized_tests! {
        test_given_a_valid_year_duration_value_then_return_the_arg_value_correctly_parsed: ("10y", Duration::Year(10)),
        test_given_a_valid_month_duration_value_then_return_the_arg_value_correctly_parsed: ("10m", Duration::Month(10)),
        test_given_a_valid_year_duration_in_uppercase_value_then_return_the_arg_value_correctly_parsed: ("10y", Duration::Year(10)),
        test_given_a_valid_month_duration_in_uppercase_value_then_return_the_arg_value_correctly_parsed: ("10m", Duration::Month(10)),
    }

    invalid_duration_parametrized_tests! {
        test_given_an_invalid_duration_suffix_then_return_error: ("10s", "`10s` not recognizable prefixed. Only `y` and `m` are valids."),
        test_given_an_negative_duration_then_return_error: ("-10m", "`-10m` isn't a valid number"),
    }
}

#[cfg(test)]
mod parser_number_customers_arg_value_test {
    use super::*;

    #[test]
    fn test_given_a_non_zero_positive_number_then_return_the_arg_value_correctly_parsed() {
        let input = "3000";

        let result = positive_integer_parser(&input);

        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), 3000);
    }

    #[test]
    fn test_given_zero_number_of_customers_then_return_the_arg_value_correctly_parsed() {
        let input = "0";

        let result = positive_integer_parser(&input);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Only values higher than 0 are valid");
    }

    #[test]
    fn test_given_an_negative_number_then_return_an_error() {
        let input = "-1000";

        let result = positive_integer_parser(&input);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "`-1000` isn't a valid number");
    }
}
