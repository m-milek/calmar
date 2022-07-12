
#[cfg(test)]
mod tests {
	use crate::validator::validate_date;
	#[test]
	fn standard_date(){
		assert_eq!(true, validate_date("21/12/2022"));
	}
	#[test]
	fn date_with_zeros(){
		assert_eq!(true, validate_date("01/01/2000"));
		assert_eq!(true, validate_date("04/11/2000"));
		assert_eq!(true, validate_date("24/03/2000"));
	}
	#[test]
	fn zeros(){
		assert_eq!(false, validate_date("00/00/0000"));
		assert_eq!(false, validate_date("01/00/0000"));
		assert_eq!(false, validate_date("01/02/0000"));
		assert_eq!(false, validate_date("01/02/0001"));
		assert_eq!(true, validate_date("01/02/2021"));
		assert_eq!(true, validate_date("01/12/2021"));
		assert_eq!(true, validate_date("11/12/2021"));
	}
	#[test]
	fn invalid_day(){
		assert_eq!(false, validate_date("32/10/2022"));
		assert_eq!(false, validate_date("43/10/2022"));
	}
	#[test]
	fn invalid_month(){
		assert_eq!(false, validate_date("10/13/2022"));
		assert_eq!(false, validate_date("10/30/2022"));
	}
	#[test]
	fn year_too_early(){
		assert_eq!(false, validate_date("10/11/102"));
		assert_eq!(false, validate_date("10/11/1969"));
	}
	#[test]
	fn day_31_in_month_with_30_days(){
		assert_eq!(false, validate_date("31/02/2022"));
		assert_eq!(false, validate_date("31/04/2022"));
		assert_eq!(false, validate_date("31/06/2022"));
		assert_eq!(false, validate_date("31/09/2022"));
		assert_eq!(false, validate_date("31/11/2022"));
	}
	#[test]
	fn more_than_day_28_in_february_if_not_leap_year(){
		assert_eq!(false, validate_date("29/02/2022"));
		assert_eq!(false, validate_date("29/02/2017"));
		assert_eq!(false, validate_date("29/02/2018"));
		assert_eq!(false, validate_date("29/02/2019"));
		assert_eq!(false, validate_date("29/02/2021"));
		assert_eq!(false, validate_date("30/02/2022"));
		assert_eq!(false, validate_date("31/02/2022"));
		assert_eq!(false, validate_date("32/02/2022"));
	}
	#[test]
	fn day_29_in_february_if_leap_year(){
		assert_eq!(true, validate_date("29/02/2020"));
		assert_eq!(true, validate_date("29/02/2016"));
		assert_eq!(true, validate_date("29/02/2024"));
		assert_eq!(true, validate_date("29/02/2028"));
		assert_eq!(true, validate_date("29/02/2032"));
		assert_eq!(true, validate_date("29/02/2036"));

	}
}
