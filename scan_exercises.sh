#!/bin/bash

# 脚本：检测 exercises 目录中包含 main 函数的文件并生成 Cargo.toml 配置

set -e

EXERCISES_DIR="exercises"

# 查找包含 main 函数的文件
main_files=()

# 递归搜索所有 .rs 文件
while IFS= read -r -d '' file; do
    # 检查文件是否包含 main 函数
    if grep -q "fn main()" "$file"; then
        # 只输出到 stderr，不影响主输出
        echo "✅ 发现包含 main 函数的文件: $file" >&2
        main_files+=("$file")
    fi
done < <(find "$EXERCISES_DIR" -name "*.rs" -type f -print0)

echo "📊 总共找到 ${#main_files[@]} 个包含 main 函数的文件" >&2

if [ ${#main_files[@]} -eq 0 ]; then
    echo "❌ 没有找到包含 main 函数的文件" >&2
    exit 0
fi

# 生成二进制文件配置到标准输出
for file in "${main_files[@]}"; do
    # 提取文件名（去掉路径和扩展名）
    filename=$(basename "$file" .rs)
    
    # 提取目录结构，用于生成唯一的二进制名称
    relative_path=${file#$EXERCISES_DIR/}
    dir_path=$(dirname "$relative_path")
    
    # 生成二进制名称
    if [ "$dir_path" = "." ]; then
        bin_name="$filename"
    else
        # 将路径中的 / 替换为 _
        dir_name=$(echo "$dir_path" | tr '/' '_')
        bin_name="${dir_name}_${filename}"
    fi
    
    echo "[[bin]]"
    echo "name = \"$bin_name\""
    echo "path = \"$file\""
    echo ""
done