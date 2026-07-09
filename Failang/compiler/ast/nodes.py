class Program:
    def __init__(self, statements): self.statements = statements
class Number:
    def __init__(self, value): self.value = value
class String:
    def __init__(self, value): self.value = value
class Variable:
    def __init__(self, name): self.name = name
class VariableDeclaration:
    def __init__(self, name, value): self.name = name; self.value = value
class PrintStatement:
    def __init__(self, expression): self.expression = expression
class WhileStatement:
    def __init__(self, condition, body): self.condition=condition; self.body=body
