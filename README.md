ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Objetivo

O ConectaStore é um projeto acadêmico desenvolvido em Rust que demonstra como grafos podem representar relações entre produtos e apoiar recomendações.

Tecnologias

- Rust
- Cargo
- Visual Studio Code
- Git e GitHub

Estruturas de dados

- Grafo para representar produtos e suas conexões.
- HashMap para organizar e localizar informações.
- Vec, HashSet e VecDeque como estruturas auxiliares, conforme utilizadas no código.

Funcionalidades

- Cadastro de produtos.
- Listagem de produtos.
- Conexão entre produtos.
- Recomendação de produtos relacionados.
- Verificação de identificadores duplicados.

Como executar

Com Rust instalado, abra o terminal na pasta do projeto e execute:

cargo run

Como executar os testes

Na pasta do projeto, execute:

cargo test

Estrutura do projeto

- "src/main.rs": entrada principal e menu do sistema.
- "src/lib.rs": lógica do sistema e testes unitários.
- "tests/integracao.rs": testes de integração.
- "Cargo.toml": configuração do pacote Rust.
- "Cargo.lock": registro das dependências resolvidas.

Desempenho

Foram realizadas medições iniciais de cadastro e listagem com 100, 1.000 e 5.000 produtos. Os tempos observados estão documentados no relatório acadêmico. São medições iniciais, não uma avaliação completa de todas as operações do grafo.

Relatório e vídeo

Relatório acadêmico: entregue em PDF na plataforma da faculdade.

Vídeo de apresentação: link será adicionado após a publicação.

Autor

Projeto acadêmico desenvolvido para a disciplina Data Structure Strategy and Implementation.
