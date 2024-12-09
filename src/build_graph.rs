use std::collections::HashMap;
use crate::read_file::Movie;
use crate::analyze_file::{analyze_type, analyze_director, analyze_country, analyze_release_year, analyze_rating, analyze_listed_in};

pub fn calculate_similarity(movie1: &Movie, movie2: &Movie) -> f32 {
    let mut score = 0.0;
    let max_possible_score = 6.0;
    score += analyze_type(&movie1.movie_type, &movie2.movie_type);
    score += analyze_director(&movie1.director, &movie2.director);
    score += analyze_country(&movie1.country, &movie2.country);
    score += analyze_release_year(movie1.release_year, movie2.release_year);
    score += analyze_rating(&movie1.rating, &movie2.rating);
    score += analyze_listed_in(&movie1.listed_in, &movie2.listed_in);
    score/max_possible_score
}

pub fn build_graph(movies: &Vec<Movie>, threshold: f32) -> HashMap<String, Vec<(String, f32)>> {
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
        if total_weight > 0.0 {
            let normalized_edges: Vec<_> = edges
            .into_iter()
            .map(|(title, weight)| (title, weight / total_weight))
            .collect();

        graph.insert(movies[i].title.clone(), normalized_edges);
        } else {
            graph.insert(movies[i].title.clone(), vec![]);
        }
        
    }

    graph
}

pub fn compute_pagerank(graph: &HashMap<String, Vec<(String, f32)>>, damping: f32, iterations: usize) -> HashMap<String, f32> {
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
