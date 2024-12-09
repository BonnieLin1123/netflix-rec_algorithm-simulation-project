mod read_file;
mod analyze_file;
mod build_graph;

use read_file::Movie;
use build_graph::{build_graph, compute_pagerank, calculate_similarity};
use rand::seq::SliceRandom; // For random sampling
use rand::thread_rng;

fn main() {
    let file_path = "netflix_titles 2.csv"; // Replace with your actual file path
    let movies = Movie::read_and_clean(file_path);

    if movies.is_empty() {
        println!("No valid movies were loaded from the file.");
        return;
    }

    // Hardcoded movie title
    let input_title = "Dick Johnson Is Dead";

    // Similarity Logic (Unchanged)
    if let Some(selected_movie) = movies.iter().find(|m| m.title == input_title) {
        println!("Comparing '{}' to all other movies...\n", selected_movie.title);

        let mut similarities = Vec::new();
        for movie in &movies {
            if movie.title != selected_movie.title {
                let similarity = calculate_similarity(selected_movie, movie);
                similarities.push((movie.title.clone(), similarity));
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("Top 5 similar movies:");
        for (title, score) in similarities.iter().take(5) {
            println!("{} \n (Similarity Score: {:.4})", title, score);
        }
    } else {
        println!("Movie '{}' not found in the dataset.", input_title);
    }

    // PageRank Logic (Modified for Optimization)
    println!("\nRunning PageRank...");

    // Random Sampling for PageRank
    let sample_size = 2000; // Randomly choose movies for PageRank
    let mut rng = thread_rng();
    let sampled_movies: Vec<_> = movies
        .choose_multiple(&mut rng, sample_size.min(movies.len()))
        .cloned()
        .collect();

    let threshold = 0.5; // Higher threshold for smaller graph
    let graph = build_graph(&sampled_movies, threshold);

    let damping_factor = 0.85; // Standard damping factor
    let iterations = 100; // Reduced iterations for faster computation
    let ranks = compute_pagerank(&graph, damping_factor, iterations);

    // Print Top 5 Movies by PageRank
    println!("\nTop 5 movies by PageRank: ");
    let mut ranked_movies: Vec<_> = ranks.iter().collect();
    ranked_movies.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for (title, rank) in ranked_movies.iter().take(5) {
        println!("{}: \n(PageRank Score: {:.5})" , title, rank);
    }
}
