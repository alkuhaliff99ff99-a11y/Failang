from enum import Enum, auto
class TokenType(Enum):
    NUMBER=auto(); STRING=auto(); IDENTIFIER=auto(); LET=auto(); PRINT=auto()
    PLUS=auto(); MINUS=auto(); MULTIPLY=auto(); DIVIDE=auto(); EQUAL=auto(); EOF=auto()
    IF=auto(); ELSE=auto(); WHILE=auto(); FUNCTION=auto(); RETURN=auto()
    COMMA=auto(); LEFT_PAREN=auto(); RIGHT_PAREN=auto(); LBRACE=auto(); RBRACE=auto(); LESS=auto()

KEYWORDS = {
    "دع": TokenType.LET, "let": TokenType.LET, "اطبع": TokenType.PRINT, "print": TokenType.PRINT,
    "إذا": TokenType.IF, "if": TokenType.IF, "وإلا": TokenType.ELSE, "else": TokenType.ELSE,
    "كرر": TokenType.WHILE, "while": TokenType.WHILE, "دالة": TokenType.FUNCTION, "fn": TokenType.FUNCTION,
    "أرجع": TokenType.RETURN, "return": TokenType.RETURN
}

class Token:
    def __init__(self, type_, value=None): self.type=type_; self.value=value
    def __repr__(self): return f"{self.type.name}:{self.value}"
