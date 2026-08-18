import json
import subprocess
import sys
from pprint import pprint

def get_fastfetch_module(module_name: str) -> None:
    cmd = subprocess.run(
        ["fastfetch", "--json", "-c", "presets/all.jsonc"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    )

    out_json = json.loads(cmd.stdout)

    target_type = module_name.lower()

    for module in out_json:
        if module.get("type", "").lower() == target_type:
            result = module.get("result")

            if isinstance(result, dict):
                for key, value in result.items():
                    print(f"{key}: {value}")
            elif result is not None:
                print(f"{module_name}: ", end="")
                pprint(result)
            else:
                print(f"Result is empty")

            sys.exit(0)

    print(f"Not found '{module_name}'", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(1)

    get_fastfetch_module(sys.argv[1])