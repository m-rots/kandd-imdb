use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use crate::title::{Titles};

#[derive(Debug)]
pub struct Rating {
  pub average: f64,
  pub votes: usize,
}

struct TitleRating {
  title: String,
  rating: Rating,
}

impl TitleRating {
  fn new(line: &str) -> Option<TitleRating> {
    let parts: Vec<&str> = line.split("\t").collect();

    Some(TitleRating {
      title: parts.get(0)?.to_string(),
      
      rating: Rating {
        average: parts.get(1)?.parse().ok()?,
        votes: parts.get(2)?.parse().ok()?,
      },
    })
  }
}

pub fn add_ratings(mut titles: Titles) -> Result<Titles> {
  let f = File::open("title.ratings.tsv")?;
  let reader = BufReader::new(f);

  let new_titles: Titles = reader.lines()
    .skip(1)
    .filter_map(|line| {
      TitleRating::new(&line.ok()?)
    })
    .filter_map(|rating| {
      let mut title = titles.remove(&rating.title)?;
      title.rating = Some(rating.rating);
      Some((rating.title, title))
    })
    .filter(|(_, title)| {
      if let Some(rating) = &title.rating {
        return rating.votes >= 5000
      }

      false
    })
    .collect();

  Ok(new_titles)
}