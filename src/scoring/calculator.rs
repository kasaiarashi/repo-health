use crate::analyzers::AnalysisResult;

pub struct ScoreCalculator;

impl ScoreCalculator {
    pub fn calculate_overall(results: &[(String, f64, AnalysisResult)]) -> f64 {
        let weighted_sum: f64 = results.iter()
            .map(|(_, weight, result)| result.score * weight)
            .sum();

        weighted_sum.min(100.0).max(0.0)
    }

    pub fn grade(score: f64) -> &'static str {
        match score {
            s if s >= 90.0 => "A+ Excellent",
            s if s >= 80.0 => "A Good",
            s if s >= 70.0 => "B Fair",
            s if s >= 60.0 => "C Needs Improvement",
            _ => "D Poor"
        }
    }

    pub fn grade_short(score: f64) -> &'static str {
        match score {
            s if s >= 90.0 => "A+",
            s if s >= 80.0 => "A",
            s if s >= 70.0 => "B",
            s if s >= 60.0 => "C",
            _ => "D"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::{AnalysisResult, Finding};

    #[test]
    fn test_calculate_overall() {
        let results = vec![
            ("Doc".to_string(), 0.20, AnalysisResult {
                score: 80.0,
                details: "test".to_string(),
                findings: vec![],
            }),
            ("Tests".to_string(), 0.25, AnalysisResult {
                score: 90.0,
                details: "test".to_string(),
                findings: vec![],
            }),
            ("CI".to_string(), 0.20, AnalysisResult {
                score: 100.0,
                details: "test".to_string(),
                findings: vec![],
            }),
            ("Deps".to_string(), 0.20, AnalysisResult {
                score: 70.0,
                details: "test".to_string(),
                findings: vec![],
            }),
            ("Bus".to_string(), 0.15, AnalysisResult {
                score: 100.0,
                details: "test".to_string(),
                findings: vec![],
            }),
        ];

        let overall = ScoreCalculator::calculate_overall(&results);
        assert!(overall >= 85.0 && overall <= 90.0);
    }

    #[test]
    fn test_grading() {
        assert_eq!(ScoreCalculator::grade(95.0), "A+ Excellent");
        assert_eq!(ScoreCalculator::grade(85.0), "A Good");
        assert_eq!(ScoreCalculator::grade(75.0), "B Fair");
        assert_eq!(ScoreCalculator::grade(65.0), "C Needs Improvement");
        assert_eq!(ScoreCalculator::grade(50.0), "D Poor");
    }
}
