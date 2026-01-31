#!/bin/bash
# Safety metrics report for quickjs-regex-rs
# Run from project root: bash scripts/safety_report.sh

cd "$(dirname "$0")/.."

echo "# Safety Metrics Report"
echo "Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo ""

# Files to analyze
FILES="src/regex/mod.rs src/regex/util.rs src/regex/engine.rs src/regex/unicode.rs"

# Use awk to analyze functions and their bodies
analyze_file() {
    local file=$1
    awk '
    BEGIN {
        pure_safe = 0
        wrapper = 0
        unsafe_fn = 0
        extern_c = 0
        in_fn = 0
        brace_depth = 0
        fn_type = ""
        has_unsafe_block = 0
    }

    # Detect function start (handles both top-level and impl block functions)
    /^[[:space:]]*(pub )?unsafe extern "C" fn / {
        if (in_fn == 0) {
            fn_type = "extern_c"
            in_fn = 1
            brace_depth = 0
            has_unsafe_block = 0
        }
    }

    /^[[:space:]]*(pub )?unsafe fn / && !/extern "C"/ {
        if (in_fn == 0) {
            fn_type = "unsafe"
            in_fn = 1
            brace_depth = 0
            has_unsafe_block = 0
        }
    }

    /^[[:space:]]*(pub )?fn / && !/unsafe/ {
        if (in_fn == 0) {
            fn_type = "safe"
            in_fn = 1
            brace_depth = 0
            has_unsafe_block = 0
        }
    }

    # Track braces when in function
    in_fn == 1 {
        # Check for unsafe blocks (unsafe { or unsafe{)
        if (/unsafe[[:space:]]*\{/) {
            has_unsafe_block = 1
        }

        # Count braces
        n = gsub(/{/, "{")
        brace_depth += n
        n = gsub(/}/, "}")
        brace_depth -= n

        # Function ended
        if (brace_depth <= 0 && index($0, "{") > 0 || (brace_depth == 0 && index($0, "}") > 0)) {
            if (fn_type == "extern_c") {
                extern_c++
            } else if (fn_type == "unsafe") {
                unsafe_fn++
            } else if (fn_type == "safe") {
                if (has_unsafe_block) {
                    wrapper++
                } else {
                    pure_safe++
                }
            }
            in_fn = 0
            fn_type = ""
        }
    }

    END {
        printf "%d %d %d %d\n", pure_safe, wrapper, unsafe_fn, extern_c
    }
    ' "$file"
}

echo "## Summary Table"
echo '```'
printf "| %-12s | %6s | %8s | %6s | %8s | %6s |\n" "File" "Pure" "Wrappers" "Unsafe" "extern C" "Lines"
printf "|--------------|--------|----------|--------|----------|--------|\n"

total_pure=0
total_wrapper=0
total_unsafe=0
total_extern=0
total_lines=0

for file in $FILES; do
    if [ -f "$file" ]; then
        bname=$(basename "$file")

        # Get counts from awk
        read pure wrapper unsafe_fn extern_c <<< $(analyze_file "$file")

        # Line count
        lines=$(wc -l < "$file" | tr -d '[:space:]')

        printf "| %-12s | %6d | %8d | %6d | %8d | %6d |\n" "$bname" "$pure" "$wrapper" "$unsafe_fn" "$extern_c" "$lines"

        total_pure=$((total_pure + pure))
        total_wrapper=$((total_wrapper + wrapper))
        total_unsafe=$((total_unsafe + unsafe_fn))
        total_extern=$((total_extern + extern_c))
        total_lines=$((total_lines + lines))
    fi
done

printf "|--------------|--------|----------|--------|----------|--------|\n"
printf "| %-12s | %6d | %8d | %6d | %8d | %6d |\n" "TOTAL" "$total_pure" "$total_wrapper" "$total_unsafe" "$total_extern" "$total_lines"
echo '```'

echo ""
echo "## Legend"
echo "- **Pure**: Safe functions with no \`unsafe\` blocks inside"
echo "- **Wrappers**: Safe API wrapping unsafe internals (contain \`unsafe { }\`)"
echo "- **Unsafe**: Functions marked \`unsafe fn\`"
echo "- **extern C**: Functions with C ABI (used as callbacks)"

echo ""
echo "## extern \"C\" functions (callbacks)"
echo ""
for file in $FILES; do
    if [ -f "$file" ]; then
        fns=$(grep -E '^[[:space:]]*(pub )?unsafe extern "C" fn ' "$file" 2>/dev/null | sed 's/(.*//;s/.*fn //' | tr '\n' ' ')
        if [ -n "$fns" ]; then
            echo "**$(basename "$file"):** $fns"
        fi
    fi
done

echo ""
echo "## Safe wrappers (safe API over unsafe internals)"
echo ""
for file in $FILES; do
    if [ -f "$file" ]; then
        # Find safe functions that contain unsafe blocks
        wrappers=$(awk '
        BEGIN { in_fn = 0; fn_name = ""; brace_depth = 0; has_unsafe = 0 }
        /^[[:space:]]*(pub )?fn [a-zA-Z_]/ && !/unsafe/ {
            if (in_fn == 0) {
                in_fn = 1
                brace_depth = 0
                has_unsafe = 0
                match($0, /fn ([a-zA-Z_][a-zA-Z0-9_]*)/, arr)
                fn_name = arr[1]
            }
        }
        in_fn == 1 {
            if (/unsafe[[:space:]]*\{/) has_unsafe = 1
            n = gsub(/{/, "{"); brace_depth += n
            n = gsub(/}/, "}"); brace_depth -= n
            if (brace_depth == 0 && fn_name != "") {
                if (has_unsafe) printf "%s ", fn_name
                in_fn = 0; fn_name = ""
            }
        }
        ' "$file")
        if [ -n "$wrappers" ]; then
            echo "**$(basename "$file"):** $wrappers"
        fi
    fi
done

echo ""
echo "## Metrics"
total_fns=$((total_pure + total_wrapper + total_unsafe + total_extern))
if [ $total_fns -gt 0 ]; then
    truly_safe_pct=$((total_pure * 100 / total_fns))
    safe_api_pct=$(((total_pure + total_wrapper) * 100 / total_fns))
    pure_rust_pct=$(((total_fns - total_extern) * 100 / total_fns))
    echo "- **Truly safe (no unsafe):** $total_pure / $total_fns ($truly_safe_pct%)"
    echo "- **Safe API (safe signature):** $((total_pure + total_wrapper)) / $total_fns ($safe_api_pct%)"
    echo "- **Pure Rust (no extern C):** $((total_fns - total_extern)) / $total_fns ($pure_rust_pct%)"
fi
echo "- **Total lines:** $total_lines"
