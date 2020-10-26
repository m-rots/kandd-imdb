use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use kandd::title::{basics};
use kandd::rating::add_ratings;
use kandd::principals::add_principals;
use kandd::names::get_names;

const HEADER: &'static str = "\
@prefix imt: <https://www.imdb.com/title/> .
@prefix imn: <https://www.imdb.com/name/> .
@prefix imdb: <https://www.imdb.com/interfaces/> .
@prefix media: <https://m-rots.com/media#> .

@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<https://m-rots.com/media> rdf:type owl:Ontology .

imdb:Film rdf:type owl:Class .
imdb:Genre rdf:type owl:Class .
imdb:Name rdf:type owl:Class .

imdb:id rdf:type owl:DatatypeProperty .
imdb:title rdf:type owl:DatatypeProperty .
imdb:year rdf:type owl:DatatypeProperty .
imdb:genre rdf:type owl:ObjectProperty .
imdb:runtime rdf:type owl:DatatypeProperty .

imdb:rating rdf:type owl:DatatypeProperty .
imdb:votes rdf:type owl:DatatypeProperty .

imdb:name rdf:type owl:DatatypeProperty .
imdb:birthYear rdf:type owl:DatatypeProperty .
imdb:deathYear rdf:type owl:DatatypeProperty .

imdb:actor rdf:type owl:ObjectProperty .
imdb:actress rdf:type owl:ObjectProperty .
imdb:director rdf:type owl:ObjectProperty .
imdb:writer rdf:type owl:ObjectProperty .

imdb:Action rdf:type imdb:Genre .
imdb:Adventure rdf:type imdb:Genre .
imdb:Animation rdf:type imdb:Genre .
imdb:Biography rdf:type imdb:Genre .
imdb:Comedy rdf:type imdb:Genre .
imdb:Crime rdf:type imdb:Genre .
imdb:Documentary rdf:type imdb:Genre .
imdb:Drama rdf:type imdb:Genre .
imdb:Family rdf:type imdb:Genre .
imdb:Fantasy rdf:type imdb:Genre .
imdb:FilmNoir rdf:type imdb:Genre .
imdb:History rdf:type imdb:Genre .
imdb:Horror rdf:type imdb:Genre .
imdb:Music rdf:type imdb:Genre .
imdb:Musical rdf:type imdb:Genre .
imdb:Mystery rdf:type imdb:Genre .
imdb:Romance rdf:type imdb:Genre .
imdb:ScienceFiction rdf:type imdb:Genre .
imdb:Sport rdf:type imdb:Genre .
imdb:Superhero rdf:type imdb:Genre .
imdb:Thriller rdf:type imdb:Genre .
imdb:War rdf:type imdb:Genre .
imdb:Western rdf:type imdb:Genre .
";

fn main() -> Result<()> {
  println!("Loading basics");
  let titles = basics()?;

  println!("Loading ratings");
  let titles = add_ratings(titles)?;

  println!("Loading principals");
  let (titles, names) = add_principals(titles)?;

  println!("Loading names");
  let names = get_names(names)?;

  let out = File::create("imdb.ttl")?;
  let mut buf = BufWriter::new(out);
  writeln!(buf, "{}", HEADER)?;

  let mut title_count: usize = 0;
  let mut name_count: usize = 0;

  titles.iter()
    .for_each(|(_, title)| {
      title_count += 1;
      writeln!(buf, "{}", title).unwrap();
      println!("{:#?}", title)
    });

  names.iter()
    .for_each(|name| {
      name_count += 1;
      writeln!(buf, "{}", name).unwrap();
      println!("{:#?}", name)
    });

  println!("number of titles: {}", title_count);
  println!("number of names: {}", name_count);

  Ok(())
}


