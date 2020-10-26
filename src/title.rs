use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::{HashMap};
use crate::rating::Rating;
use crate::genre::Genre;
use std::fmt;

#[derive(Debug)]
pub enum TitleKind {
  Movie,
}

impl TitleKind {
  pub fn new(kind: &str) -> Option<TitleKind> {
    match kind {
      "movie" => Some(TitleKind::Movie),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub struct Title {
  pub id: String,
  pub kind: TitleKind,
  pub primary_title: String,
  pub start_year: u16,
  pub runtime: u8,
  pub rating: Option<Rating>,
  pub genres: Vec<Genre>,
  pub directors: Vec<String>,
  pub actors: Vec<String>,
  pub actresses: Vec<String>,
}

impl Title {
  pub fn new(line: &str) -> Option<Title> {
    let parts: Vec<&str> = line.split("\t").collect();
    
    Some(Title {
      id: parts.get(0)?.to_string(),
      kind: TitleKind::new(parts.get(1)?)?,
      primary_title: parts.get(2)?.to_string().replace('"', "\\\""),
      start_year: parts.get(5)?.parse().ok()?,
      runtime: parts.get(7)?.parse().ok()?,
      genres: parts.get(8)?.split(",").filter_map(|genre| genre.parse().ok()).collect(),
      rating: None,
      actors: vec![],
      actresses: vec![],
      directors: vec![],
    })
  }
}

impl fmt::Display for Title {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let genres: Vec<String> = self.genres.iter()
        .map(|genre| format!("imdb:{:?}", genre))
        .collect();
      
      write!(f, "\
        imt:{} rdf:type imdb:Film ; \
        imdb:id \"{}\" ; \
        imdb:title \"{}\" ; \
        imdb:year \"{}\"^^xsd:integer ; \
        imdb:genre {} ; \
        ", self.id, self.id, self.primary_title, self.start_year, genres.join(", "))?;

      if self.actors.len() > 0 {
        let actors: Vec<String> = self.actors.iter()
          .map(|actor| format!("imn:{}", actor))
          .collect();

        write!(f, "imdb:actor {} ; ", actors.join(", "))?;
      }

      if self.actresses.len() > 0 {
        let actresses: Vec<String> = self.actresses.iter()
          .map(|actress| format!("imn:{}", actress))
          .collect();

        write!(f, "imdb:actress {} ; ", actresses.join(", "))?;
      }

      if self.directors.len() > 0 {
        let directors: Vec<String> = self.directors.iter()
          .map(|director| format!("imn:{}", director))
          .collect();

        write!(f, "imdb:director {} ; ", directors.join(", "))?;
      }

      if let Some(rating) = &self.rating {
        write!(f, "\
          imdb:rating \"{}\"^^xsd:decimal ; \
          imdb:votes \"{}\"^^xsd:integer ; \
          ", rating.average, rating.votes)?;
      }

      write!(f, "imdb:runtime \"{}\"^^xsd:integer .", self.runtime)?;

      Ok(())
  }
}

pub type Titles = HashMap<String, Title>;

pub fn basics() -> Result<Titles> {
  let f = File::open("title.basics.tsv")?;
  let reader = BufReader::new(f);

  let titles: Titles = reader.lines()
    .skip(1)
    .filter_map(|line| {
      let line = line.ok()?;
      let title = Title::new(&line)?;
      Some((title.id.to_string(), title))
    })
    .collect();

  Ok(titles)
}