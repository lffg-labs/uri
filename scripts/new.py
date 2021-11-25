#!/usr/local/bin/python3

import re
import os
from os.path import join


def make_sub(*, code, name):
    return lambda match: {
        "code": code,
        "name": name
    }[match.group(1)]


def main(*, templates_dir, problems_dir):
    regex = re.compile("\[\[:(.+?):\]\]")

    name = input("Project name: ").strip()
    code = input("Project code: ").strip()

    base_problem = join(problems_dir, f"uri_{code}")
    os.makedirs(base_problem)

    for file in os.listdir(templates_dir):
        if file.startswith("."):
            continue

        with open(join(templates_dir, file), "r") as f:
            contents = f.read()
            generated = regex.sub(make_sub(name=name, code=code), contents)

        with open(join(base_problem, file), "w") as f:
            f.write(generated)


if __name__ == "__main__":
    ROOT = os.getcwd()
    main(
        templates_dir=join(ROOT, "scripts", "new-templates"),
        problems_dir=join(ROOT, "problems")
    )
