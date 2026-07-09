import os
from pathlib import Path

# اسم المجلد الرئيسي
ROOT = Path("Failang")

# قائمة المجلدات (بما في ذلك المجلدات الفارغة)
folders = [
    "std/io", "std/math", "std/collections", "std/filesystem", "std/network",
    "std/json", "std/crypto", "std/database", "std/datetime", "std/ai",
    "tools/formatter", "tools/debugger", "tools/linter", "tools/package_manager", "tools/language_server",
    "packages",
    "tests/lexer", "tests/parser", "tests/compiler", "tests/runtime", "tests/language", "tests/performance", "tests/regression",
    "docs/tutorials",
    "assets/language"
]

# قائمة الملفات بمساراتها
files = [
    "README.md", "VISION.md", "ARCHITECTURE.md", "LICENSE", "VERSION", "CHANGELOG.md", "CONTRIBUTING.md", "failang.toml",
    
    "core/values.py", "core/types.py", "core/symbols.py", "core/environment.py", "core/modules.py", "core/config.py",
    
    "engine/pipeline.py", "engine/execution.py", "engine/scheduler.py", "engine/optimizer.py",
    
    "compiler/lexer/lexer.py", "compiler/lexer/tokens.py",
    "compiler/parser/parser.py", "compiler/parser/grammar.ebnf",
    "compiler/ast/nodes.py", "compiler/ast/expressions.py",
    "compiler/semantic/analyzer.py", "compiler/semantic/type_checker.py",
    "compiler/borrow_checker/ownership.py", "compiler/borrow_checker/lifetime.py", "compiler/borrow_checker/borrow_checker.py",
    "compiler/optimizer/optimizer.py", "compiler/optimizer/constant_folding.py",
    "compiler/backend/bytecode.py", "compiler/backend/llvm_backend.py", "compiler/backend/native_backend.py",
    "compiler/modules/importer.py", "compiler/modules/resolver.py", "compiler/modules/namespace.py",
    
    "vm/bytecode_vm.py", "vm/interpreter.py", "vm/runtime.py",
    
    "runtime/memory/allocator.py", "runtime/memory/ownership.py", "runtime/memory/lifetime.py",
    "runtime/concurrency/scheduler.py", "runtime/concurrency/channels.py", "runtime/concurrency/threads.py",
    "runtime/errors/exceptions.py", "runtime/errors/recovery.py",
    
    "tools/cli/main.py", "tools/cli/commands.py", "tools/cli/terminal.py",
    
    "examples/hello.fail", "examples/calculator.fail", "examples/web_server.fail", "examples/database.fail", "examples/ai_example.fail",
    
    "benchmarks/speed.fail", "benchmarks/memory.fail", "benchmarks/concurrency.fail",
    
    "docs/LANGUAGE_SPEC.md", "docs/LANGUAGE_IDENTITY.md", "docs/DESIGN.md", "docs/MEMORY_MODEL.md", "docs/ERROR_HANDLING.md", "docs/GRAMMAR.ebnf", "docs/ROADMAP.md",
    
    "scripts/build.sh", "scripts/test.sh", "scripts/install.sh",
    
    "assets/logo.svg",
    
    ".github/workflows/build.yml", ".github/workflows/test.yml", ".github/workflows/release.yml",
    ".github/ISSUE_TEMPLATE/bug_report.md",
    ".github/PULL_REQUEST_TEMPLATE.md"
]

def create_structure():
    # إنشاء المجلد الرئيسي
    ROOT.mkdir(exist_ok=True)

    # إنشاء المجلدات الفارغة
    for folder in folders:
        path = ROOT / folder
        path.mkdir(parents=True, exist_ok=True)

    # إنشاء الملفات والمجلدات الحاوية لها
    for file_path in files:
        path = ROOT / file_path
        path.parent.mkdir(parents=True, exist_ok=True)

        # كتابة محتوى مبدئي حسب نوع الملف
        if path.suffix == ".py":
            content = '"""Failang Module"""\n'
        elif path.suffix == ".fail":
            content = "// Failang Source Code\n"
        elif path.suffix == ".sh":
            content = "#!/bin/bash\n# Failang Script\n"
        elif path.suffix == ".md":
            content = f"# {path.stem.replace('_', ' ').title()}\n\nFailang Documentation."
        elif path.suffix == ".toml":
            content = "[package]\nname = \"failang\"\nversion = \"0.1.0\"\n"
        else:
            content = ""

        path.write_text(content, encoding="utf-8")

    print(f"✅ تم إنشاء هيكل مشروع Failang بنجاح في المسار: {ROOT.absolute()}")

if __name__ == "__main__":
    create_structure()

