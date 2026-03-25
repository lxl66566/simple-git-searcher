use ignore::WalkBuilder;
use memchr::{memchr, memmem};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

// 使用 thiserror 定义自定义错误
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory traversal error: {0}")]
    Ignore(#[from] ignore::Error),
}

// 存储匹配结果的结构体
#[derive(Debug, Clone)]
pub struct Match {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub pattern: String,
    pub line_content: String,
}

/// 在指定的 Git 仓库中全量搜索多个字符串
pub fn search_in_repo<P: AsRef<Path>>(
    repo_path: P,
    patterns: &[String],
    ignore: &[PathBuf],
) -> Result<Vec<Match>, SearchError> {
    let repo_path = repo_path.as_ref();
    let ignore = ignore.iter().map(|p| p.as_path()).collect::<Vec<_>>();

    // 1. 使用 ignore 收集所有需要搜索的文件路径
    // WalkBuilder 默认会遵守 .gitignore 并跳过隐藏文件/目录（如 .git）
    let mut files = Vec::new();
    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .follow_links(false)
        .build();

    for result in walker {
        let entry = result?;
        // 只保留文件，过滤掉目录
        if entry.file_type().is_some_and(|ft| ft.is_file()) && !ignore.contains(&entry.path()) {
            files.push(entry.into_path());
        }
    }

    let finders: Vec<memmem::Finder<'_>> = patterns
        .iter()
        .map(|p| memmem::Finder::new(p.as_bytes()))
        .collect();

    // 2. 使用 rayon 并行处理所有文件
    let matches: Vec<Match> = files
        .into_par_iter()
        .filter_map(|path| {
            // 读取整个文件到内存中
            let buffer = std::fs::read(&path).ok()?;

            // 3. 避开非 Text 文件：如果文件中包含 \0 (Null byte)，则认为是二进制文件并跳过
            if memchr(b'\0', &buffer).is_some() {
                return None;
            }

            let mut file_matches = Vec::new();

            // 按行分割文件内容 (\n)
            for (line_idx, line) in buffer.split(|&b| b == b'\n').enumerate() {
                let line_num = line_idx + 1;

                // 遍历所有需要搜索的模式
                for (i, finder) in finders.iter().enumerate() {
                    if finder.find(line).is_some() {
                        // 找到匹配项，将字节转换为字符串（处理非标准 UTF-8 字符）
                        let line_content = String::from_utf8_lossy(line).into_owned();
                        file_matches.push(Match {
                            file_path: path.clone(),
                            line_number: line_num,
                            pattern: patterns[i].clone(),
                            line_content,
                        });
                    }
                }
            }

            if file_matches.is_empty() {
                None
            } else {
                Some(file_matches)
            }
        })
        .flatten() // 将每个文件产生的 Vec<Match> 展平为一个全局的 Vec<Match>
        .collect();

    Ok(matches)
}
