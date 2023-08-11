use chrono::NaiveDate;
use martial_arts::MartialArt;

mod experience_module {
    enum Experience{
        CurrentExperience{experience: GeneralExperience},
        PastExperience{experience: GeneralExperience, end_date: NaiveDate}
    }

    pub struct GeneralExperience {
        id: i16,
        club_name: String,
        starting_date: NaiveDate,
        //one club may offer several martial arts at once
        martial_arts: Vec<MartialArt>,
        //find a smarter way to associate each martial art and its average duration per
        average_practice_per_week_in_min: Vec<i16>
    }
}
