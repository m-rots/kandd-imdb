<p align="center"><img width=450 src="banner.svg" /></p>

## Introduction

As part of my Knowledge & Data university project, I had to create a parser which transformed datasets from the Internet Movie Database (IMDb) to a turtle file.

While I have never used Rust before in one of my projects, I decided to try working with it as both a challenge and because I thought Rust would be the fastest tool available.

The parser combines the following [datasets](https://www.imdb.com/interfaces/) (~3,1 GB) into a turtle file (~7 MB):

- `title.basics`
- `title.principals`
- `title.ratings`
- `name.basics`

### Filters

IMDb contains a lot of titles (~7 million) of which many are not relevant to the project. Therefore, the following filters are applied to reduce the number of titles between the different parsing phases:

1. Only add titles of type `movie` from `title.basics`.
2. Remove titles which do not have a rating available in `title.ratings`.
3. Remove titles of which the rating has been determined by less than 5000 votes.
4. Only add names from `name.basics` when they appear as an actor, actress or director for a title determined by `title.principals`.

These filters combined reduced the total number of titles to a mere ~13k. 

### Properties

The following properties where collected for each title (`tconst`):

- `tconst` from `title.basics`.
- `primaryTitle` from `title.basics`.
- `startYear` from `title.basics`.
- `runtimeMinutes` from `title.basics`.
- `genres` from `title.basics`.
- `averageRating` from `title.ratings`.
- `numVotes` from `title.ratings`.
- `nconst` from `title.principals`. (multiple)
- `category` from `title.principals`. (multiple)

For each name (`nconst`), the following additional properties were collected:

- `primaryName` from `name.basics`.
- `birthYear` from `name.basics`.
- `deathYear` from `name.basics`. (if available)

## Running the CLI

So you want to give it a spin? Make sure you have prepared your system:

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. [Download the datasets](https://www.imdb.com/interfaces/) from IMDb.
4. Unpack the datasets and place them next to the `README.MD` file.
5. Make sure the datasets are named correctly: `title.basics.tsv`, `title.principals.tsv`, `title.ratings.tsv` and `name.basics.tsv`.
6. Run the following command and watch the magic unfold: `cargo run --release`.

If you executed all the above steps correctly, then you should now see an `imdb.ttl` file next to the `README.MD` file.