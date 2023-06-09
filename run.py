#!/usr/bin/env python3
import os
import toml
import clize
import subprocess
import shutil

abspath = os.path.abspath
dirname = os.path.dirname
path_join = os.path.join

BASE_DIR = dirname(abspath(__file__))
SRC_DIR = path_join(BASE_DIR, "src")
TOML_FILE = path_join(BASE_DIR, "Cargo.toml")
TEMPLATE_FILE = path_join(BASE_DIR, "template/template.rs")


def configure():
    bins = next(os.walk(SRC_DIR))[1]
    toml_data = toml.load(TOML_FILE)

    toml_data['bin'] = [
        {'name': x, 'path': path_join(SRC_DIR, f'{x}/main.rs')} for x in bins
    ]
    with open(TOML_FILE, 'w') as f:
        toml.dump(toml_data, f)


def execute(bin_name: str):
    exit_code = subprocess.call(
        ['cargo', 'run', '--release', '--bin', bin_name])


def new(bin_name: str):
    new_proj_dir = path_join(SRC_DIR, bin_name)
    if os.path.exists(new_proj_dir):
        print(f"path at \"{new_proj_dir}\" already exists")
        return

    os.mkdir(new_proj_dir)
    shutil.copy(TEMPLATE_FILE, path_join(new_proj_dir, 'main.rs'))
    configure()


if __name__ == '__main__':
    clize.run(configure, execute, new)
