extern crate diesel;

use ru_practice::establish_connection;
use ru_practice::models::*;
use self::diesel::prelude::*;

//todo: I probably should check for errors here, I'm just unwrapping everything for now
pub fn get_declension_table(word: &str) -> Vec<Vec<String>> {
    use ru_practice::schema::nouns::dsl::*;

    let mut declension_table = vec![vec![String::new(); 2]; 6];

    let connection = establish_connection();
    let result = nouns.filter(sing_nominative.eq(word.to_string()))
        .limit(1)
        .load::<Noun>(&connection)
        .expect("Word not found");

    if result.len() > 0 {
        let r = result.get(0).unwrap();

        declension_table[0][0] = r.sing_nominative.as_ref().unwrap().to_string();
        declension_table[1][0] = r.sing_genitive.as_ref().unwrap().to_string();
        declension_table[2][0] = r.sing_dative.as_ref().unwrap().to_string();
        declension_table[3][0] = r.sing_accusative.as_ref().unwrap().to_string();
        declension_table[4][0] = r.sing_instrumental.as_ref().unwrap().to_string();
        declension_table[5][0] = r.sing_prepositional.as_ref().unwrap().to_string();

        declension_table[0][1] = r.plur_nominative.as_ref().unwrap().to_string();
        declension_table[1][1] = r.plur_genitive.as_ref().unwrap().to_string();
        declension_table[2][1] = r.plur_dative.as_ref().unwrap().to_string();
        declension_table[3][1] = r.plur_accusative.as_ref().unwrap().to_string();
        declension_table[4][1] = r.plur_instrumental.as_ref().unwrap().to_string();
        declension_table[5][1] = r.plur_prepositional.as_ref().unwrap().to_string();
        
    }
    declension_table
}
