mod martial_arts {

    pub struct MartialArt {
        name: String,
        punches: i8,
        kicks: i8,
        knees: i8,
        elbows: i8,
        standup_grappling: i8,
        ground_grappling: i8,
        ground_n_pound: i8,
        trapping: i8,
        weapon_manipulation: i8,
        weapon_defense: i8,
    }

    fn sum_equal_to_100(martial_art: MartialArt) {
        martial_art.punches
            + martial_art.kicks
            + martial_art.standup_grappling
            + martial_art.ground_grappling
            + martial_art.trapping
            + martial_art.weapon_manipulation
            + martial_art.weapon_defense
            + martial_art.knees
            + martial_art.elbows
            + martial_art.ground_n_pound
            == 100;
    }
}
