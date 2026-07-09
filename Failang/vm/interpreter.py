from compiler.lexer.tokens import TokenType
from compiler.ast.nodes import *
class Interpreter:
    def __init__(self): self.vars = {}
    def run(self, prog):
        for stmt in prog.statements: self.execute(stmt)
    def execute(self, node):
        if isinstance(node, VariableDeclaration): self.vars[node.name] = self.evaluate(node.value)
        elif isinstance(node, PrintStatement): print(self.evaluate(node.expression))
        elif isinstance(node, WhileStatement):
            while self.evaluate(node.condition):
                for s in node.body: self.execute(s)
    def evaluate(self, node):
        if isinstance(node, Number): return node.value
        if isinstance(node, Variable): return self.vars.get(node.name)
        if isinstance(node, BinaryOperation): # بسيط للمقارنة
            l = self.evaluate(node.left); r = self.evaluate(node.right)
            return l < r # كمثال بسيط للشرط
