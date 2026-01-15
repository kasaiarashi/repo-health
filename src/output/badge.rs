pub fn generate_badge_url(score: f64) -> String {
    let (label, color) = match score {
        s if s >= 90.0 => ("excellent", "brightgreen"),
        s if s >= 80.0 => ("good", "green"),
        s if s >= 70.0 => ("fair", "yellow"),
        s if s >= 60.0 => ("needs--improvement", "orange"),
        _ => ("poor", "red"),
    };

    format!(
        "https://img.shields.io/badge/repo--health-{}-{}?style=flat-square&logo=github",
        label,
        color
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_url_generation() {
        let url = generate_badge_url(95.0);
        assert!(url.contains("excellent"));
        assert!(url.contains("brightgreen"));

        let url = generate_badge_url(85.0);
        assert!(url.contains("good"));
        assert!(url.contains("green"));

        let url = generate_badge_url(50.0);
        assert!(url.contains("poor"));
        assert!(url.contains("red"));
    }
}
