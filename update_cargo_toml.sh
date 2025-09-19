#!/bin/bash

# 改进的 update_cargo_toml.sh - 支持区域标记防重复
# 使用 START 和 END 标记来管理自动生成的 [[bin]] 配置

SCRIPT_DIR="$(dirname "$0")"
CARGO_TOML="$SCRIPT_DIR/Cargo.toml"
TEMP_FILE="$SCRIPT_DIR/Cargo.toml.tmp"

# 标记常量
START_MARKER="# AUTO-GENERATED EXERCISE BINARIES START - DO NOT EDIT MANUALLY"
END_MARKER="# AUTO-GENERATED EXERCISE BINARIES END"

echo "🔍 检查 Cargo.toml 中的自动生成区域..."

# 检查是否已存在标记
if grep -q "$START_MARKER" "$CARGO_TOML"; then
    echo "📍 发现现有的自动生成区域，将替换内容..."
    
    # 删除现有的自动生成区域
    awk "
    BEGIN { skip = 0 }
    /$START_MARKER/ { skip = 1; next }
    /$END_MARKER/ { skip = 0; next }
    !skip { print }
    " "$CARGO_TOML" > "$TEMP_FILE"
    
    # 将临时文件内容写回
    mv "$TEMP_FILE" "$CARGO_TOML"
    echo "🧹 已清理现有的自动生成区域"
else
    echo "🆕 未发现现有标记，将在文件末尾添加新区域"
fi

# 生成新的配置
echo "🔄 扫描 exercises 目录..."
NEW_CONFIGS=$("$SCRIPT_DIR/scan_exercises.sh" 2>/dev/null)

if [ -z "$NEW_CONFIGS" ]; then
    echo "⚠️ 未找到任何含 main 函数的练习文件"
    exit 1
fi

# 统计文件数量
FILE_COUNT=$(echo "$NEW_CONFIGS" | grep -c "^\[\[bin\]\]")
echo "📊 检测到 $FILE_COUNT 个练习文件"

# 在文件末尾添加新的标记区域
{
    echo "$START_MARKER"
    echo "# Generated on: $(date)"
    echo "# Files processed: $FILE_COUNT"
    echo "$NEW_CONFIGS"
    echo "$END_MARKER"
} >> "$CARGO_TOML"

echo "✅ 已成功更新 Cargo.toml"
echo "📁 处理了 $FILE_COUNT 个练习文件的 [[bin]] 配置"
echo ""
echo "🧪 测试运行示例："
echo "   cargo run --bin clippy_clippy1"
echo "   cargo run --bin variables_variables1"