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

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module
    use std::collections::HashMap;

    
    #[test]
    fn test_pagerank() {
        // Test with a small dense graph
        let mut graph = HashMap::new();
        graph.insert(
            "A".to_string(),
            vec![
                ("B".to_string(), 1.0),
                ("C".to_string(), 1.0),
                ("D".to_string(), 1.0),
            ],
        );
        graph.insert(
            "B".to_string(),
            vec![
                ("A".to_string(), 1.0),
                ("C".to_string(), 1.0),
                ("D".to_string(), 1.0),
            ],
        );
        graph.insert(
            "C".to_string(),
            vec![
                ("A".to_string(), 1.0),
                ("B".to_string(), 1.0),
                ("D".to_string(), 1.0),
            ],
        );
        graph.insert(
            "D".to_string(),
            vec![
                ("A".to_string(), 1.0),
                ("B".to_string(), 1.0),
                ("C".to_string(), 1.0),
            ],
        );

        let ranks = compute_pagerank(&graph, 0.85, 50);

        // All nodes should have equal rank due to symmetry
        let rank_a = ranks.get("A").unwrap();
        let rank_b = ranks.get("B").unwrap();
        let rank_c = ranks.get("C").unwrap();
        let rank_d = ranks.get("D").unwrap();

        assert!((rank_a - rank_b).abs() < 1e-6);
        assert!((rank_b - rank_c).abs() < 1e-6);
        assert!((rank_c - rank_d).abs() < 1e-6);
    }
    #[test]
    fn test_calculate_similarity() {
        // Define two movies with different levels of similarity
        let movie1 = Movie {
            title: "Movie A".to_string(),
            movie_type: "Documentary".to_string(),
            director: "Director 1".to_string(),
            country: "USA".to_string(),
            release_year: 2020,
            rating: "PG-13".to_string(),
            listed_in: "Education, Science".to_string(),
        };

        let movie2 = Movie {
            title: "Movie B".to_string(),
            movie_type: "Documentary".to_string(),
            director: "Director 2".to_string(),
            country: "USA, Canada".to_string(),
            release_year: 2021,
            rating: "PG-13".to_string(),
            listed_in: "Education, History".to_string(),
        };

        // Calculate similarity
        let similarity = calculate_similarity(&movie1, &movie2);

        // Assert that similarity is between 0 and 1
        assert!(similarity >= 0.0 && similarity <= 1.0);

        // Assert that there is some similarity (not zero) due to shared attributes
        assert!(similarity > 0.5); // Adjust based on expected similarity
    }

    #[test]
    fn test_build_graph() {
        // Define a list of movies
        let movies = vec![
            Movie {
                title: "Movie A".to_string(),
                movie_type: "Documentary".to_string(),
                director: "Director 1".to_string(),
                country: "USA".to_string(),
                release_year: 2020,
                rating: "PG-13".to_string(),
                listed_in: "Education, Science".to_string(),
            },
            Movie {
                title: "Movie B".to_string(),
                movie_type: "Documentary".to_string(),
                director: "Director 2".to_string(),
                country: "USA".to_string(),
                release_year: 2021,
                rating: "PG-13".to_string(),
                listed_in: "Education, History".to_string(),
            },
            Movie {
                title: "Movie C".to_string(),
                movie_type: "Comedy".to_string(),
                director: "Director 3".to_string(),
                country: "Canada".to_string(),
                release_year: 2019,
                rating: "R".to_string(),
                listed_in: "Drama".to_string(),
            },
        ];

        // Build the graph
        let threshold = 0.5;
        let graph = build_graph(&movies, threshold);

        // Assert that the graph contains all movie titles as nodes
        assert!(graph.contains_key("Movie A"));
        assert!(graph.contains_key("Movie B"));
        assert!(graph.contains_key("Movie C"));

        // Assert that connections are above the threshold
        for (node, edges) in &graph {
            for (_, weight) in edges {
                assert!(*weight >= threshold, "Edge weight below threshold for node {}", node);
            }
        }

        // Check specific connections for expected structure
        let edges_a = graph.get("Movie A").unwrap();
        assert!(!edges_a.is_empty(), "Movie A should have edges");

        let edges_c = graph.get("Movie C").unwrap();
        assert!(edges_c.is_empty(), "Movie C should have no edges (dissimilar to others)");
    }
}