#[derive(Queryable)]
pub struct Noun {
    pub id: i32,

    pub gender: String,
    pub animate: i32,

    pub sing_nominative: Option<String>,
    pub sing_genitive: Option<String>,
    pub sing_dative: Option<String>,
    pub sing_accusative: Option<String>,
    pub sing_instrumental: Option<String>,
    pub sing_prepositional: Option<String>,
    pub sing_locative: Option<String>,

    pub plur_nominative: Option<String>,
    pub plur_genitive: Option<String>,
    pub plur_dative: Option<String>,
    pub plur_accusative: Option<String>,
    pub plur_instrumental: Option<String>,
    pub plur_prepositional: Option<String>,
    pub plur_locative: Option<String>,
}

impl Noun {
    pub fn new() -> Noun {
        Noun {
            id: 0,
            gender: String::new(),
            animate: 0,
            sing_nominative: None,
            sing_genitive: None,
            sing_dative: None,
            sing_accusative: None,
            sing_instrumental: None,
            sing_prepositional: None,
            sing_locative: None,
        
            plur_nominative: None,
            plur_genitive: None,
            plur_dative: None,
            plur_accusative: None,
            plur_instrumental: None,
            plur_prepositional: None,
            plur_locative: None,
        }
    }
}