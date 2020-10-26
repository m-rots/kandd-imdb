#[derive(Debug)]
pub enum Genre {
  Action,
  Adventure,
  Animation,
  Biography,
  Comedy,
  Crime,
  Documentary,
  Drama,
  Family,
  Fantasy,
  FilmNoir,
  History,
  Horror,
  Music,
  Musical,
  Mystery,
  Romance,
  ScienceFiction,
  Sport,
  Superhero,
  Thriller,
  War,
  Western,
}

impl std::str::FromStr for Genre {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "Action" => Ok(Genre::Action),
      "Adventure" => Ok(Genre::Adventure),
      "Animation" => Ok(Genre::Animation),
      "Biography" => Ok(Genre::Biography),
      "Comedy" => Ok(Genre::Comedy),
      "Crime" => Ok(Genre::Crime),
      "Documentary" => Ok(Genre::Documentary),
      "Drama" => Ok(Genre::Drama),
      "Family" => Ok(Genre::Family),
      "Fantasy" => Ok(Genre::Fantasy),
      "Film-Noir" => Ok(Genre::FilmNoir),
      "History" => Ok(Genre::History),
      "Horror" => Ok(Genre::Horror),
      "Music" => Ok(Genre::Music),
      "Musical" => Ok(Genre::Musical),
      "Mystery" => Ok(Genre::Mystery),
      "Romance" => Ok(Genre::Romance),
      "Sci-Fi" => Ok(Genre::ScienceFiction),
      "Sport" => Ok(Genre::Sport),
      "Superhero" => Ok(Genre::Superhero),
      "Thriller" => Ok(Genre::Thriller),
      "War" => Ok(Genre::War),
      "Western" => Ok(Genre::Western),
      _ => Err(format!("'{}' is not a valid value for Genre", input)),
    }
  }
}