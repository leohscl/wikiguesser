incr=2
num_article=100
# num_article=10
for start in $(seq 0 $incr $num_article); do
    end=$((start + $incr))
    cargo run $start $end
    sleep 1
done
