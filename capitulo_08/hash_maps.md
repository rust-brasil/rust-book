# Hash Maps

## Definição / características
- Estrutura de dados que armazena chaves e valores
- É conhecida em outras linguagens com outros nomes: objeto (Javascript), dicionário (Python), array associativo (PHP) etc.
- Estrutura inventada em 1953 (hash table), amplamente melhorada desde então, especialmente quanto aos algoritimos de hashing e uso de eficiente de memória e processamento.
- É utilizada quando o programador precisa obter os valores de acordo com uma chave que pode ser de qualquer tipo.
- Em Rust, não possui tanto suporte como os outros tipos de coleções: não há macros para a sua criação e não está no Prelúdio, sendo necessário importá-la:
```rust
use std::collections::HashMap;
```
## Criação e inicialização
### Criação de um hash map vazio e população direta com o `insert`
```rust
use std::collections::HashMap;

fn main() {
    // Definição
    let mut book_shelves = HashMap::new();
    // Inserção
    book_shelves.insert(
        String::from("Para Ler"),
        vec![String::from("Como treinar o seu próprio dragão")],
    );
    book_shelves.insert(
        String::from("Lendo"),
        vec![String::from("Raízes de mandrágora e suas propriedades")],
    );
    book_shelves.insert(
        String::from("Lidos"),
        vec![String::from("O livro invisível da invisibilidade")],
    );
    
    println!("{:?}", book_shelves);
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=68650694440240703a3dc28e1b5ae4d8

### Criação e população a partir de um vetor de tuplas chave-valor
```rust
use std::collections::HashMap;

fn main() {
    // Definição a partir de um vetor de tuplas chave-valor
    let book_shelves: HashMap<String, Vec<String>> = vec![
        ("Para ler".to_string(), vec!["Moby Dick".to_string()]),
        ("Lendo".to_string(), vec!["Histórias Fantásticas".to_string()]),
    ]
    .into_iter()
    .collect();
    // Impressão
    println!("{:#?}", book_shelves)
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9765d507b584ab12c1e3ee5cc8ef8132

## *Ownership* e inserção de referências
- "Para tipos que implementam o trait `Copy`, como i32, os valores são copiados para dentro do hash map. Para valores que trabalham com o conceito de propriedade, como `String`, os valores serão movidos para o hash map, que será o proprietário deles":

```rust
use std::collections::HashMap;

fn main() {
    let mut book_shelves: HashMap<String, String> = HashMap::new();

    // Impressão
    println!("{:#?}", book_shelves);

    // O hashmap possui os valores inseridos
    let um_livro_especial = "O conde de Monte Cristo".to_string();
    book_shelves.insert(String::from("Favoritos"), um_livro_especial);
    // um_livro_especial agora é propriedade de book_shelves e,
    // portanto, qualquer referência a ela deixa de existir a partir daqui
    println!("{}", um_livro_especial); //NÃO FUNCIONA
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8a6bc7de892e853879a0ae96cfabb426

- "Se inserirmos referências num hash map, os valores não serão movidos para dentro dele. Os valores para os quais as referências apontam devem ser válidos pelo menos enquanto o hash map for válido":

```rust
#![allow(unused)]
use std::collections::HashMap;

fn insere_outro_livro_especial(book_shelves: &mut HashMap<&str, &str>) {
    let outro_livro_especial = "O mágico de Oz".to_string();
    book_shelves.insert("Favoritos", &outro_livro_especial);
    
}   // Isto não funciona porque outro_livro_especial perde o escopo aqui e,
    // no entanto, sua referência foi adicionada ao hash map que sobrevive.
    // O compilador nos impede de ter ponteiros pendurados.

fn main() {
    let mut book_shelves: HashMap<&str, &str> = HashMap::new();
    
    insere_outro_livro_especial(&mut book_shelves);

    // Impressão
    println!("{:#?}", book_shelves);
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5a69783d7d27aa19d140504350b470e2

## Pegando o valor de uma chave
- Assim como com o vetor, o hash maps tem o método `get()` que retorna uma enum `Option` com o valor, se presente, `Some` ou ausente, `None` - podemos utilizar um bloco `match` para desempacotar o valor do enum:

```rust
use std::collections::HashMap;

fn main() {
    let mut book_shelves = HashMap::new();

    book_shelves.insert(
        String::from("Para ler"),
        String::from("Rust completo e total"),
    );
    book_shelves.insert(String::from("Lendo"), String::from("Python Fluente"));
    book_shelves.insert(String::from("Lidos"), String::from("C completo e Total"));

    // Pegando o valor de uma chave
    let categoria = String::from("Lidos");
    let lidos = book_shelves.get(&categoria);
    match lidos {
        Some(livro) => println!("{}", livro),
        None => println!("Nenhum livro na categoria {}", &categoria),
    }
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f8c12e99050464b92f20bd8ab1e069d2


## Iterando sobre chaves e valores de um hash map
- A iteração exatamente igual a que vimos nos vetores com a adição que cada item é uma tupla chave-valor, que pode ser desempacotada no laço `for`:

```rust
use std::collections::HashMap;

fn main() {
    let mut book_shelves = HashMap::new();

    book_shelves.insert(
        String::from("Para ler"),
        String::from("Rust completo e total"),
    );
    book_shelves.insert(String::from("Lendo"), String::from("Python Fluente"));
    book_shelves.insert(String::from("Lidos"), String::from("C completo e Total"));

    // Iterando sobre chaves e valores de um vetor
    for (categoria, livro) in book_shelves {
        println!("Livro '{}' na categoria '{}'", livro, categoria)
    }
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6e61e338729e7bf694af94fca6deab07

## Estratégias de atualização de valores

### 1. Substituindo um valor existente
- `insert()`, se chamado com uma chave que já existe no hashmap, irá atualizar o seu valor ao invés de dar um erro. Para alguns isso parece contraintuitivo, talvez a pessoa esperasse um método `update` ou similar, mas esta é a forma como os hashmaps são implementados em Rust.

```rust
use std::collections::HashMap;

fn main() {
    let mut book_shelves = HashMap::new();

    book_shelves.insert(
        String::from("Para ler"),
        vec![String::from("Rust completo e total")],
    );
    book_shelves.insert(String::from("Lendo"), vec![String::from("Python Fluente")]);
    book_shelves.insert(
        String::from("Lidos"),
        vec![String::from("C completo e Total")],
    );

    // Estratégias para atualizar um valor
    //1. Substituindo um valor existente
    let categoria_para_ler = String::from("Para ler");
    println!(
        "ANTES: {:?}",
        book_shelves.get(&categoria_para_ler).unwrap()
    );
    book_shelves.insert(
        String::from("Para ler"),
        vec![String::from("Rust vs Go: o dilema do Devops bitolado")],
    );
    println!(
        "DEPOIS: {:?}",
        book_shelves.get(&categoria_para_ler).unwrap()
    );
}

```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=82c26763b1f1477929539eae108d603a

### 2. Somente atualizando se a chave não tiver valor associado
- Rust disponibiliza a [Entry API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) para lidar com entradas num hash map. No exemplo abaixo, utilizamos a função or_insert, que será executará o `insert` (atualizará uma chave) apenas se esta chave não existir:
```rust
use std::collections::HashMap;

fn main() {
    let mut book_shelves = HashMap::new();

    book_shelves.insert(
        String::from("Para ler"),
        vec![String::from("Rust completo e total")],
    );
    book_shelves.insert(String::from("Lendo"), vec![String::from("Python Fluente")]);
    book_shelves.insert(
        String::from("Lidos"),
        vec![String::from("C completo e Total")],
    );

    // Estratégias para atualizar um vetor
    //2. Somente atualizando se a chave não tiver valor associado
    let categoria_para_ler = String::from("Para ler");
    println!(
        "ANTES: {:?}",
        book_shelves.get(&categoria_para_ler).unwrap()
    );
    book_shelves
        .entry(String::from("Para ler"))
        .or_insert(vec![String::from("Rust vs Go: o dilema do Devops")]);
    println!(
        "DEPOIS: {:?}\nOBSERVE QUE O VALOR NÃO MUDOU.",
        book_shelves.get(&categoria_para_ler).unwrap()
    );
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4deaf675dce0f007d9f530b1b4034b78

### 3. Atualizando com base num valor anterior
- A função `or_insert` da Entry API retorna uma referência mutável à entrada inserida ou existente. Neste caso, para cada letra na frase criamos uma entrada e a populamos com 0, para depois incrementar este valor logo em seguida ou no caso do próxima iteração com o valor já existente se for o caso:

```rust
#![allow(unused)]
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let text = "Quando Gregor Samsa despertou, certa manhã, de um sonho agitado, viu que se transformara, em sua cama, numa espécie monstruosa de inseto.";
    let mut words = HashMap::new();
    // Estratégias para atualizar um vetor
    //3. Atualizando com base num valor anterior
    for letter in text.chars() {
        // Se a chave não existir, insere com o valor 0;
        // existindo ou não, retorna uma referência mutável da chave
        let count = words.entry(letter).or_insert(0);
        // Utilizamos esta referência para incrementar o valor associado
        *count += 1;

        // Também podemos usar o match com o resultado de entry(), que é
        // um enum. O código abaixo faz o mesmo que acima:
        /*
        match words.entry(letter) {
            Entry::Occupied(mut entry) => {
                // A entrada já está ocupada, pegue a referência mutável
                // e incremente o seu valor associado
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                // Não existe esta entrada, insira uma nova com o valor 1
                entry.insert(1);
            }
        }
        */
        
    }
    println!("{:#?}", words);
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3435349e7cd3341113670e0c529ab385
