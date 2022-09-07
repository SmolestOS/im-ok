pub enum Drunkness {
    pub Cool,
    pub LittleHead,
    pub Bream,
    pub Gnat,
    pub Ant,
    pub ImOk,
}

impl Default for Drunkness{
    fn default() -> Self {
        Drunkness::Cool
    }
}

pub struct Craziness {
    pub drunkness: Drunkness,
    pub coitus: bool,
    pub drive: bool,
    pub talked_2_ex: bool,
    //pub date,
    pub location: String,
}

impl Default for Craziness {
    fn default() -> Self {
        Craziness{
            drunkness: Drunkness::Cool,
            coitus: false,
            drive: false,
            talked_2_ex: false,
            location: "Athens".to_string(),
        }
    }
}
