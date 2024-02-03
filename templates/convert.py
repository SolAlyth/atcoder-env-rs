
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


#from glob import glob
#from os.path import basename

#converted = []

#for fullfn in glob("./*.rs"):
#    fn = basename(fullfn)[:-3]
#    with open(f"{fn}.rs") as f:
#        converted.append((fn, convert(f.read())))
#
#with open("converted.txt", "w") as outf:
#    #outf.write("\n\n".join([f"[{v[0]}]\n\"{v[1]}\"" for v in converted]))
#    outf.write("\n\n".join([f"\"{v[1]}\"" for v in converted]))

with open("templates/template.rs", encoding="utf-8") as f:
    out = f"\"{convert(f.read())}\""

#with open("templates/converted.txt", "w", encoding="utf-8") as outf:
    # outf.write(out)

with open(".vscode/templates.code-snippets", "r", encoding="utf-8") as f:
    templates = f.readlines()

with open(".vscode/templates.code-snippets", "w", encoding="utf-8") as snippet_file:
    templates[4] = f'\t\t"body": {out}\n'
    snippet_file.writelines(templates)
