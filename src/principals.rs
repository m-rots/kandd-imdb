use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use crate::title::Titles;
use crate::names::Names;

#[derive(Debug)]
pub enum Principal {
  Actress(String),
  Actor(String),
  Director(String),
  Writer(String),
}

impl Principal {
  pub fn new(id: &str, principal: &str) -> Option<Principal> {
    match principal {
      "actress" => Some(Principal::Actress(id.to_string())),
      "actor" => Some(Principal::Actor(id.to_string())),
      "director" => Some(Principal::Director(id.to_string())),
      "writer" => Some(Principal::Writer(id.to_string())),
      _ => None,
    }
  }
}

pub fn add_principals(mut titles: Titles) -> Result<(Titles, Names)> {
  let f = File::open("title.principals.tsv")?;
  let reader = BufReader::new(f);

  let mut names = Names::new();

  reader.lines()
    .skip(1)
    .filter_map(|line| {
      let line = line.ok()?;
      let parts: Vec<&str> = line.split("\t").collect();
      let title_id = parts.get(0)?.to_string();

      let principal = Principal::new(parts.get(2)?, parts.get(3)?)?;
      Some((title_id, principal))
    })
    .for_each(|(title_id, principal)| {
      if let Some(title) = titles.get_mut(&title_id) {
        match principal {
          Principal::Actor(name) => {
            names.insert(name.clone());
            title.actors.push(name);
          },
          Principal::Actress(name) => {
            names.insert(name.clone());
            title.actresses.push(name)
          },
          Principal::Director(name) => {
            names.insert(name.clone());
            title.directors.push(name)
          },
          _ => (),
        }
      }      
    });

  Ok((titles, names))
}