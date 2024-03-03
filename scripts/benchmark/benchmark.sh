#!/usr/bin/env bash

current_time=$(date "+%Y%m%d-%H%M%S")
script_dir=$(dirname "$0")
report="$script_dir/out/report-$current_time.md"
warmup=100
runs=400

cargo build --release

# Function to get the path to the local wc command
local_wc() {
    realpath "./target/release/wc"
}

# Function to run hyperfine benchmarks
hf() {
    hyperfine --warmup "$warmup" --runs "$runs" "${@}"
}

append_to_report() {
    echo $1 >>"$report"
}

# Function to append results to the report and clean up
append_results_to_report_and_cleanup() {
    append_to_report "## \`wc$2\` benchmark"
    cat "$1" >>"$report"
    rm "$1"
}

# Function to get data files
get_data_files() {
    echo "$script_dir"/data/*.txt
}

# Benchmarks for lines, words, bytes, and all
bench_modes() {
    bench_modes=("lines" "words" "bytes" "all")
    for mode in "${bench_modes[@]}"; do
        mode_report="$script_dir/out/${mode}.md"
        export_md="--export-markdown $mode_report"

        # Check if the mode is "all"
        if [ "$mode" == "all" ]; then
            mode_arg=""
        else
            mode_arg=" --${mode}"
        fi

        args=($export_md)
        for data_fn in $(get_data_files); do
            data_fn_relative=$(realpath "$data_fn" --relative-to="$script_dir")
            cmd="$(local_wc) $data_fn$mode_arg"
            args+=("-n" "wc $data_fn_relative$mode_arg" "$cmd")
        done

        hf "${args[@]}"
        append_results_to_report_and_cleanup "$mode_report" "$mode_arg"
    done
}

# Benchmark for wildcard
bench_wildcard() {
    wildcard_report="$script_dir/out/wildcard.md"
    wc_path=$(local_wc)
    data_files=$(get_data_files)

    local_cmd="$wc_path $data_files"
    os_cmd="wc $data_files"

    hf --export-markdown "$wildcard_report" \
        -n "wc(local) data/*.txt" "$local_cmd" \
        -n "wc(GNU) data/*.txt" "$os_cmd"

    append_results_to_report_and_cleanup "$wildcard_report" " data/*.txt"
}

touch "$report"

append_to_report "# Benchmark report"
append_to_report "## Settings"
append_to_report "| name | value |"
append_to_report "|------|-------|"
append_to_report "| **Number of runs** | $runs |"
append_to_report "| **Number of warumps**| $warmup |"

bench_wildcard
bench_modes
