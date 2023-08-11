use experience_module::Experience;
use std::collections::Vec;

use super::schema::martial_artist;
use super::schema::martial_artist::dsl::martial_artist as martial_artist_dsl;

mod martial_artist_mod{
    pub struct MartialArtist{
        id: i16,
        first_name: String,
        last_name: String,
        experiences: Vec<Experience>
    }

    impl MartialArtist{

    }
}