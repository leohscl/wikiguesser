incr=5
num_article=500
cargo build --release
for start in $(seq 0 $incr $num_article); do
    end=$((start + $incr))
    ./target/release/wiki_random $start $end
    sleep 0.1
done
