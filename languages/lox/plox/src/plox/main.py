"""plox"""

from pathlib import Path

from plox.scanner import Scanner

HAD_ERROR = False


def report(line: int, where: str, message: str) -> None:
    global HAD_ERROR
    print("[line " + line + "] Error" + where + ": " + message)
    HAD_ERROR = True


def error(line: int, message: str) -> None:
    report(line, "", message)


def run(source: str) -> None:
    scanner = Scanner(source)
    tokens = scanner.scan_tokens()
    for token in tokens:
        print(token)


def run_prompt() -> None:
    global HAD_ERROR
    while True:
        text = input("> ")
        if text == "":
            break
        run(text)
        HAD_ERROR = False


def run_file(path: str) -> None:
    run(Path(path).read_text())

    # Indicate an error in the exit code.
    if HAD_ERROR:
        exit(65)  # EX_DATAERR
