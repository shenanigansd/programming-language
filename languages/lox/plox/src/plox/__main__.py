import sys

from plox.main import run_file, run_prompt

if __name__ == "__main__":
    match sys.argv:
        case [executable]:
            run_prompt()
        case [executable, file_path]:
            run_file(file_path)
        case _:
            print("Usage: python -m plox [script]")
            sys.exit(64)  # EX_USAGE
