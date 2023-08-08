use chrono::NaiveDate;
use martial_arts::MartialArt;

mod experience_module {
    pub struct Experience {
        club_name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        martial_arts: Vec<MartialArt>,
    }
}
