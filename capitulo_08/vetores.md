# Vetores

## Definição / características
- Coleção de itens do mesmo tipo
- Array com tamanho dinâmico, ou seja, que pode crescer em tempo de execução
- Itens são armazenados na memória `heap`, de forma contígua, isto é, um próximo ao outro, sequencialmente.

## Aplicação
- Armazenamento de itens definidos em tempo de execução
- Exemplos: lista de itens num carrinho de compras, as linhas de um arquivo, valores produzidos pelo usuário etc.

## Estrutura
- Um vetor é sempre um trio composto por um ponteiro, a capacidade e o tamanho
- O ponteiro (`ptr`) aponta para os dados armazenados pelo vetor na `heap`
- O tamanho (`len`) indica a ocupação atual do vetor, ou seja, quantos itens foram inseridos nele
- A capacidade (`capacity`) indica quantos itens são possíveis de serem inseridos no vetor sem que aconteça uma *realocação*
- A *realocação* acontece sempre que `len` >= `capacity`

## Definição e inicialização

### Definição

Definindo dois vetores para elementos `i32` vazios:

```rust
let vetor1: Vec<i32> = Vec::new();
let vetor2: Vec<i32> = vec![];
```

Se no mesmo escopo não formos inserir itens nos vetores, precisamos anotá-lo com o tipo de dados que ele armanezará.

### Definição e inicialização ao mesmo tempo

```rust
let mut vetor1 = Vec::new();
vetor1.push(1); // método push insere um elemento no final do vetor
vetor1.push(2);
```

```rust
let vetor2 = vec![1, 2]; // macro vec! criou e inseriu os elementos de uma vez só
```

Observe que nos dois casos acima o compilador inferiu o tipo do vetor de acordo com os itens inseridos.

## Acesso aos itens
### Diretamente
Vetores são indexados por números, começando pelo zero e, tal como arrays, e podem ser acessados com a notação `&vec[<indice>]`:

```rust
let mut vetor2 = vec![1, 2, 3, 4];
let terceiro_elemento = &vetor2[2];
println!("{:#?}", terceiro_elemento); // 3
```

Acessar um elemento fora dos limites do vetor gerará um erro irrecuperável em tempo de execução (panic). Isso pode ser útil se for o comportamento esperado para a sua aplicação, por exemplo, um parâmetro de configuração necessário para o seu funcionamento não tem o tamanho esperado.

### Com a função `get()`:

```rust
let mut vetor2 = vec![1, 2, 3, 4];
let terceiro_elemento = &vetor2.get(2);
println!("{:#?}", terceiro_elemento); // Some(3)
```

A função `get()` retorna uma variante de `Option`: ou o valor encapsulado em `Some`, ou `None` se não houver valor.

```rust
let mut vetor2 = vec![1, 2, 3, 4];
let quinto_elemento = &vetor2.get(4);
println!("{:#?}", quinto_elemento); // None
```
Esta forma de acessar os elementos é mais conveniente quando você quer lidar com possível erro em tempo de execução.

## Atualizando um vetor
- É possível inserir um elemento no final com o método `push` conforme o exemplo acima.
- É possível remover um elemento do final com método `pop`, que retorna Some(<valor>) enquanto houver valores, ou `None`, quando todos os valores tiverem sido retirados:

```rust
let mut vetor2 = vec![1, 2];
let segundo_retirado = vetor2.pop();
println!("{:#?}", &segundo_retirado); // Some(2)
```
- É possível inserir um elemento no índice `x` com a função `insert`:
```rust
let mut vetor2 = vec![1, 2];
vetor2.insert(1, 3);
println!("{:?}", &vetor2); // [1, 3, 2]
```
- É possível remover um elemento no índice `x` com a função `remove`.
- É possível alterar os elementos de um vetor diretamente:

```rust
let mut vetor2 = vec![1, 2];
vetor2[1] = 3;
println!("{:?}", &vetor2); // [1, 3]
```

## O vetor é dono de seus elementos
- Ao sair de escopo, o vetor e seus elementos são destruídos.

## Iterando elementos:

### Iteração imutável
```rust
let serie_fibonacci = vec![0, 1, 1, 2, 3, 5, 8, 13];
for elemento in serie_fibonacci {
  println!("{}", elemento);
}
```

### Iteração mutável
```rust
let mut serie_fibonacci = vec![0, 1, 1, 2, 3, 5, 8, 13];
for elemento in &mut serie_fibonacci {
  *elemento *= 2
}
println!("{:?}", &serie_fibonacci); // [0, 2, 2, 4, 6, 10, 16, 26]
```

## Utilização de enums para guardar itens de tipos diferentes
Sobre enums, relembre que:
- um enum é efetivamente um tipo à parte definido pelo usuário
- um enum pode ter uma ou mais variantes
- as variantes podem conter dados

É possível fazer um vetor com n elementos de um mesmo tipo de enum:

```rust
enum CelulaTabela {
  Inteiro(i32),
  Texto(String),
  PontoFlutuante(f64),
}

let row = vec![
    CelulaTabela::Inteiro(3),
    CelulaTabela::Texto(String::from("blue")),
    CelulaTabela::PontoFlutuante(10.12),
];
```
### Notas
- O compilador precisa saber quais variantes do mesmo tipo poderão ser usadas como um elemento no vetor, de forma a poder alocar a memória na `heap` com o tamanho necessário para cada estrutura
- Esta abordagem tem a vantagem de deixar o código explícito quanto aos tipos que iremos alocar
- Quando utilizada com o bloco `match`, que é exaustivo, temos um tratamento robusto das possibilidades de fluxo, em tempo de compilação
- Nem sempre sabemos que tipos de item precisaremos alocar, para este caso especial, veremos os `trait objects` no Capítulo 17.
