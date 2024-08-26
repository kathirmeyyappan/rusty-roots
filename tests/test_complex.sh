#!/bin/bash

cleanup() {
    rm -rf $1
    rm -f rusty_roots_output.txt tree_output.txt
}

# vars
test_dir="test_tree"
test_output_dir="outs"

# compile
RUSTFLAGS="-Awarnings" cargo build --release

# create complex directory structure
mkdir -p $test_dir/subdir1/{subsubdir1,subsubdir2}
mkdir -p $test_dir/subdir2/{subsubdir3,subsubdir4/{subsubsubdir1,subsubsubdir2}}
mkdir -p $test_dir/subdir3/{subsubdir5,subsubdir6/{subsubsubdir3,subsubsubdir4}}
mkdir -p $test_dir/subdir4/subsubdir7

# create files
touch $test_dir/file1.txt
touch $test_dir/subdir1/file2.txt
touch $test_dir/subdir1/subsubdir1/file3.txt
touch $test_dir/subdir1/subsubdir2/file4.txt
touch $test_dir/subdir2/file5.txt
touch $test_dir/subdir2/subsubdir3/file6.txt
touch $test_dir/subdir2/subsubdir4/file7.txt
touch $test_dir/subdir2/subsubdir4/subsubsubdir1/file8.txt
touch $test_dir/subdir2/subsubdir4/subsubsubdir2/file9.txt
touch $test_dir/subdir3/file10.txt
touch $test_dir/subdir3/subsubdir5/file11.txt
touch $test_dir/subdir3/subsubdir6/file12.txt
touch $test_dir/subdir3/subsubdir6/subsubsubdir3/file13.txt
touch $test_dir/subdir3/subsubdir6/subsubsubdir4/file14.txt
touch $test_dir/subdir4/file15.txt
touch $test_dir/subdir4/subsubdir7/file16.txt

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
cleanup "$test_dir"
cleanup "$test_output_dir"
cargo clean