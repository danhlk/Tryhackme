import exiftool
import json
import os
import sys

path_to_folder_contains_file = str(sys.argv[1])

files = os.listdir(path_to_folder_contains_file)
metadata = list()

with exiftool.ExifTool() as et:
    for file in files:
        metadata.append(et.get_metadata_batch([os.path.join(path_to_folder_contains_file, file)]))

print ('File(s) containes string Version: 1.1:')
c = 0
for d in metadata:
    if 'Version\': 1.1' in str(d):
        print (d[0]['File:FileName'])
        c += 1
print ('Total:', c)