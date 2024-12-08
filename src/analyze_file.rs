use std::collections::HashSet;

pub fn analyze_type(type1: &str, type2: &str) -> f32 {
    if type1.is_empty() || type2.is_empty() {
        0.0
    } else if type1 == type2 {
        1.0
    } else {
        0.0
    }
}

pub fn analyze_director(director1: &str, director2: &str) -> f32 {
    if director1.is_empty() || director2.is_empty() {
        0.0
    } else if director1 == director2 {
        1.0
    } else {
        0.0
    }
}

pub fn analyze_country(country1: &str, country2: &str) -> f32 {
    if country1.is_empty() || country2.is_empty() {
        0.0
    } else if country1 == country2 {
        1.0
    } else {
        let countries1: HashSet<&str> = country1.split(',').map(|c| c.trim()).collect();
        let countries2: HashSet<&str> = country2.split(',').map(|c| c.trim()).collect();

        let intersection: usize = countries1.intersection(&countries2).count();

        if intersection > 0 {
            0.5 // Partial match
        } else {
            0.0
        }
    }
}

pub fn analyze_release_year(year1: u32, year2: u32) -> f32 {
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

pub fn analyze_rating(rating1: &str, rating2: &str) -> f32 {
    if rating1.is_empty() || rating2.is_empty() {
        0.0
    } else if rating1 == rating2 {
        1.0
    } else {
        0.0
    }
}

pub fn analyze_listed_in(genres1: &str, genres2: &str) -> f32 {
    if genres1.is_empty() || genres2.is_empty() {
        0.0
    } else {
        let genres1: HashSet<&str> = genres1.split(',').map(|g| g.trim()).collect();
        let genres2: HashSet<&str> = genres2.split(',').map(|g| g.trim()).collect();

        let intersection: usize = genres1.intersection(&genres2).count();
        let union: usize = genres1.union(&genres2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}
