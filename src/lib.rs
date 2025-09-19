// src/lib.rs
// 使用构建脚本自动生成的 exercises 模块声明

// 包含构建时生成的模块声明
include!(concat!(env!("OUT_DIR"), "/exercises_modules.rs"));
