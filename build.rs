// build.rs - 简化版本
// 自动生成 exercises 模块声明的构建脚本

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exercises_modules.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    // 告诉 Cargo 当 exercises 目录发生变化时重新运行构建脚本
    println!("cargo:rerun-if-changed=exercises");

    let exercises_dir = Path::new("exercises");
    if !exercises_dir.exists() {
        panic!("exercises directory not found!");
    }

    // 写入文件头
    writeln!(f, "// 自动生成的 exercises 模块声明").unwrap();
    writeln!(f, "// 此文件由 build.rs 生成，请勿手动编辑").unwrap();
    writeln!(f).unwrap();

    let mut modules = Vec::new();

    // 扫描 exercises 目录
    collect_exercise_files(exercises_dir, &mut modules, "");

    // 生成模块声明
    for (module_path, file_path) in &modules {
        let current_dir = env::current_dir().unwrap();
        let absolute_path = current_dir.join(file_path);
        writeln!(f, "#[path = \"{}\"]", absolute_path.display()).unwrap();
        writeln!(f, "pub mod {};", module_path.replace('/', "_")).unwrap();
    }

    writeln!(f).unwrap();
    writeln!(f, "// 将所有 exercises 组织到一个模块中").unwrap();
    writeln!(f, "pub mod exercises {{").unwrap();

    for (module_path, _) in &modules {
        let module_name = module_path.replace('/', "_");
        writeln!(f, "    pub use crate::{};", module_name).unwrap();
    }

    writeln!(f, "}}").unwrap();

    println!("Generated {} exercise modules", modules.len());
}

fn collect_exercise_files(
    dir: &Path,
    modules: &mut Vec<(String, std::path::PathBuf)>,
    prefix: &str,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() && !name.starts_with('.') && name != "target" {
                // 递归处理子目录
                let new_prefix = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{}/{}", prefix, name)
                };
                collect_exercise_files(&path, modules, &new_prefix);
            } else if path.is_file() && name.ends_with(".rs") && name != "mod.rs" {
                // 检查是否是完成的练习
                if is_exercise_completed(&path) {
                    let module_name = name.trim_end_matches(".rs");
                    let full_name = if prefix.is_empty() {
                        module_name.to_string()
                    } else {
                        format!("{}/{}", prefix, module_name)
                    };
                    modules.push((full_name, path));
                }
            }
        }
    }
}

fn is_exercise_completed(file_path: &Path) -> bool {
    // 如果读取文件失败，默认认为未完成
    if let Ok(content) = fs::read_to_string(file_path) {
        // 如果包含 "I AM NOT DONE"，说明练习未完成
        !content.contains("I AM NOT DONE")
    } else {
        false
    }
}
