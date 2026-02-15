run() {
    cargo run -- -s 2048 "$@"
}

mkdir turtles -p
run turtle-L   0       -c cet-l10 --bg 333 -o turtles/turtle-L.png
run turtle-F   0       -c cet-l10 --bg 333 -o turtles/turtle-F.png
run turtle-hex 0       -c cet-l10 --bg 333 -o turtles/turtle-hex.png
run dragon     9       -c cet-l10 --bg 333 -o turtles/drake.png
run dragon     22 -t 0 -c cet-l10 --bg 333 -o turtles/dragon.png
run dragon     22 -t 0 -c h       --bg 333 -o turtles/hilbert-dragon.png
run hilbert    5       -c cet-l10 --bg 333 -o turtles/hilbert.png   
run koch       3       -c cet-l10 --bg 333 -o turtles/koch.png      
run gosper     8 -t 0  -c h       --bg eee -o turtles/splash.png --fg 080808 -a 0.25
