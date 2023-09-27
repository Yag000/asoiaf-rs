use super::ToUrl;

pub enum BookFilter {
    Name(String),
    FromReleaseDate(String),
    ToReleaseDate(String),
}

impl ToUrl for BookFilter {
    fn to_url(&self) -> String {
        match self {
            BookFilter::Name(name) => format!("name={name}",),
            BookFilter::FromReleaseDate(date) => format!("fromReleaseDate={date}"),
            BookFilter::ToReleaseDate(date) => format!("toReleaseDate={date}"),
        }
    }
}

pub enum CharacterFilter {
    Name(String),
    Gender(String),
    Culture(String),
    Born(String),
    Died(String),
    IsAlive(bool),
}

impl ToUrl for CharacterFilter {
    fn to_url(&self) -> String {
        match self {
            CharacterFilter::Name(name) => format!("name={name}"),
            CharacterFilter::Gender(gender) => format!("gender={gender}"),
            CharacterFilter::Culture(culture) => format!("culture={culture}"),
            CharacterFilter::Born(born) => format!("born={born}"),
            CharacterFilter::Died(died) => format!("died={died}"),
            CharacterFilter::IsAlive(is_alive) => format!("isAlive={is_alive}"),
        }
    }
}

pub enum HouseFilter {
    Name(String),
    Region(String),
    Words(String),
    HasWords(bool),
    HasTitles(bool),
    HasSeats(bool),
    HasDiedOut(bool),
    HasAncestralWeapons(bool),
}

impl ToUrl for HouseFilter {
    fn to_url(&self) -> String {
        match self {
            HouseFilter::Name(name) => format!("name={name}"),
            HouseFilter::Region(name) => format!("region={name}"),
            HouseFilter::Words(name) => format!("words={name}"),
            HouseFilter::HasWords(name) => format!("hasWords={name}"),
            HouseFilter::HasTitles(name) => format!("hasTitles={name}"),
            HouseFilter::HasSeats(name) => format!("hasSeats={name}"),
            HouseFilter::HasDiedOut(name) => format!("hasDiedOut={name}"),
            HouseFilter::HasAncestralWeapons(name) => format!("hasAncestralWeapons={name}"),
        }
    }
}
