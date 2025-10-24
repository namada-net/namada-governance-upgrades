from html import parser
import os
import json
import subprocess
import argparse
from pathlib import Path
from typing import Dict

WITH_LOG = os.getenv("DEBUG", "0") == "1"

def serialize_proposal_json(proposal_data: Dict, wasm_path: Path, output_path: Path):
    print(f"  - Reading WASM data from '{wasm_path}'...")
    try:
        with open(wasm_path, "rb") as f:
            wasm_data = list(f.read())
    except FileNotFoundError:
        print(f"  --> ❌ ERROR: Compiled WASM file not found at '{wasm_path}'.")
        return
    except Exception as e:
        print(f"  --> ❌ ERROR: Failed to read WASM file: {e}")
        return

    # Extract the 'content' fields from the proposal metadata
    content = {
        key: proposal_data['proposal'][key]
        for key in [
            "title", "authors", "discussions-to", "license",
            "abstract", "motivation", "details", "requires"
        ] if key in proposal_data['proposal']
    }

    # Construct the final JSON structure as specified
    final_json = {
        "proposal": {
            "content": content,
            "author": proposal_data['proposal']['author'],
            "voting_start_epoch": proposal_data['proposal']['voting_start_epoch'],
            "voting_end_epoch": proposal_data['proposal']['voting_end_epoch'],
            "activation_epoch": proposal_data['proposal']['activation_epoch']
        },
        "data": wasm_data
    }

    # Serialize the structure to the output file
    try:
        with open(output_path, 'w') as f:
            json.dump(final_json, f, indent=2)
        print(f"  --> ✅ SUCCESS: Serialized proposal to '{output_path}'")
    except Exception as e:
        print(f"  --> ❌ FAILED: Could not write JSON to '{output_path}': {e}")

def build_proposal(proposal_path: Path, debug=False):
    package_name = proposal_path.name
    network_name = proposal_path.parent.name
    package = package_name.removeprefix(f"{network_name}")
    data_json_path = proposal_path / "data.json"

    print("-" * 80)
    print(f"Processing proposal: {package}")

    if not data_json_path.is_file():
        print(f"  --> ⚠️ WARNING: 'data.json' not found in '{proposal_path}'. Skipping.")
        return

    try:
        with open(data_json_path, 'r') as f:
            data = json.load(f)
        rust_version = data.get('rust-version')
        if not rust_version:
            print(f"  --> ❌ ERROR: 'rust-version' key not found in '{data_json_path}'. Skipping.")
            return
    except json.JSONDecodeError:
        print(f"  --> ❌ ERROR: Could not parse '{data_json_path}'. Invalid JSON. Skipping.")
        return
    except Exception as e:
        print(f"  --> ❌ ERROR: An unexpected error occurred while reading the file: {e}")
        return
        
    print(f"  - Found Rust version: {rust_version}")
    print(f"  - Package: {package}")

    command = [
        "earthly",
        f"--build-arg=RUST_VERSION={rust_version}",
        f"--build-arg=PACKAGE={package}",
        "+build"
    ]

    print(f"  - Executing command: {' '.join(command)}")
    try:
        if debug:
            subprocess.run(command, check=True)
        else:
            subprocess.run(command, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        print(f"  --> ✅ SUCCESS: Proposal '{package}' built successfully.")

        print(f"  - Preparing to serialize proposal output...")
        
        artifacts_dir = Path("artifacts")
        artifacts_dir.mkdir(exist_ok=True)
        
        wasm_file_name = f"{package_name.replace('-', '_')}.wasm"
        wasm_path = artifacts_dir / "wasms" / wasm_file_name

        output_json_path = artifacts_dir / f"{package_name}.json"

        serialize_proposal_json(data, wasm_path, output_json_path)
    except FileNotFoundError:
        print("  --> ❌ CRITICAL ERROR: 'earthly' command not found. Is Earthly installed and in your PATH?")
        exit(1)
    except subprocess.CalledProcessError:
        print(f"  --> ❌ FAILED: Earthly build failed for proposal '{package}'.")
        exit(1)
    except Exception as e:
        print(f"  --> ❌ FAILED: An unexpected error occurred during the build: {e}")
        exit(1)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--directory", help="Directory containing a specific proposal to build.")
    parser.add_argument("--debug", action="store_true", help="Enable debug mode with verbose output.")
    args = parser.parse_args()

    build_proposal(Path(args.directory), args.debug)


if __name__ == "__main__":
    main()
