use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Movie Struct
#[derive(Debug, Clone)]
struct Movie {
    show_id: String,
    title: String,
    director: Option<String>,
    cast: Vec<String>,
    country: Option<String>,
    release_year: u16,
    listed_in: Vec<String>,
    rating: Option<String>,
}

impl Movie {
    fn from_csv_row(row: &str) -> Option<Movie> {
        let columns: Vec<&str> = row.split(',').collect();
        if columns.len() < 12 {
            println!("Skipping invalid row: {}", row);
            return None;
        }

        let show_id = columns[0].to_string();
        let title = columns[2].to_string();
        let director = if !columns[3].is_empty() { Some(columns[3].to_string()) } else { None };
        let cast = columns[4]
            .split('|')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        let country = if !columns[5].is_empty() { Some(columns[5].to_string()) } else { None };
        let release_year = columns[7].parse().unwrap_or(0); // Default to 0 for malformed year
        let listed_in = columns[10]
            .split('|')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        let rating = if !columns[8].is_empty() { Some(columns[8].to_string()) } else { None };

        Some(Movie {
            show_id,
            title,
            director,
            cast,
            country,
            release_year,
            listed_in,
            rating,
        })
    }
}

// Data Reading
fn read_csv(file_path: &str) -> Vec<Movie> {
    let mut movies = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            if let Some(movie) = Movie::from_csv_row(&line) {
                movies.push(movie);
            }
        }
    }
    movies
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Graph Struct
struct GraphData {
    graph: Graph<String, u32>,
    title_to_index: HashMap<String, NodeIndex>,
    movies: HashMap<String, Movie>,
}

impl GraphData {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            title_to_index: HashMap::new(),
            movies: HashMap::new(),
        }
    }

    fn add_movie(&mut self, movie: Movie) {
        if !self.title_to_index.contains_key(&movie.title) {
            let index = self.graph.add_node(movie.title.clone());
            self.title_to_index.insert(movie.title.clone(), index);
            self.movies.insert(movie.title.clone(), movie);
        }
    }

    fn add_edges(&mut self) {
        let titles: Vec<String> = self.movies.keys().cloned().collect();
        for i in 0..titles.len() {
            for j in (i + 1)..titles.len() {
                let movie1 = &self.movies[&titles[i]];
                let movie2 = &self.movies[&titles[j]];

                if let (Some(&index1), Some(&index2)) = (
                    self.title_to_index.get(&movie1.title),
                    self.title_to_index.get(&movie2.title),
                ) {
                    let weight = self.calculate_similarity(movie1, movie2);
                    if weight > 0 {
                        self.graph.add_edge(index1, index2, weight);
                    }
                }
            }
        }
    }

    fn calculate_similarity(&self, movie1: &Movie, movie2: &Movie) -> u32 {
        let shared_genres = movie1
            .listed_in
            .iter()
            .filter(|g| movie2.listed_in.contains(g))
            .count() as u32;

        let shared_cast = movie1
            .cast
            .iter()
            .filter(|c| movie2.cast.contains(c))
            .count() as u32;

        let shared_director = if movie1.director == movie2.director { 1 } else { 0 };

        let same_country = if movie1.country == movie2.country { 1 } else { 0 };

        let release_year_diff = (movie1.release_year as i32 - movie2.release_year as i32).abs();
        let release_year_similarity = if release_year_diff <= 5 { 1 } else { 0 };

        (shared_genres * 3) + (shared_cast * 2) + shared_director + same_country + release_year_similarity
    }

    fn recommend(&self, title: &str, top_n: usize) -> Vec<String> {
        if let Some(&start_index) = self.title_to_index.get(title) {
            let mut recommendations = Vec::new();

            for neighbor in self.graph.neighbors(start_index) {
                let weight = self.graph.edge_weight(start_index, neighbor).unwrap_or(&0);
                recommendations.push((neighbor, *weight));
            }

            recommendations.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by weight descending
            recommendations
                .into_iter()
                .take(top_n)
                .map(|(index, _)| self.graph[index].clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

fn main() {
    let file_path = "path/to/your/dataset.csv";
    println!("Reading data from {}", file_path);

    let movies = read_csv(file_path);
    println!("Loaded {} movies", movies.len());

    let mut graph_data = GraphData::new();
    for movie in movies {
        graph_data.add_movie(movie);
    }
    graph_data.add_edges();

    println!("Graph construction completed.");

    let title = "Blood & Water"; // Example movie title
    let recommendations = graph_data.recommend(title, 5);

    println!("Recommendations for '{}':", title);
    for rec in recommendations {
        println!("- {}", rec);
    }
}