import zipfile
import os
import sys

path_to_folder_contain_file = str(sys.argv[1])
path_to_folder_extract = str(sys.argv[2])

files = [file for file in os.listdir(path_to_folder_contain_file) if file.endswith('.zip')]
for file in files:
    with zipfile.ZipFile(os.path.join(path_to_folder_contain_file, file), 'r') as zip_ref:
        zip_ref.extractall(path_to_folder_extract)

print ("Extract done!")
