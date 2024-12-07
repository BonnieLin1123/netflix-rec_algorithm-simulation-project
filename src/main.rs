use std::collections::{HashMap, HashSet};
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
struct Movie {
    title: String,
    movie_type: String,
    director: String,
    cast: String,
    country: String,
    release_year: u32,
    duration: String,
    listed_in: String,
}

impl Movie {
    // Function to read and clean data from the CSV file
    fn read_and_clean(file_path: &str) -> Vec<Movie> {
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
                    eprintln!(
                        "Skipping invalid record at position {:?}: {:?}",
                        rdr.position(),
                        err
                    );
                }
            }
        }

        movies
    }
}

// Analyze type similarity
fn analyze_type(type1: &str, type2: &str) -> f32 {
    if type1.is_empty() || type2.is_empty() {
        0.0
    } else if type1 == type2 {
        1.0
    } else {
        0.0
    }
}

// Analyze director similarity
fn analyze_director(director1: &str, director2: &str) -> f32 {
    if director1.is_empty() || director2.is_empty() {
        0.0
    } else if director1 == director2 {
        1.0
    } else {
        0.0
    }
}

// Analyze genres similarity
fn analyze_genres(genres1: &str, genres2: &str) -> f32 {
    if genres1.is_empty() || genres2.is_empty() {
        0.0
    } else {
        let genres1: HashSet<&str> = genres1.split(',').map(|s| s.trim()).collect();
        let genres2: HashSet<&str> = genres2.split(',').map(|s| s.trim()).collect();

        let intersection: usize = genres1.intersection(&genres2).count();
        let union: usize = genres1.union(&genres2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

// Analyze release year similarity
fn analyze_release_year(year1: u32, year2: u32) -> f32 {
    if year1 == 0 || year2 == 0 {
        0.0
    } else {
        let diff = (year1 as i32 - year2 as i32).abs();
        if diff <= 5 {
            1.0
        } else if diff <= 10 {
            0.5
        } else {
            0.0
        }
    }
}

// Analyze cast similarity using Jaccard Similarity
fn analyze_cast(cast1: &str, cast2: &str) -> f32 {
    if cast1.is_empty() || cast2.is_empty() {
        0.0
    } else {
        let cast1: HashSet<&str> = cast1.split(',').map(|s| s.trim()).collect();
        let cast2: HashSet<&str> = cast2.split(',').map(|s| s.trim()).collect();

        let intersection: usize = cast1.intersection(&cast2).count();
        let union: usize = cast1.union(&cast2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

// Analyze country similarity
fn analyze_country(country1: &str, country2: &str) -> f32 {
    if country1.is_empty() || country2.is_empty() {
        0.0
    } else if country1 == country2 {
        1.0
    } else {
        0.0
    }
}

// Analyze duration similarity
fn analyze_duration(duration1: &str, duration2: &str) -> f32 {
    let parse_duration = |duration: &str| -> Option<i32> {
        if duration.ends_with("min") {
            duration.replace("min", "").trim().parse::<i32>().ok()
        } else if duration.contains("Season") {
            duration.split_whitespace().next()?.parse::<i32>().ok().map(|x| x * 60 * 24)
        } else {
            None
        }
    };

    let dur1 = parse_duration(duration1);
    let dur2 = parse_duration(duration2);

    match (dur1, dur2) {
        (Some(d1), Some(d2)) => {
            let difference = (d1 - d2).abs();
            if difference <= 30 {
                1.0
            } else if difference <= 60 {
                0.5
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

// Calculate overall similarity score between two movies
fn calculate_similarity(movie1: &Movie, movie2: &Movie) -> f32 {
    let mut score = 0.0;
    score += analyze_type(&movie1.movie_type, &movie2.movie_type);
    score += analyze_director(&movie1.director, &movie2.director);
    score += analyze_genres(&movie1.listed_in, &movie2.listed_in);
    score += analyze_release_year(movie1.release_year, movie2.release_year);
    score += analyze_cast(&movie1.cast, &movie2.cast);
    score += analyze_country(&movie1.country, &movie2.country);
    score += analyze_duration(&movie1.duration, &movie2.duration);
    score
}

// Build the graph based on similarity scores
fn build_graph(movies: &Vec<Movie>, threshold: f32) -> HashMap<String, Vec<(String, f32)>> {
    let mut graph = HashMap::new();

    for i in 0..movies.len() {
        let mut edges = Vec::new();
        for j in 0..movies.len() {
            if i != j {
                let similarity = calculate_similarity(&movies[i], &movies[j]);
                if similarity >= threshold {
                    edges.push((movies[j].title.clone(), similarity));
                }
            }
        }

        // Normalize edges
        let total_weight: f32 = edges.iter().map(|(_, weight)| weight).sum();
        let normalized_edges = edges
            .into_iter()
            .map(|(title, weight)| (title, weight / total_weight))
            .collect();

        graph.insert(movies[i].title.clone(), normalized_edges);
    }

    graph
}

// Compute PageRank scores
fn compute_pagerank(graph: &HashMap<String, Vec<(String, f32)>>, damping: f32, iterations: usize) -> HashMap<String, f32> {
    let num_nodes = graph.len() as f32;
    let initial_rank = 1.0 / num_nodes;
    let mut ranks: HashMap<String, f32> = graph.keys().map(|node| (node.clone(), initial_rank)).collect();

    for _ in 0..iterations {
        let mut new_ranks = HashMap::new();
        for (node, edges) in graph {
            let mut rank_sum = 0.0;
            for (neighbor, weight) in edges {
                rank_sum += ranks[neighbor] * weight;
            }
            new_ranks.insert(node.clone(), (1.0 - damping) / num_nodes + damping * rank_sum);
        }
        ranks = new_ranks;
    }

    ranks
}

// Recommend top N movies based on PageRank scores
fn recommend_movies(ranks: &HashMap<String, f32>, top_n: usize) -> Vec<(String, f32)> {
    let mut ranked_movies: Vec<_> = ranks.iter().collect();
    ranked_movies.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    ranked_movies.into_iter().rev().take(top_n).map(|(title, rank)| (title.clone(), *rank)).collect()
}

// Main function
fn main() {
    let file_path = "netflix_titles 2.csv"; // Replace with your actual CSV file path
    let movies = Movie::read_and_clean(file_path);

    // Build the graph
    let threshold = 0.3; // Adjust threshold for similarity
    let graph = build_graph(&movies, threshold);

    // Compute PageRank
    let damping_factor = 0.85;
    let iterations = 100;
    let ranks = compute_pagerank(&graph, damping_factor, iterations);

    // Recommend top 5 movies
    let recommendations = recommend_movies(&ranks, 5);
    println!("Top 5 recommended movies:");
    for (title, rank) in recommendations {
        println!("{} (PageRank Score: {:.4})", title, rank);
    }
}