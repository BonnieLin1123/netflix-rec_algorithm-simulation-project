mod read_file;
mod analyze_file;
mod build_graph;

use read_file::Movie;
use build_graph::{build_graph, calculate_similarity};

fn main() {
    let file_path = "netflix_titles 2.csv"; 
    let movies = Movie::read_and_clean(file_path);

    if movies.is_empty() {
        println!("No valid movies were loaded from the file.");
        return;
    }

    // Hardcoded movie title
    let input_title = "Dick Johnson Is Dead";

    // Find the selected movie
    if let Some(selected_movie) = movies.iter().find(|m| m.title == input_title) {
        println!("Comparing '{}' to all other movies...\n", selected_movie.title);

        // Calculate similarity with all other movies
        let mut similarities = Vec::new();
        for movie in &movies {
            if movie.title != selected_movie.title {
                let similarity = calculate_similarity(selected_movie, movie);
                similarities.push((movie.title.clone(), similarity));
            }
        }

        // Sort by similarity scores in descending order
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Print the top 5 most similar movies
        println!("Top 5 similar movies:");
        for (title, score) in similarities.iter().take(5) {
            println!("{} (Similarity Score: {:.4})", title, score);
        }
    } else {
        println!("Movie '{}' not found in the dataset.", input_title);
    }
}
