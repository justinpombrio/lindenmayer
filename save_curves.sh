run() {
    cargo run -- -s 2048 "$@"
}

mkdir images -p
run moore               11 -t 0     -c 3                   -a 0.25   -o images/moore.png
run zorder              11 -t 0     -c 4                   -a 0      -o images/z-order.png
run dragon              22 -t 0     -c cet-l10 --bg 333    -a 0.25   -o images/dragon.png
run gosper               8 -t 0     -c h       --bg eee    -a 0      -o images/gosper.png --fg 080808
run sierpinski-triangle 13 -t 0     -c cet-l17 --bg 000a00 -a 0.25   -o images/sierpinski-triangle.png
run sierpinski-curve    11 -t 0     -c h                   -a 0.125  -o images/sierpinski-curve.png
run koch                 6 -t 0.75  -c b       --bg ddd    -a 0.25   -o images/koch.png     --fill abc
run triangle            11 -t 0     -c 6       --bg fff    -a 0.25   -o images/triangle.png
run s-curve              5 -t 0.275 -c ry      --bg 181812 -a 0.25   -o images/s-curve.png  --style curvy
run wunderlich-3         3 -t 0.65  -c m       --bg 181818 -a 0.25   -o images/wunderlich-3.png
run arioni               5 -t 0.7   -c o6      --bg 222    -a -0.078 -o images/arioni.png
run steemann             3 -t 0.45  -c cet-l08 --bg 111    -a 0.25   -o images/steemann.png --style curvy
run fivefold            10 -t 0     -c h       --bg fff    -a 0.4167 -o images/fivefold.png -s 1024
