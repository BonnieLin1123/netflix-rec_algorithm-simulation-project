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

pub fn analyze_genres(genres1: &str, genres2: &str) -> f32 {
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

pub fn analyze_cast(cast1: &str, cast2: &str) -> f32 {
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

pub fn analyze_duration(duration1: &str, duration2: &str) -> f32 {
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