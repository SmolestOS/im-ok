#[derive(Debug, PartialEq)]
#[allow(dead_code)]
/// An enum to track the level of Drunkness (0 - 5)
pub enum User {
    Lostsaka,
    Gkasma,
}

impl Default for User {
    fn default() -> Self {
        Self::Lostsaka
    }
}

#[derive(Debug, PartialEq)]
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
        Self::Cool
    }
}

#[allow(dead_code)]
/// A struct to track the result of the night
pub struct Craziness {
    pub user: User,
    pub drunkness: Drunkness,
    pub coitus: bool,
    pub drive: bool,
    pub talked_2x: bool,
    // TODO: date field,
    pub location: String,
}

impl Default for Craziness {
    fn default() -> Self {
        Craziness {
            user: User::default(),
            drunkness: Drunkness::default(),
            coitus: false,
            drive: false,
            talked_2x: false,
            location: "Athens".to_string(),
        }
    }
}
