## Benchmarking

Here you can use pre-written Python scripts to automate benchmarking process against Git

### Requirements

1. Python 3
2. Git installed and set on path

### Process

1. Create testing folder named `bench_data` in this folder.
```bash
mkdir bench_data
```
2. Inside the bench_data directory create 3 folders for small, medium and large datasets.
```bash
cd bench_data
mkdir small_dataset
mkdir medium_dataset
mkdir large_dataset
cd ../
```
3. Create testing data of suitable size. Follow the instructions after running the script.
```bash
python create_bench_data.py
```
4. Build the release version.
```bash
cargo build --release
```
5. Run the benchmarking script.
```bash
python versionix_benchmarking.py
```

### Results

The results are saved in a CSV file named results.csv. Average results are in the order
`[compression_ratio,compression_ratio_git,difference_compression_size,difference_compression_time,difference_recover_time,difference_new_commit_time]`

benchmarking.csv contains following results of each directory
`[Original_Size,Compressed_Size,Time_Taken_To_Compress,Time_Taken_To_Recover_Entirely,Time_Taken_To_Make_A_New_Commit,Compressed_Size_By_Git,Time_Taken_To_Compress_By_Git,Time_Taken_To_Recover_Entirely_By_Git,Time_Taken_To_Make_A_New_Commit_By_Git]`

> Before running the benchmarking script again please delete .git folder from each of the 3 datasets!

(If you can edit the script to reliably delete it everytime on different operating systems please contribute to it!)
