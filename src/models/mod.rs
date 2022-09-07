#[allow(dead_code)]
/// An enum to track the level of Drunkness (0 - 5)
pub enum Drunkness {
    Cool,
    LittleHead,
    Bream,
    Gnat,
    Ant,
    ImOk,
}

impl Default for Drunkness {
    fn default() -> Self {
        Drunkness::Cool
    }
}

#[allow(dead_code)]
/// A struct to track the result of the night
pub struct Craziness {
    drunkness: Drunkness,
    coitus: bool,
    drive: bool,
    talked_2_ex: bool,
    // TODO: date field,
    location: String,
}

impl Default for Craziness {
    fn default() -> Self {
        Craziness {
            drunkness: Drunkness::default(),
            coitus: false,
            drive: false,
            talked_2_ex: false,
            location: "Athens".to_string(),
        }
    }
}
