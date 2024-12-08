use csv::ReaderBuilder;

#[derive(Debug, Clone)]
pub struct Movie {
    pub title: String,
    pub movie_type: String,
    pub director: String,
    pub country: String,
    pub release_year: u32,
    pub rating: String,
    pub listed_in: String,
}

impl Movie {
    pub fn read_and_clean(file_path: &str) -> Vec<Movie> {
        let mut movies = Vec::new();
        let mut rdr = ReaderBuilder::new()
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)
            .expect("Could not open file");

        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let movie = Movie {
                        title: record.get(2).unwrap_or("").to_string(),
                        movie_type: record.get(1).unwrap_or("").to_string(),
                        director: record.get(3).unwrap_or("").to_string(),
                        country: record.get(5).unwrap_or("").to_string(),
                        release_year: record.get(7).unwrap_or("").parse().unwrap_or(0),
                        rating: record.get(8).unwrap_or("").to_string(),
                        listed_in: record.get(10).unwrap_or("").to_string(),
                    };
                    movies.push(movie);
                }
                Err(err) => {
                    eprintln!("Skipping invalid record: {:?}", err);
                }
            }
        }

        movies
    }
}
