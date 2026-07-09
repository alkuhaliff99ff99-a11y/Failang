from compiler.lexer.tokens import TokenType, Token, KEYWORDS
class Lexer:
    def __init__(self, text): self.text=text; self.position=0
    def tokenize(self):
        tokens = []
        while self.position < len(self.text):
            char = self.text[self.position]
            if char.isspace(): self.position+=1; continue
            if char.isdigit():
                num = ""
                while self.position < len(self.text) and self.text[self.position].isdigit(): num += self.text[self.position]; self.position+=1
                tokens.append(Token(TokenType.NUMBER, int(num))); continue
            if char.isalpha() or "\u0600" <= char <= "\u06FF":
                w = ""
                while self.position < len(self.text) and (self.text[self.position].isalnum() or "\u0600" <= self.text[self.position] <= "\u06FF" or self.text[self.position] == "_"):
                    w += self.text[self.position]; self.position+=1
                tokens.append(Token(KEYWORDS.get(w, TokenType.IDENTIFIER), w)); continue
            if char == "{": tokens.append(Token(TokenType.LBRACE)); self.position+=1; continue
            if char == "}": tokens.append(Token(TokenType.RBRACE)); self.position+=1; continue
            if char == "<": tokens.append(Token(TokenType.LESS)); self.position+=1; continue
            if char == "=": tokens.append(Token(TokenType.EQUAL)); self.position+=1; continue
            if char == "+": tokens.append(Token(TokenType.PLUS)); self.position+=1; continue
            self.position+=1
        tokens.append(Token(TokenType.EOF))
        return tokens
