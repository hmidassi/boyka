use chrono::NaiveDate;
use martial_arts::MartialArt;

mod experience_module {
    pub struct Experience {
        start_date: NaiveDate,
        end_date: NaiveDate,
        martial_art: MartialArt,
    }
}
