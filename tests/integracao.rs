use megastore::Sistema;

#[test]
fn cadastrar_e_listar_produtos() {
    let mut sistema = Sistema::novo();

    sistema
        .cadastrar_produto(1, "Notebook".to_string(), "Eletronico".to_string())
        .expect("O cadastro deveria funcionar");

    sistema
        .cadastrar_produto(2, "Mouse".to_string(), "Eletronico".to_string())
        .expect("O cadastro deveria funcionar");

   let produtos =
   sistema.listar_produtos();

    assert_eq!(produtos.len(), 2);
}

#[test]
fn medir_desempenho_com_diferentes_volumes() {
    use std::time::Instant;

    for quantidade in [100u32, 1000, 5000] {
        let mut sistema = Sistema::novo();

        let inicio_cadastro = Instant::now();

        for id in 1..=quantidade {
            sistema
                .cadastrar_produto(
                    id,
                    format!("Produto {}", id),
                    "Teste".to_string(),
                )
                .expect("O cadastro deveria funcionar");
        }

        let tempo_cadastro = inicio_cadastro.elapsed();

        let inicio_listagem = Instant::now();
        let produtos = sistema.listar_produtos();
        let tempo_listagem = inicio_listagem.elapsed();

        println!(
            "Produtos: {} | Cadastrados: {} | Cadastro: {:?} | Listagem: {:?}",
            quantidade,
            produtos.len(),
            tempo_cadastro,
            tempo_listagem
        );

        assert_eq!(produtos.len(), quantidade as usize);
    }
}