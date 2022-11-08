from dataclasses import dataclass

from plox.token_types import TokenType


@dataclass(frozen=True, slots=True)
class Token:
    type: TokenType
    lexeme: str
    literal: object
    line: int
