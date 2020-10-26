use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashSet;
use std::fmt;

pub type Names = HashSet<String>;

#[derive(Debug)]
pub struct Name {
  pub id: String,
  pub primary_name: String,
  pub birth_year: u16,
  pub death_year: Option<u16>,
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\
      imn:{} rdf:type imdb:Name ; \
      imdb:name \"{}\" ; \
      imdb:birthYear \"{}\"^^xsd:integer ; \
      ", self.id, self.primary_name, self.birth_year)?;

    if let Some(death_year) = &self.death_year {
      write!(f, "imdb:deathYear \"{}\"^^xsd:integer ; ", death_year)?;
    }

    write!(f, "imdb:id \"{}\" .", self.id)?;

    Ok(())
  }
}

pub fn get_names(unique_names: Names) -> Result<Vec<Name>> {
  let f = File::open("name.basics.tsv")?;
  let reader = BufReader::new(f);

  let result: Vec<Name> = reader.lines()
    .skip(1)
    .filter_map(|line| {
      let line = line.ok()?;
      let parts: Vec<&str> = line.split("\t").collect();

      Some(Name {
        id: parts.get(0)?.to_string(),
        primary_name: parts.get(1)?.to_string(),
        birth_year: parts.get(2)?.parse().ok()?,
        death_year: parts.get(3)?.parse().ok(),
      })
    })
    .filter(|name| {
      unique_names.contains(&name.id)
    })
    .collect();

  Ok(result)
}