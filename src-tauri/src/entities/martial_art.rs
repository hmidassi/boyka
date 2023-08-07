mod martial_arts {

    pub struct MartialArt {
        name: String,
        punches: i32,
        kicks: i32,
        standup_grappling: i32,
        ground_grappling: i32,
        trapping: i32,
        weapon_manipulation: i32,
        weapon_defense: i32,
    }

    fn sum_equal_to_100(martial_art: MartialArt){
        martial_art.punches + martial_art.kicks + martial_art.standup_grappling 
        +martial_art.ground_grappling + martial_art.trapping + martial_art.weapon_manipulation + martial_art.weapon_defense == 100;
    }
}
