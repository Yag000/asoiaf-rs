use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct House {
    pub url: String,
    pub name: String,
    pub region: String,

    #[serde(rename = "coatOfArms")]
    pub coat_of_arms: String,

    pub words: String,
    pub titles: Vec<String>,
    pub seats: Vec<String>,

    #[serde(rename = "currentLord")]
    pub current_lord: String,

    pub heir: String,
    pub overlord: String,
    pub founded: String,
    pub founder: String,

    #[serde(rename = "diedOut")]
    pub died_out: String,

    #[serde(rename = "ancestralWeapons")]
    pub ancestral_weapons: Vec<String>,

    #[serde(rename = "cadetBranches")]
    pub cadet_branches: Vec<String>,

    #[serde(rename = "swornMembers")]
    pub sworn_members: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Character {
    pub url: String,
    pub nanme: String,
    pub gender: Option<String>,
    pub culture: Option<String>,
    pub born: Option<String>,
    pub died: Option<String>,
    pub titles: Vec<String>,
    pub aliases: Vec<String>,
    pub father: Option<String>,
    pub mother: Option<String>,
    pub spouse: Option<String>,
    pub allegiances: Vec<String>,
    pub books: Vec<String>,

    #[serde(rename = "povBooks")]
    pub pov_books: Vec<String>,

    #[serde(rename = "tvSeries")]
    pub tv_series: Vec<String>,

    #[serde(rename = "playedBy")]
    pub played_by: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Book {
    pub url: String,
    pub name: String,
    pub isbn: String,
    pub authors: Vec<String>,

    #[serde(rename = "numberOfPages")]
    pub number_of_pages: u32,

    pub publisher: String,
    pub country: String,

    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub released: String,
    pub characters: Vec<String>,

    #[serde(rename = "povCharacters")]
    pub pov_characters: Vec<String>,
}
