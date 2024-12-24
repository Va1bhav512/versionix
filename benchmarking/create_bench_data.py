import os
import random
import sys
from pathlib import Path

def create_dir(path, partitions):
    curr = path 
    i = 0
    for dir_size in partitions:
        dir_path = os.path.join(curr, f"dir{i}")
        os.mkdir(dir_path)
        
        curr = create_txt(dir_path, dir_size)
        i += 1

def create_txt(path, size):
    partitions = []
    
    if size <= 1:
        partitions.append(size)
    else:
        num_partition = random.randint(3, 10)  
        while size > 0 and num_partition > 1:
            max_partition_size = max(1, size // 2)
            partition_size = random.randint(1, max_partition_size)
            partitions.append(partition_size)
            size -= partition_size
            num_partition -= 1

        partitions.append(size)

    random.shuffle(partitions)

    total_size = sum(partitions)
    if total_size != sum(partitions):
        print(f"Error: partition sizes do not sum to {sum(partitions)}")
        sys.exit()

    for j, partition_size in enumerate(partitions):
        filepath = os.path.join(path, f"test{j}.txt") 
        with open(filepath, "w") as f:
            f.write("a" * partition_size)
    
    return path  

n, b = input("Enter the size (xx bytes/ xx KB/ xx MB/ xx GB): ").strip().split()
if (b == "bytes"):
    n = int(n)
elif (b.upper() == "KB"):
    n = int(n) * 1024
elif (b.upper() == "MB"):
    n = int(n) * 1024 * 1024
elif (b.upper() == "GB"):
    n = int(n) * 1024 * 1024 * 1024
else:
    print("Enter in proper format")
    sys.exit()

print("n", n)
path = Path(input("Enter path: "))
num_partition = random.randint(10, 20)  
partitions = []

if n <= 1:
    partitions.append(n)
else:
    for i in range(num_partition - 1):
        max_partition_size = max(1, n // 2)  
        partition_size = random.randint(1, max_partition_size)
        partitions.append(partition_size)
        n -= partition_size

    if n > 0:
        partitions.append(n)
    else:
        partitions.append(1)

random.shuffle(partitions)

create_dir(path, partitions)
