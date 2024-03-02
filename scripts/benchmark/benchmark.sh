#!/usr/bin/env bash

current_time=$(date "+%Y%m%d-%H%M%S")
script_dir=$(dirname "$0")
report="$script_dir/out/report-$current_time.md"
lines_report="$script_dir/out/lines.md"
words_report="$script_dir/out/words.md"
warmup=15
runs=100

cargo build --release

# expects 2 arg:
# [1] => export file (markdown)
# [2] => mode -w | -l | --words | --lines
# construct args list based on data folder to avoid redundancy
bench(){
    export_md=$1
    mode=$2

    args=("--export-markdown" "$export_md" "--warmup" "$warmup" "--runs" "$runs")
    for data_fn in $script_dir/data/*.txt
    do
        data_fn_relative=$(realpath $data_fn --relative-to="$script_dir")
        cmd_path=$(realpath "./target/release/wc")
        cmd="$cmd_path $data_fn $mode"
        args+=("-n" "wc $data_fn_relative $mode" "$cmd")
    done

    hyperfine "${args[@]}"

    echo "## \`wc $mode\` benchmark" >> $report
    cat $export_md >> $report
    rm $export_md
}


echo "# Benchmark report" > $report

# line mode
bench $lines_report --lines

# word mode
bench $words_report --words
