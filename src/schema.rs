table! {
    nouns (id_noun) {
        id_noun -> Integer,
        gender -> Text,
        animate -> Integer,
        sing_nominative -> Nullable<Text>,
        sing_genitive -> Nullable<Text>,
        sing_dative -> Nullable<Text>,
        sing_accusative -> Nullable<Text>,
        sing_instrumental -> Nullable<Text>,
        sing_prepositional -> Nullable<Text>,
        sing_locative -> Nullable<Text>,
        plur_nominative -> Nullable<Text>,
        plur_genitive -> Nullable<Text>,
        plur_dative -> Nullable<Text>,
        plur_accusative -> Nullable<Text>,
        plur_instrumental -> Nullable<Text>,
        plur_prepositional -> Nullable<Text>,
        plur_locative -> Nullable<Text>,
    }
}
