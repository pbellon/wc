data=("200" "500")
warmup=10
runs=1000

for nb_paragraphs in "${data[@]}"
do
    # line mode
    hyperfine --warmup "$warmup" --runs "$runs" \
        "target\release\wc.exe scripts\benchmark\data\\$nb_paragraphs.txt -l" \
        "wc.exe -l scripts\benchmark\data\\$nb_paragraphs.txt"

    # word mode
    # line mode
    hyperfine --warmup "$warmup" --runs "$runs" \
        "target\release\wc.exe scripts\benchmark\data\\$nb_paragraphs.txt -w" \
        "wc.exe -w scripts\benchmark\data\\$nb_paragraphs.txt"
done
