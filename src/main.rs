use clap::Parser;
use colored::Colorize;
use repo_health::{
    cli::{Cli, parse_repo_input},
    github::GitHubClient,
    analyzers::{
        Analyzer, DocumentationAnalyzer, TestsAnalyzer,
        CiCdAnalyzer, DependenciesAnalyzer, BusFactorAnalyzer,
    },
    scoring::ScoreCalculator,
    output::MarkdownGenerator,
};
use std::fs;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> repo_health::Result<()> {
    let cli = Cli::parse();

    // Parse repository input
    let (owner, repo) = parse_repo_input(&cli.repository)?;

    if !cli.quiet {
        println!("{}", "=".repeat(60).cyan());
        println!("{}", "Repository Health Analyzer".cyan().bold());
        println!("{}", "=".repeat(60).cyan());
        println!("\n{} {}/{}", "Analyzing:".bold(), owner.green(), repo.green());

        if cli.token.is_none() {
            println!("\n{} No GitHub token provided. Using unauthenticated access (lower rate limits).", "⚠".yellow());
            println!("{} Set GITHUB_TOKEN env var or use --token for higher rate limits.\n", "ℹ".cyan());
        }

        println!();
    }

    // Initialize GitHub client
    let client = GitHubClient::new(cli.token)?;

    // Fetch all data
    if !cli.quiet {
        println!("{}", "Fetching repository data...".yellow());
    }

    let repo_data = client.fetch_all_data(&owner, &repo).await?;

    if !cli.quiet {
        println!("{} Data fetched successfully", "✓".green());
        println!();
    }

    // Run analyzers
    let analyzers: Vec<(String, f64, Box<dyn Analyzer>)> = vec![
        ("Documentation".to_string(), 0.20, Box::new(DocumentationAnalyzer)),
        ("Tests".to_string(), 0.25, Box::new(TestsAnalyzer)),
        ("CI/CD".to_string(), 0.20, Box::new(CiCdAnalyzer)),
        ("Dependencies".to_string(), 0.20, Box::new(DependenciesAnalyzer)),
        ("Bus Factor".to_string(), 0.15, Box::new(BusFactorAnalyzer)),
    ];

    if !cli.quiet {
        println!("{}", "Running analyzers...".yellow());
        println!();
    }

    let mut results = Vec::new();

    for (name, weight, analyzer) in analyzers {
        if !cli.quiet {
            print!("  {} {}... ", "→".cyan(), name.bold());
        }

        let result = analyzer.analyze(&repo_data).await?;

        if !cli.quiet {
            let grade = ScoreCalculator::grade_short(result.score);
            let score_colored = if result.score >= 80.0 {
                format!("{:.1}/100 ({})", result.score, grade).green()
            } else if result.score >= 60.0 {
                format!("{:.1}/100 ({})", result.score, grade).yellow()
            } else {
                format!("{:.1}/100 ({})", result.score, grade).red()
            };
            println!("{}", score_colored);
        }

        results.push((name, weight, result));
    }

    // Calculate overall score
    let overall_score = ScoreCalculator::calculate_overall(&results);
    let grade = ScoreCalculator::grade(overall_score);

    if !cli.quiet {
        println!();
        println!("{}", "=".repeat(60).cyan());
        print!("{} ", "Overall Score:".bold());

        let score_colored = if overall_score >= 80.0 {
            format!("{:.1}/100 ({})", overall_score, grade).green().bold()
        } else if overall_score >= 60.0 {
            format!("{:.1}/100 ({})", overall_score, grade).yellow().bold()
        } else {
            format!("{:.1}/100 ({})", overall_score, grade).red().bold()
        };

        println!("{}", score_colored);
        println!("{}", "=".repeat(60).cyan());
        println!();
    }

    // Generate markdown report
    let markdown = MarkdownGenerator::generate(&owner, &repo, overall_score, &results);

    // Write to file
    fs::write(&cli.output, &markdown)?;

    if !cli.quiet {
        println!("{} Report saved to: {}", "✓".green(), cli.output.green().bold());
    }

    // Print to stdout if not quiet
    if !cli.quiet {
        println!();
        println!("{}", "=".repeat(60).cyan());
        println!("{}", "Report Preview:".cyan().bold());
        println!("{}", "=".repeat(60).cyan());
        println!();
        println!("{}", markdown);
    }

    Ok(())
}
