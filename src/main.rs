use simple_git_searcher::search_in_repo;
use std::path::PathBuf;
use std::process;

use palc::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    repo: Option<PathBuf>,
    patterns: Vec<String>,
    #[arg(short, long)]
    ignore: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    if cli.patterns.is_empty() {
        eprintln!("No patterns specified.");
        process::exit(1);
    }

    // 调用 lib 中的搜索逻辑
    match search_in_repo(
        cli.repo.unwrap_or(PathBuf::from(".")),
        &cli.patterns,
        &cli.ignore,
    ) {
        Ok(matches) => {
            if matches.is_empty() {
                // 没搜索到任何内容，返回非 0 error code
                eprintln!("No matches found.");
                process::exit(1);
            } else {
                // 打印搜索结果
                for m in matches {
                    println!(
                        "{}:{}: [{}] {}",
                        m.file_path.display(),
                        m.line_number,
                        m.pattern,
                        m.line_content.trim_end() // 去除行尾可能存在的 \r
                    );
                }
                // 搜索成功且有结果，返回 0
                process::exit(0);
            }
        }
        Err(e) => {
            eprintln!("Error searching repository: {}", e);
            process::exit(2);
        }
    }
}
