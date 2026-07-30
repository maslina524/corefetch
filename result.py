import json
import subprocess
import sys
from pprint import pprint

cmd = subprocess.run(["fastfetch", "--json", "-c", "presets/all.jsonc"], capture_output=True, text=True, encoding="utf-8")
stdout = cmd.stdout
out_json = json.loads(stdout)

args = sys.argv

for module in out_json:
    if module["type"].lower() == args[1].lower():
        result = module["result"]
        for k in list(result.keys()):
            print(f"{k}: {result[k]}")

        sys.exit(0)