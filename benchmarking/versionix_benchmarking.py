import time
import subprocess
from pathlib import Path
import csv
import shutil

def get_directory_size(root_directory: Path)->int:
    """
    root_directory: Path
    Returns the size of all the files in the root directory in bytes
    """
    return sum(f.stat().st_size for f in root_directory.glob('**/*') if f.is_file())

def delete_except(target_directory: Path, key: str)->None:
    """
    target_directory: Path
    Deletes all folders and files in the target directory except key folder
    """
    for item in target_directory.iterdir():
        if item.name == key:
            continue
        if item.is_dir():
            shutil.rmtree(item)
        else:
            item.unlink()

def main()->None:
    versionix_exe = "..\\target\\release\\versionix"
    small_dataset = Path(".\\bench_data\\small_dataset").resolve()
    medium_dataset = Path(".\\bench_data\\medium_dataset").resolve()
    large_dataset = Path(".\\bench_data\\large_dataset").resolve()
    datasets = [small_dataset,medium_dataset,large_dataset]
    # datasets = [small_dataset]
    with open('.\\benchmarking.csv', 'w', newline = '') as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["Original_Size","Compressed_Size","Time_Taken_To_Compress","Time_Taken_To_Recover_Entirely","Time_Taken_To_Make_A_New_Commit",
                         "Compressed_Size_By_Git","Time_Taken_To_Compress_By_Git","Time_Taken_To_Recover_Entirely_By_Git","Time_Taken_To_Make_A_New_Commit_By_Git"]) # header
        for dataset in datasets:
            row = []
            row.append(get_directory_size(dataset))

            start_time = time.time()
            subprocess.run([versionix_exe] + ["init"], cwd=dataset)
            end_time = time.time()
            compression_time = end_time - start_time
            row.append(get_directory_size(Path.joinpath(dataset,".vx")))
            row.append(f"{compression_time:.4f}")

            delete_except(dataset,".vx")
            result = subprocess.run([versionix_exe] + ["log"], capture_output=True, cwd=dataset)
            stdout_str = result.stdout.decode('utf-8')
            initial_commit_hash = stdout_str[15:(stdout_str.find('\n',15))] #Skip the first 15 characters for "initial commit "
            start_time = time.time()
            subprocess.run([versionix_exe] + ["rc"] + [initial_commit_hash], cwd=dataset)
            end_time = time.time()
            recover_entirely_time = end_time - start_time
            row.append(f"{recover_entirely_time:.4f}")

            # Make a new file and commit it
            new_file = Path.joinpath(dataset, "new_test.txt")
            with open(new_file, 'w') as f:
                f.write('This is new content')

            start_time = time.time()
            subprocess.run([versionix_exe] + ["commit","test"], cwd=dataset)
            end_time = time.time()
            new_commit_time = end_time - start_time
            row.append(f"{new_commit_time:.4f}")
            shutil.rmtree(Path.joinpath(dataset,".vx"))
            new_file.unlink()

            start_time = time.time()
            subprocess.run(["git", "init"], cwd=dataset)
            subprocess.run(["git","add","."], cwd=dataset)
            subprocess.run(["git","commit","-m","initial commit"], cwd=dataset)
            end_time = time.time()
            compression_time = end_time - start_time
            row.append(get_directory_size(Path.joinpath(dataset,".git")))
            row.append(f"{compression_time:.4f}")


            delete_except(dataset, ".git")
            # subprocess.run(["git","restore","--source=HEAD", "--staged","."], cwd=dataset)
            start_time = time.time()
            subprocess.run(["git","restore","--source=HEAD","."], cwd=dataset)
            end_time = time.time()
            recover_entirely_time = end_time - start_time
            row.append(f"{recover_entirely_time:.4f}")

            # Make a new file and commit it
            new_file = Path.joinpath(dataset, "new_test.txt")
            with open(new_file, 'w') as f:
                f.write('This is new content')

            start_time = time.time()
            subprocess.run(["git","add","new_test.txt"], cwd=dataset)
            subprocess.run(["git","commit", "-m","test"], cwd=dataset)
            end_time = time.time()
            new_commit_time = end_time - start_time
            row.append(f"{new_commit_time:.4f}")
            new_file.unlink()
            writer.writerow(row)

    with open('.\\benchmarking.csv', 'r') as csvfile:
        reader = csv.reader(csvfile)
        n = 0
        compression_ratio = 0
        compression_ratio_git = 0
        difference_compression_size = 0
        difference_compression_time = 0
        difference_recover_time = 0
        difference_new_commit_time = 0
        next(reader,None)
        for line in reader:
            n += 1
            og_size = float(line[0])
            c_size_vx = float(line[1])
            t_compress_vx = float(line[2])
            t_recover_vx = float(line[3])
            t_new_vx = float(line[4])
            c_size_git = float(line[5])
            t_compress_git = float(line[6])
            t_recover_git = float(line[7])
            t_new_git = float(line[8])
            print("Compression ratio = ", ((og_size - c_size_vx)/og_size)*100)
            compression_ratio += ((og_size - c_size_vx)/og_size)*100
            print("Compression ratio git = ", ((og_size - c_size_git)/og_size)*100)
            compression_ratio_git += ((og_size - c_size_git)/og_size)*100
            print("Difference in compressed size = ", c_size_vx - c_size_git)
            difference_compression_size += c_size_vx - c_size_git
            print("Difference in compression time = ", t_compress_vx - t_compress_git)
            difference_compression_time += t_compress_vx - t_compress_git
            print("Difference in recover time = ", t_recover_vx - t_recover_git)
            difference_recover_time += t_recover_vx - t_recover_git
            print("Difference in new commit time = ", t_new_vx - t_new_git)
            difference_new_commit_time += t_new_vx - t_new_git

        compression_ratio /= n
        compression_ratio_git /= n
        difference_compression_size /= n
        difference_compression_time /= n
        difference_recover_time /= n
        difference_new_commit_time /= n
            
        with open('.\\results.csv', 'a', newline = '') as file:
            row = [compression_ratio,compression_ratio_git,difference_compression_size,
                   difference_compression_time,difference_recover_time,difference_new_commit_time]
            writer = csv.writer(file)
            writer.writerow(row)

if __name__ == "__main__":
    main()
