"""plox"""

import sys
from pathlib import Path


def run_prompt() -> None:
    while True:
        text = input("> ")
        if text == "":
            break
        run(text)


def run_file(path: str) -> None:
    text = Path(path)
    run(text)


if __name__ == '__main__':
    match sys.argv:
        case [executable]:
            run_prompt()
        case [executable, file_path]:
            run_file(file_path)
        case _:
            print("Usage: python -m plox [script]")
            sys.exit(64)  # EX_USAGE
