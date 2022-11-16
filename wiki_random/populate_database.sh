incr=2
for start in $(seq 0 $incr 999); do
    end=$((start + $incr))
    cargo run $start $end
    sleep 1
done
