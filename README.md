# Runic

<img src="./docs/images/logo.svg" alt="Runic Logo" width="120"/>

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)](https://github.com/SpideyZac/runic/actions?workflow=CI)
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)](./LICENSE)

Runic is a minimalist Rust library for building lexers, parsers, interpreters, transpilers, and compilers. It provides the raw tools needed to construct language tooling from scratch, without relying on parser generators, grammar files, or heavyweight abstractions.

Inspired by the raw expressiveness of early programming tools and languages, Runic helps you forge your own compilers and interpreters using simple, explicit, and composable components.

Whether you’re building a hobby language, analyzing real-world code, or crafting a custom runtime for embedded scripting, Runic provides the bones, you bring the soul.

## ✨ Philosophy

Runic is designed with the following core principles:

* **🧱 Explicit > Implicit**: You build everything using explicit components. There are no hidden magic or implicit behaviors. Control flow and data flow are clear and visible.
* **🪄 Simple Macros Only**: Runic uses simple macros to provide basic functionality and remove boilerplate. No complex macro systems or DSLs.
* **🦴 Bare-bones by Design**: Runic is intentionally minimal. It provides the essential building blocks without unnecessary abstractions or features. You can add complexity as needed.
* **🔬 Fully Inspectable**: Every component is fully inspectable and debuggable. You can see exactly how your code is structured and how it behaves at runtime.

## 🧠 Why Use Runic?

* You're building a compiler, interpreter, or transpiler from scratch and want full control over the process.
* You don't want to learn or maintain `.lalr`/`.peg` grammar files.
* You prefer writing struct-based, idiomatic Rust over configuring a codegen tool.
* You want a toolkit, not a framework. Runic provides the tools, you build the framework.

## 🛠️ Status & Roadmap

* 🟢 Error printing.
* 🟢 Basic lexer & token components.
* 🟡 Lexer utilities and common features.
* 🔴 Basic parsing components.
* 🔴 Parser utilities and common features.
* 🔴 Codegen, etc. (TBD).

## 📦 Usage

(TBD)
