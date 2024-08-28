#!/bin/bash

# vars
test_dir="test_tree"
test_output_dir="outs"

# compile
RUSTFLAGS="-Awarnings" cargo build --release

# create 
mkdir -p $test_dir/subdir1 $test_dir/subdir2
touch $test_dir/file1.txt $test_dir/subdir1/file2.txt $test_dir/subdir2/file3.txt

# run expected and observed
mkdir $test_output_dir
tree $test_dir > $test_output_dir/tree_output.txt
../target/release/rusty-roots -p $test_dir > $test_output_dir/rusty_roots_output.txt

# adjust files for stylistic differences
sed '1d; s/^.\{4\}//' $test_output_dir/rusty_roots_output.txt > $test_output_dir/adjusted_rusty_roots_output.txt
head -n -2 $test_output_dir/tree_output.txt > $test_output_dir/adjusted_tree_output.txt

# compare
diff $test_output_dir/adjusted_tree_output.txt $test_output_dir/adjusted_rusty_roots_output.txt

# clean up
cleanup() {
    rm -rf $1
}

cleanup "$test_dir"
cleanup "$test_output_dir"
cargo clean