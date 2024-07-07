# Introdução

Este projeto de programação fornece uma calculadora simples de linha de comando que permite aos usuários escrever e avaliar expressões matemáticas usando a crate `meval` em Rust.

# Organização do projeto:

As pastas e arquivos deste projeto são os seguintes:

* `src/main.rs`: Este arquivo contém a implementação principal da calculadora. Ele lê a entrada do usuário, processa as expressões e as avalia usando `meval`.
* `Cargo.toml`: Este arquivo especifica as dependências e os metadados para o projeto **Rust**.
* `README.md`: Este arquivo.

# Running the Calculator

Para rodar esse programa você deve ter o [Rust](https://www.rust-lang.org/) instalado. Siga as instruções para executar o projeto:

1. Clone o repositório com o **Git**:
   ```sh
   git clone https://github.com/ianco-so/rust-calculator.git
   cd rust-calculator
   ```
2. Execute o projeto:

    Faça o build do projeto com **Cargo**:
    ```sh
    cargo build
    ```
    Rode o projeto com **Cargo***:
    ```sh
    cargo run
    ```
    Alternativamente, você pode construir e executar o projeto sem o Cargo.
    Para compilar o programa com o **rustc**:
    ```sh
    rustc src/main.rs
    ```
    Execute o programa:
    ```sh
    ./main # ou main.exe no Windows
    ```
# Uso

A calculadora suporta operações aritméticas básicas e funções. Aqui estão alguns exemplos de expressões válidas:

```sh
$ x = 10
resultado: 10
$ x + 2
resultado: 12
$ x / b
error: Erro avaliando a expressão
$ b = x + 12
resultado: 22
$ x = b / x
resultado: 2.2
$ x
resultado: 2.2
```

# Dependências

O projeto usa as seguintes dependências:

[**meval:**](https://crates.io/crates/meval) Uma crate para analisar e avaliar expressões matemáticas em Rust. Está especificada no arquivo Cargo.toml.
Você pode adicionar mais dependências conforme necessário, atualizando o arquivo Cargo.toml.

# Autoria

Programa desenvolvido por Ianc (<ianco.so@gmail.com>), 2024.1

&copy; IMD/UFRN.