pub mod count_daily {
    use chrono::NaiveDate;
    use chrono::{Timelike, Utc};
    use chrono_tz::Europe::Paris;
    pub fn count_noons_since_start() -> usize {
        let third_of_may = NaiveDate::from_ymd_opt(2023, 5, 3).expect("Valid date");
        count_noons_since_date(&third_of_may)
    }
    pub fn count_noons_since_date(date: &NaiveDate) -> usize {
        let mut noon_count = 0;
        let mut current_date = *date;

        let paris_time = Utc::now().with_timezone(&Paris);

        // Convert to UTC time
        let current_time = paris_time;
        // Keep incrementing the current date by one day until today
        let today = current_time.date_naive();
        while current_date < today {
            current_date = current_date.succ_opt().expect("There should be a next day");
            noon_count += 1;
        }
        if current_time.hour() >= 12 {
            noon_count += 1;
        }

        println!("Number of noons since start: {}", noon_count);

        noon_count
    }
}

// #[cfg(test)]
// mod tests {
// }
