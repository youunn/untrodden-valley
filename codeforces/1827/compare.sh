cat input.txt | ./sample > o1.txt
cat input.txt | cargo run --bin "contest${PWD##*/}-d" 1> o2.txt
diff o1.txt o2.txt
