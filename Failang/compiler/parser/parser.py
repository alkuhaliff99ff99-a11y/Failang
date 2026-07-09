from compiler.lexer.tokens import TokenType
from compiler.ast.nodes import Program, VariableDeclaration, PrintStatement, WhileStatement, BinaryOperation, Number, Variable

class Parser:
    def __init__(self, tokens): self.tokens = tokens; self.position = 0
    def current(self): return self.tokens[self.position] if self.position < len(self.tokens) else None
    def advance(self): self.position += 1
    def parse(self):
        stmts = []
        while self.current() and self.current().type != TokenType.EOF:
            stmt = self.statement()
            if stmt: stmts.append(stmt)
        return Program(stmts)
    def statement(self):
        token = self.current()
        if not token: return None
        if token.type == TokenType.LET:
            self.advance(); name = self.current().value; self.advance(); self.advance(); val = self.expression()
            return VariableDeclaration(name, val)
        if token.type == TokenType.PRINT:
            self.advance(); return PrintStatement(self.expression())
        if token.type == TokenType.WHILE:
            self.advance(); cond = self.expression(); self.advance() # LBRACE
            body = []
            while self.current() and self.current().type != TokenType.RBRACE: body.append(self.statement())
            self.advance() # RBRACE
            return WhileStatement(cond, body)
        self.advance(); return None
    def expression(self):
        left = self.primary()
        if self.current() and self.current().type in [TokenType.PLUS, TokenType.LESS]:
            op = self.current(); self.advance(); return BinaryOperation(left, op, self.primary())
        return left
    def primary(self):
        token = self.current(); self.advance()
        if token.type == TokenType.NUMBER: return Number(token.value)
        if token.type == TokenType.IDENTIFIER: return Variable(token.value)
        return None
