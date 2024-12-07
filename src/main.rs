mod read_file;
mod analyze_file;
mod build_graph;

use read_file::Movie;
use build_graph::{build_graph, compute_pagerank};

fn main() {
    let file_path = "netflix_titles 2.csv"; // Update to the correct path
    let movies = Movie::read_and_clean(file_path);

    // Build the graph
    let threshold = 0.3; // Adjust threshold for similarity
    let graph = build_graph(&movies, threshold);

    // Compute PageRank
    let damping_factor = 0.85;
    let iterations = 100;
    let ranks = compute_pagerank(&graph, damping_factor, iterations);

    // Recommend top 5 movies
    let recommendations: Vec<_> = ranks.iter()
        .take(5)
        .map(|(title, rank)| (title.clone(), *rank))
        .collect();

    println!("Top 5 recommended movies:");
    for (title, rank) in recommendations {
        println!("{} (PageRank Score: {:.4})", title, rank);
    }
}