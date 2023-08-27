
# run: templates\convert.py

from re import sub

def convert(s: str) -> str:
    tmp = s
    
    if tmp[-1] == '\n':
        tmp = tmp[:-1]
    
    out = tmp \
        .replace("\\", r"\\") \
        .replace("    ", r"\t") \
        .replace("\n", r"\n") \
        .replace("$", r"\\$") \
        .replace('"', r'\"')
    
    out = sub("/\* \\\\\\\\\$(.*?) \*/", "$\\1", out)
    
    return out


from glob import glob
from os.path import basename

converted = []

for fullfn in glob("./*.rs"):
    fn = basename(fullfn)[:-3]
    with open(f"{fn}.rs") as f:
        converted.append((fn, convert(f.read())))

with open("converted.txt", "w") as outf:
    outf.write("\n\n".join([f"[{v[0]}]\n\"{v[1]}\"" for v in converted]))
