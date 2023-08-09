use chrono::NaiveDate;
use martial_arts::MartialArt;

mod session_mod{
    pub struct Session{
        session_date: NaiveDate,
        session_duration_min: i32,
        martial_art: MartialArt,
        punches: i8,
        kicks: i8,
        knees: i8,
        elbows: i8,
        standup_grappling: i8,
        ground_grappling: i8,
        ground_n_pound: i8,
        trapping: i8,
        weapon_manipulation: i8,
        weapon_defense: i8
    }
}