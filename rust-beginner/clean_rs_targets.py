# This script traverse the directory and runs `cargo clean` on all the directories that contain `Cargo.toml` files.

import os
import subprocess

def unit_to_bytes(unit):
    if unit == "KiB":
        return 1024
    elif unit == "MiB":
        return 1024 * 1024
    elif unit == "GiB":
        return 1024 * 1024 * 1024
    else:
        assert False, "Unknown unit: " + unit

def parse_size(size_str):
    # The size string looks like this: {size}{unit}
    # The unit can be KiB, MiB, GiB
    # We want to convert the size to bytes

    # Split the string into the size and the unit
    size_str = size_str.strip()
    size = size_str[:-3]
    unit = size_str[-3:]
    # Convert the size to an integer
    size = float(size)
    # Convert the unit to bytes
    unit = unit_to_bytes(unit)
    # Multiply the size by the unit
    size = size * unit
    return size

def build_rs_targets():
    # This function builds all the targets
    # Useful only for testing clean_rs_targets.py instead of manually building targets
    for root, _, files in os.walk("."):
        if "Cargo.toml" in files:
            print("Building: " + root)
            # Run `cargo build` in the directory, and collect the output
            _ = subprocess.run(["cargo", "build"], cwd=root, capture_output=True)

def clean_rs_targets():
    total_files_removed = 0
    total_space_freed = 0.0
    for root, _, files in os.walk("."):
        if "Cargo.toml" in files:
            print("Cleaning: " + root)
            # Run `cargo clean` in the directory, and collect the output
            output = subprocess.run(["cargo", "clean"], cwd=root, capture_output=True)
            # Check if the command was successful
            if output.returncode != 0:
                print("Failed to clean: " + root)
                print(output.stderr)
            else:
                # Read the output and collect how many files were removed and how much space was freed
                output_str = output.stderr.decode("utf-8")
                lines = output_str.split("\n")
                for line in lines:
                    # The line we're looking for looks like this:
                    #    Removed {file_count} files [, {space_freed} total]
                    line = line.strip()
                    if line.startswith("Removed"):
                        parts = line.split(" ")
                        file_count = parts[1]
                        if len(parts) == 5:
                            space_freed = parts[3]
                        else:
                            space_freed = "0KiB"
                        space_freed = str(parse_size(space_freed))
                        total_files_removed += int(file_count)
                        total_space_freed += float(space_freed)
                        break
    print("Total files removed: " + str(total_files_removed))
    print("Total space freed: " + str(total_space_freed) + " bytes (" + str(total_space_freed / (1024 * 1024)) + " MiB)")
    
if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        clean_rs_targets()
    elif sys.argv[1] == "build":
        print("Why would you want to build all the targets? Bro...", file=sys.stderr)
        exit(1)
        build_rs_targets()
    elif sys.argv[1] == "clean":
        clean_rs_targets()