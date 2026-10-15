use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

pub struct Sistema {
    pub produtos: HashMap<u32, Produto>,
    pub grafo: HashMap<u32, Vec<u32>>,
}

impl Sistema {
    pub fn novo() -> Self {
        Self {
            produtos: HashMap::new(),
            grafo: HashMap::new(),
        }
    }

    // 1. Cadastrar produto
    pub fn cadastrar_produto(
        &mut self,
        id: u32,
        nome: String,
        categoria: String,
    ) -> Result<(), String> {
        if self.produtos.contains_key(&id) {
            return Err("Esse ID ja existe.".to_string());
        }

        if nome.trim().is_empty() || categoria.trim().is_empty() {
            return Err("Preencha nome e categoria.".to_string());
        }

        self.produtos.insert(
            id,
            Produto {
                id,
                nome,
                categoria,
            },
        );

        self.grafo.entry(id).or_default();

        Ok(())
    }

    // 2. Conectar dois produtos no grafo
    pub fn conectar_produtos(
        &mut self,
        id_a: u32,
        id_b: u32,
    ) -> Result<(), String> {
        if id_a == id_b {
            return Err("Escolha dois produtos diferentes.".to_string());
        }

        if !self.produtos.contains_key(&id_a)
            || !self.produtos.contains_key(&id_b)
        {
            return Err("Cadastre os dois produtos primeiro.".to_string());
        }

        let vizinhos_a = self.grafo.entry(id_a).or_default();

        if !vizinhos_a.contains(&id_b) {
            vizinhos_a.push(id_b);
        }

        let vizinhos_b = self.grafo.entry(id_b).or_default();

        if !vizinhos_b.contains(&id_a) {
            vizinhos_b.push(id_a);
        }

        Ok(())
    }

    // 3. Consultar um produto pelo ID
    pub fn consultar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    // 4. Listar todos os produtos
    pub fn listar_produtos(&self) -> Vec<&Produto> {
        let mut lista: Vec<&Produto> =
            self.produtos.values().collect();

        lista.sort_by_key(|produto| produto.id);

        lista
    }

    // 5. Recomendar produtos usando BFS
    pub fn recomendar_produtos(
        &self,
        produto_id: u32,
        limite: usize,
    ) -> Result<Vec<Produto>, String> {
        if !self.produtos.contains_key(&produto_id) {
            return Err("Produto nao encontrado.".to_string());
        }

        if limite == 0 {
            return Ok(Vec::new());
        }

        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        fila.push_back(produto_id);
        visitados.insert(produto_id);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.grafo.get(&atual) {
                for vizinho in vizinhos {
                    if visitados.insert(*vizinho) {
                        fila.push_back(*vizinho);

                        if let Some(produto) =
                            self.produtos.get(vizinho)
                        {
                            recomendacoes.push(produto.clone());

                            if recomendacoes.len() >= limite {
                                return Ok(recomendacoes);
                            }
                        }
                    }
                }
            }
        }

        Ok(recomendacoes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testar_cadastro_e_listagem() {
        let mut sistema = Sistema::novo();

        assert!(sistema.cadastrar_produto(
            1,
            "Notebook".to_string(),
            "Eletronico".to_string()
        ).is_ok());

        assert_eq!(sistema.listar_produtos().len(), 1);
    }

    #[test]
    fn testar_id_duplicado() {
        let mut sistema = Sistema::novo();

        sistema.cadastrar_produto(
            1,
            "Notebook".to_string(),
            "Eletronico".to_string()
        ).unwrap();

        assert!(sistema.cadastrar_produto(
            1,
            "Mouse".to_string(),
            "Eletronico".to_string()
        ).is_err());
    }
}