use megastore::Sistema;
use std::io::{self, Write};

fn ler(mensagem: &str) -> String {
    print!("{mensagem}");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    entrada.trim().to_string()
}

fn main() {
    let mut sistema = Sistema::novo();

    loop {
        println!("\n=== CONECTASTORE ===");
        println!("1 - Cadastrar produto");
        println!("2 - Listar produtos");
        println!("3 - conectar dois produtos");
        println!("4 - recomendar produtos");
        println!("0 - Sair");

        let opcao = ler("Escolha uma opção: ");

        match opcao.as_str() {
            "1" => {
                let id: u32 = match ler("ID do produto: ").parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Digite um ID numérico válido.");
                        continue;
                    }
                };

                let nome = ler("Nome do produto: ");
                let categoria = ler("Categoria: ");

                match sistema.cadastrar_produto(id, nome, categoria) {
                    Ok(()) => println!("Produto cadastrado com sucesso!"),
                    Err(erro) => println!("Erro: {erro}"),
                }
            }

            "2" => {
                if sistema.produtos.is_empty() {
                    println!("Nenhum produto cadastrado.");
                } else {
                    for produto in sistema.produtos.values() {
                        println!(
                            "ID: {} | Nome: {} | Categoria: {}",
                            produto.id, produto.nome, produto.categoria
                        );
                    }
                }
            }

"3" => {

    let id_a: u32 = match ler("ID do primeiro produto: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("ID inválido.");
            continue;
        }
    };

    let id_b: u32 = match ler("ID do segundo produto: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("ID inválido.");
            continue;
        }
    };

    match sistema.conectar_produtos(id_a, id_b) {
        Ok(()) => println!("Produtos conectados no grafo!"),
        Err(erro) => println!("Erro: {erro}"),
    }
}

    "4" => {
    let id: u32 = match ler("ID do produto de referencia: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("ID invalido.");
            continue;
        }
    };

    match sistema.recomendar_produtos(id, 10) {
        Ok(lista) if lista.is_empty() => {
            println!("Nenhuma recomendacao encontrada.");
        }
        Ok(lista) => {
            println!("\nProdutos recomendados:");

            for produto in lista {
                println!(
                    "ID: {} | Nome: {} | Categoria: {}",
                    produto.id,
                    produto.nome,
                    produto.categoria
                );
            }
        }
        Err(erro) => println!("Erro: {}", erro),
    }
}


            "0" => {
                println!("Programa encerrado.");
                break;
            }

            _ => println!("Opção inválida."),
        }
    }
}