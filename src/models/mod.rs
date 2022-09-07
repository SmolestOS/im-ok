#[allow(dead_code)]
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
            drunkness: Drunkness::Cool,
            coitus: false,
            drive: false,
            talked_2_ex: false,
            location: "Athens".to_string(),
        }
    }
}
