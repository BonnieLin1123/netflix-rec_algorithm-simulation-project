use csv::ReaderBuilder;

#[derive(Debug, Clone)]
pub struct Movie {
    pub title: String,
    pub movie_type: String,
    pub director: String,
    pub cast: String,
    pub country: String,
    pub release_year: u32,
    pub duration: String,
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
                        title: String::from_utf8_lossy(record.get(1).unwrap_or("").as_bytes()).to_string(),
                        movie_type: String::from_utf8_lossy(record.get(0).unwrap_or("").as_bytes()).to_string(),
                        director: String::from_utf8_lossy(record.get(2).unwrap_or("").as_bytes()).to_string(),
                        cast: String::from_utf8_lossy(record.get(3).unwrap_or("").as_bytes()).to_string(),
                        country: String::from_utf8_lossy(record.get(4).unwrap_or("").as_bytes()).to_string(),
                        release_year: record.get(6).unwrap_or("").parse().unwrap_or(0),
                        duration: String::from_utf8_lossy(record.get(8).unwrap_or("").as_bytes()).to_string(),
                        listed_in: String::from_utf8_lossy(record.get(9).unwrap_or("").as_bytes()).to_string(),
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
