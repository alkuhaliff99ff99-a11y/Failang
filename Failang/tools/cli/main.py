from compiler.lexer.lexer import Lexer
from compiler.parser.parser import Parser
from vm.interpreter import Interpreter

code = """
دع س = 1
كرر س < 5 {
    اطبع س
    دع س = س + 1
}
"""

lexer = Lexer(code)
tokens = lexer.tokenize()
print(f"Tokens: {tokens}") # هل تظهر الرموز هنا؟

parser = Parser(tokens)
prog = parser.parse()
print(f"Nodes: {len(prog.statements)}") # هل الـ Parser يرى الأوامر؟

vm = Interpreter()
vm.run(prog)

