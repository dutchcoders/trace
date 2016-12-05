import json
from pprint import pprint

with open('TABELLA_64.json') as data_file:
    data = json.load(data_file)

for k, s in data.items():
    print("Syscall{{number: {0}, name: \"{1}\", file: \"{2}\", sysname: \"{3}\"}},".format(s[0], s[1], s[2], s[3]))
