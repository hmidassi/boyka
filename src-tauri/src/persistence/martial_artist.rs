use experience_module::Experience;
use std::collections::Vec;

mod martial_artist_mod{
    pub struct MartialArtist{
        id: i16,
        first_name: String,
        last_name: String,
        experiences: Vec<Experience>
    }
}