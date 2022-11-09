incr=2
for start in $(seq 0 $incr 99); do
    end=$((start + $incr))
    cargo run $start $end
    sleep 5
done
