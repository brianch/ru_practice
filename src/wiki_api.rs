extern crate curl;
extern crate serde_json;
extern crate serde;
extern crate kuchiki;
extern crate gtk;
extern crate gio;
extern crate std;

use curl::easy::Easy;
use kuchiki::traits::*;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WordNotFoundError {
    FromUtf8Error(std::string::FromUtf8Error),
    SerdeError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    CurlError(curl::Error),
}

#[derive(Debug)]
pub enum WikiError {
    WordNotFoundError(self::WordNotFoundError),
}

impl From<std::string::FromUtf8Error> for WordNotFoundError {
    fn from(err: std::string::FromUtf8Error) -> WordNotFoundError {
        WordNotFoundError::FromUtf8Error(err)
    }
}

impl From<serde_json::Error> for WordNotFoundError {
    fn from(err: serde_json::Error) -> WordNotFoundError {
        WordNotFoundError::SerdeError(err)
    }
}
impl From<std::num::ParseIntError> for WordNotFoundError {
    fn from(err: std::num::ParseIntError) -> WordNotFoundError {
        WordNotFoundError::ParseIntError(err)
    }
}

impl From<curl::Error> for WordNotFoundError {
    fn from(err: curl::Error) -> WordNotFoundError {
        WordNotFoundError::CurlError(err)
    }
}

impl fmt::Display for WordNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordNotFoundError::FromUtf8Error(ref err) => write!(f, "FromUtf8Error error: {}", err),
            WordNotFoundError::SerdeError(ref err) =>  write!(f, "SerdeError error: {}", err),
            WordNotFoundError::ParseIntError(ref err) =>  write!(f, "ParseIntError error: {}", err),
            WordNotFoundError::CurlError(ref err) =>  write!(f, "CurlError error: {}", err),
        }
    }
}

impl error::Error for WordNotFoundError {
    fn description(&self) -> &str {
        match *self {
            WordNotFoundError::FromUtf8Error(ref err) => err.description(),
            WordNotFoundError::SerdeError(ref err) => err.description(),
            WordNotFoundError::ParseIntError(ref err) => err.description(),
            WordNotFoundError::CurlError(ref err) =>  err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WordNotFoundError::FromUtf8Error(ref err) => Some(err),
            WordNotFoundError::SerdeError(ref err) => Some(err),
            WordNotFoundError::ParseIntError(ref err) => Some(err),
            WordNotFoundError::CurlError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WikiError::WordNotFoundError(ref err) => write!(f, "WordNotFoundError error: {}", err),
        }
    }
}

impl error::Error for WikiError {
    fn description(&self) -> &str {
        match *self {
            WikiError::WordNotFoundError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WikiError::WordNotFoundError(ref err) => Some(err),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Section {
    toclevel: u8,
    level: String,
    line: String,
    number: String,
    index: String,

    #[serde(default)]
    fromtitle: String,

    byteoffset: Option<u32>,
    anchor: String,
}

#[derive(Serialize, Deserialize)]
struct Parse {
    title: String,
    pageid: u32,

    #[serde(default)]
    sections: Vec<Section>,

    #[serde(default)]
    text: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseWiki {
    parse: Parse,
}

pub const NOMINATIVE    :usize = 0;
pub const GENITIVE      :usize = 1;
pub const DATIVE        :usize = 2;
pub const ACUSATIVE     :usize = 3;
pub const INSTRUMENTAL  :usize = 4;
pub const PREPOSITIONAL :usize = 5;

pub const SINGULAR :usize = 0;
pub const PLURAL   :usize = 1;

const NOMINATIVE_TITLE    : &str = "именительный";
const GENITIVE_TITLE      : &str = "родительный";
const DATIVE_TITLE        : &str = "дательный";
const ACUSATIVE_TITLE     : &str = "винительный";
const INSTRUMENTAL_TITLE  : &str = "творительный";
const PREPOSITIONAL_TITLE : &str = "предложный";

const NOMINATIVE_STRING    : &str = "Nominative";
const GENITIVE_STRING      : &str = "Genitive";
const DATIVE_STRING        : &str = "Dative";
const ACUSATIVE_STRING     : &str = "Acusative";
const INSTRUMENTAL_STRING  : &str = "Instrumental";
const PREPOSITIONAL_STRING : &str = "Prepositional";

fn get_page_info(word: &str) -> Result<String, WordNotFoundError> {
    let mut easy = Easy::new();
    let mut buf = Vec::new();
    let mut url = "https://ru.wiktionary.org/w/api.php?action=parse&redirects&format=json&prop=sections&page=".to_owned();
    url.push_str(word);
    try!(easy.url(&url));
    {
        let mut transfer = easy.transfer();
        try!(transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }));
        try!(transfer.perform());
    }
    match String::from_utf8(buf) {
        Ok(page_info) => Ok(page_info),
        Err(err) => Err(err).map_err(WordNotFoundError::FromUtf8Error),
    }
}

fn get_morphological_section_number(page_info: String) -> Result<u16,WordNotFoundError> {
    let ret: ResponseWiki;
    let mut section_index = 0;

    //ret = serde_json::from_str(&page_info).unwrap();
    ret = try!(serde_json::from_str(&page_info));
    for section in ret.parse.sections {
        if section.line == "Морфологические и синтаксические свойства" {
            //section_index = section.index.parse::<u16>().unwrap();
            section_index = try!(section.index.parse::<u16>());
            break;
        }
    }
    Ok(section_index)
}

fn get_morphological_section(word: String, index: u16) -> Result<String, WordNotFoundError> {
    let mut easy = Easy::new();
    let mut buf = Vec::new();
    let mut url = "https://ru.wiktionary.org/w/api.php?action=parse&prop=text&redirects&format=json&formatversion=2&page=".to_owned();
    url.push_str(&word);
    url.push_str("&section=");
    url.push_str(&index.to_string());

    try!(easy.url(&url));
    {
        let mut transfer = easy.transfer();
        try!(transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }));
        try!(transfer.perform());
    }
    let json_str = try!(String::from_utf8(buf));
    let response: ResponseWiki = try!(serde_json::from_str(&json_str));
    Ok(response.parse.text)
}

fn get_title(case: usize) -> String {
    match case {
        NOMINATIVE => NOMINATIVE_TITLE.to_owned(),
        GENITIVE   => GENITIVE_TITLE.to_owned(),
        DATIVE     => DATIVE_TITLE.to_owned(),
        ACUSATIVE  => ACUSATIVE_TITLE.to_owned(),
        INSTRUMENTAL  => INSTRUMENTAL_TITLE.to_owned(),
        PREPOSITIONAL => PREPOSITIONAL_TITLE.to_owned(),
        //todo: see if there's a better way of handling this
        _            => NOMINATIVE_TITLE.to_owned(),
    }
}

pub fn get_case_name(case: usize) -> String {
    match case {
        NOMINATIVE => NOMINATIVE_STRING.to_owned(),
        GENITIVE   => GENITIVE_STRING.to_owned(),
        DATIVE     => DATIVE_STRING.to_owned(),
        ACUSATIVE  => ACUSATIVE_STRING.to_owned(),
        INSTRUMENTAL  => INSTRUMENTAL_STRING.to_owned(),
        PREPOSITIONAL => PREPOSITIONAL_STRING.to_owned(),
        _            => "".to_owned(),
    }
}

// Extract table from the page html
fn extract_declension_table(content: String) -> Vec<Vec<String>> {
    let mut declension_table = vec![vec![String::new(); 2]; 6];
    //println!("extract dec = {}", content);
    let doc = kuchiki::parse_html().one(content.replace("\n", "").replace("<br />", " / "));

    for case in [NOMINATIVE, GENITIVE, DATIVE, ACUSATIVE, INSTRUMENTAL, PREPOSITIONAL].iter() {
        let mut selector = "td a[title='".to_owned();
        selector.push_str(&get_title(*case));
        selector.push_str("']");

        let anchor = doc.select(&selector).unwrap().next().unwrap();
        let tr = anchor.as_node().parent().unwrap().parent().unwrap();

        declension_table[*case][SINGULAR] = tr.children().nth(1).unwrap().text_contents();
        declension_table[*case][PLURAL] = tr.children().nth(2).unwrap().text_contents();
        //TODO: Remove those unwraps above
    }
    /*for case in [NOMINATIVE, GENITIVE, DATIVE, ACUSATIVE, INSTRUMENTAL, PREPOSITIONAL].iter() {
        println!("Case {} singular: {}", get_title(*case), declension_table[*case][SINGULAR]);
        println!("Case {} plural: {}", get_title(*case), declension_table[*case][PLURAL]);
    }*/
    declension_table
}

pub fn get_declension_table(word: &str) -> Result<Vec<Vec<String>>,WikiError> {
    println!("getting table of {}",word);

    let page_info =
        match get_page_info(word) {
            Ok(info) => info,
            Err(err) => return Err(err).map_err(WikiError::WordNotFoundError),
        };

    let section_index =
        match get_morphological_section_number(page_info) {
            Ok(index) => index,
            Err(err)  => return Err(err).map_err(WikiError::WordNotFoundError),
        };
    let section_content =
        match get_morphological_section(word.to_owned(), section_index) {
            Ok(section) => section,
            Err(err)    => return Err(err).map_err(WikiError::WordNotFoundError),
        };
    Ok(extract_declension_table(section_content))
}
