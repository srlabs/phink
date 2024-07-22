import json
import os
import argparse
import subprocess


def read_hash_from_json(file_path):
    try:
        with open(file_path, 'r') as file:
            data = json.load(file)
            hash_value = data['source']['hash']
            return hash_value
    except (FileNotFoundError, KeyError, json.JSONDecodeError) as e:
        print(f"Error reading {file_path}: {e}")
        return None


def print_hash(label, hash_value):
    if hash_value:
        print(f"{label} -- {hash_value}")


def main():
    parser = argparse.ArgumentParser(description="Extract and print hash from JSON files")
    parser.add_argument('--directory', type=str, default='target/ink', help='Base directory containing the JSON files')
    args = parser.parse_args()

    files_and_labels = [
        ('accumulator/accumulator.json', 'accumulator'),
        ('subber/subber.json', 'subber'),
        ('adder/adder.json', 'adder')
    ]

    hashes = {}
    for file, label in files_and_labels:
        file_path = os.path.join(args.directory, file)
        hash_value = read_hash_from_json(file_path)
        if hash_value:
            hashes[label] = hash_value
            print_hash(label, hash_value)

    if len(hashes) == 3:
        accumulator_hash = hashes['accumulator']
        adder_hash = hashes['adder']
        subber_hash = hashes['subber']

        command = [
            'cargo', 'contract', 'encode', '--message', 'new', '--args',
            '4444', '123',
            accumulator_hash,
            adder_hash,
            subber_hash,
            '--', os.path.join(args.directory, 'multi_contract_caller.json')
        ]

        try:
            subprocess.run(command, check=True)
            print("Command executed successfully.")
        except subprocess.CalledProcessError as e:
            print(f"Command failed with error: {e}")


if __name__ == "__main__":
    main()
