# Programação funcional em Rust: iteradores e closures

## Introdução

- Uma das grandes influências no design da linguagem Rust foi a programação funcional
- O que é programação funcional?

```
Programação funcional é o processo de construir software através da composição de funções puras, 
evitando-se o compartilhamento do estado, a mutação dos dados e efeitos colaterais.
A Programação funcional é declarativa ao invés de imperativa e o o estado da aplicação flui
através de funções puras. [1]
```

- O capítulo não se propõe a definir o que é programação funcional, mas sim discutir algumas funcionalidades de Rust que se assemelham a outras de linguagens funcionais:
  - Closures
  - Iteradores

- Depois de explicar isso, iremos:
  - Melhorar o projeto do capítulo anterior (projeto em linha de comando)
  - Avaliar a performance dessas funcionalidades

- Além disso, escrever Closures e Iterators é uma parte fundamental de produzir código em Rust idiomático


## Closures
- Closures são funções anônimas que podem capturar valores do escopo na qual foram definidas
- Elas podem ser salvas em variáveis e passadas para outras funções para que estas a utilizem.
- Funções que recebem outras funções como parâmetro são chamadas de funções de alta ordem (High Order Functions)
- O principal objetivo das closures é ter a habilidade de definir o código num lugar e executá-lo em outro diferente. Isso permite melhor reutilização de código e customização de comportamentos.
- Para ilustrar esses pontos, trabalharemos num projeto

### O projeto-exemplo

- "Considere esta situação hipotética: trabalhamos em uma startup que está fazendo um aplicativo para gerar planos de exercícios personalizados. O back-end é escrito em Rust, e o algoritmo que gera o plano de treino leva em consideração muitos fatores, como a idade do usuário do aplicativo, índice de massa corporal, preferências de exercício, treinos recentes e um número de intensidade especificado por eles. O algoritmo real usado não é importante neste exemplo; o importante é que esse cálculo leve alguns segundos. Queremos chamar este algoritmo apenas quando precisamos e apenas chamá-lo uma vez para não fazer o usuário esperar mais do que o necessário."

(código)

- Nós vamos simular a chamada a esta algoritmo hipotético com uma função `simulated_expensive_calculation`. Esta função simplesmente imprime a mensagem "Calculando lentamente..." espera dois segundos e depois retorna o valor de intensidade que recebeu como parâmetro:


```rust
// Filename: src/main.rs
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculando lentamente...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

- Depois, na função `main`, nós vamos o código que o app chamará quando o usuário requisitar um plano de exercício. Para deixar mais didático, nós vamos fixar os valores de entrada e imprimir os valores de saída, mas para fins de documentação, segue a descrição desses valores de entrada e saída:

  - Um número de intensidade vindo do usuário, o que indica se ele quer um plano de exercício mais leve ou mais intenso.
  - Um número aleatório que é usado para dar mais variedade ao plano de exercício
  - A saída será um plano de exercício

``` rust
//Filename: src/main.rs
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

- Vamos agora para a lógica do negócio do App, a função `generate_workout` em sua primeira versão:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Hoje, faça {} flexões!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Depois, faça {} agachamentos!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Hoje descanse um pouco! Lembre-se de ficar hidratado!");
        } else {
            println!(
                "Hoje, corra por {} minutos!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

- Observe que o código acima tem múltiplas chamadas para a função de cálculo (`simulated_expensive_calculation`). O primeiro bloco `if` tem duas chamadas, o bloco `else` mais interno chama ela uma vez.

- Nosso objetivo é refatorar este código para que a função `simulated_expensive_calculation` seja chamada apenas uma vez. Seria interessante fazer essa alteração no lugar onde esta função é chamada duas vezes sem adicionar mais chamadas.

#### Refatorando usando funções

- Poderíamos substituir a chamada dupla à função `simulated_expensive_calculation` com o cálculo de seu valor anteriormente numa variável (`expensive_result`):

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Hoje, faça {} flexões!", expensive_result);
        println!("Depois, faça {} agachamentos!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Descanse hoje! Lembre-se de manter-se hidratado.");
        } else {
            println!("Hoje, corra por {} minutos!", expensive_result);
        }
    }
}
```

- O problema dessa abordagem é que estamos, em todos os casos, executando `simulated_expensive_calculation`, quando ela é necessária em apenas dois dos três casos.
- O que nós queremos fazer é definir o código num local e executá-lo somente quando precisarmos, onde precisarmos.

#### Refatorando com closures
- Em vez de sempre chamar a função `simulated_expensive_calculation` antes dos blocos `if`, nós podemos definir uma closure e guardar essa closure numa variável ao invés de guardar o resultado da invocação da função. Nós vamos mover o corpo da função `simulated_expensive_calculation` para dentro de uma closure:

```rust
    let expensive_closure = |num| {
        println!("Calculando lentamente...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

##### Explicando a sintaxe da closure:
1. Associamos a closure a variável `expensive_closure` tal como fazemos com valores. A closure é tudo o que vem depois do sinal de igual (`=`)
2. Para definir a closure, usamos um par de pipes (`||`), dentro dos quais colocamos os parâmetros da closure. Neste caso, a nossa closure tem um parâmetro `num`, se nós tivéssemos mais de um parâmetro, separaríamos eles por vírgulas.
3. Depois dos parâmetros, nós definimos o corpo da closure entre chaves (`{}`). Se a nossa closure consistisse numa única expressão, essas chaves seriam opcionais
4. No final da closure, depois do fechamento das chaves, colocamos um ponto-e-vírgula (`;`) para completar a declaração `let`.
5. O valor retornado pela closure, quando invocada, será a sua última linha sem ponto-e-vírgula, um caso de retorno implícito como vimos nas funções
6. Observe que a declaração let inteira contem a **definição** de uma closure não o resultado de sua **invocação**.


- Com a closure definida, podemos agora alterar os códigos nos blocos `if` para invocá-la ao invés da função. Nós invocamos uma closure da mesma forma que invocamos uma função:

##### Inferência e anotações de tipos em closures
- Closures não exigem que você anote os tipos dos parâmetros como as funções - tipos são obrigatórios em funções porque elas precisam ser genéricas e podem ser chamadas em qualquer contexto, esses tipos fazem parte de um contrato com os usuários da função.
- Já as closures não são expostas como as funções, geralmente são usadas localmente e associadas a uma variável também local. As closures geralmente são pequenas e tem relevância apenas para uma pequena parte bem definida de nosso código ao invés de um contexto genérico.
- Com esse contexto bem limitado, o compilador do Rust consegue inferir com segurança os tipos dos parâmetros e o retorno de uma closure da mesma forma como consegue inferir os tipos das variáveis.
- De qualquer forma, se quisermos, podemos anotar esses tipos, por exemplo:
```rust
let expensive_closure = |num: u32| -> u32 {
    println!("Calculando lentamente...");
    thread::sleep(Duration::from_secs(2));
    num
};
```
- Com tipos, a sintaxe de uma closure se aproxima da sintaxe de uma função:
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
- Observe que invocar as versões `add_one_v3` e `add_one_v4` pelo menos uma vez é obrigatório para compilar o código, pois somente assim o compilador poderá inferir os tipos.
- Para cada closure, apenas uma versão concreta será inferida para seus tipos. O código abaixo não compila:
```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
- A primeira vez que invocamos `example_closure`, o compilador anota o tipo de `x` e do retorno como sendo `String`. A segunda invocação não altera essa anotação, o que a torna incompatível com o tipo `i32` passado pela segunda vez.

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculando lentamente...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Hoje, faça {} flexões!", expensive_closure(intensity));
        println!("Depois, faça {} agachamentos!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Descanse hoje! Lembre-se de manter-se hidratado.");
        } else {
            println!(
                "Hoje, corra por {} minutos!",
                expensive_closure(intensity)
            );
        }
    }
}
```
- Neste ponto, a função demorada que calcula está definida em apenas um local e estamos chamando ela apenas quando precisamos de seu resultado.
- Contudo, ainda temos o problema da chamada dupla no primeiro bloco `if`. Poderíamos resolver este problema com uma variável local para este bloco para guardar o resultado da invocação da closure, mas o uso de closures nos permite outra solução, que veremos em seguida.
- Mas antes, vamos falar sobre anotações de tipos em closures

#### Memoizacão com uma estrutura
- Voltando ao nosso projeto, nós poderíamos o resolver o problema com o uso de uma variável, como dito acima. Porém isso geraria um código repetido, não é uma solução elegante.
- O que nós iremos fazer é criar uma estrutura que guarde a nossa closure e o resultado de sua invocação. O resultado será calculado apenas uma vez, pois guardaremos o valor de sua execução. Essa técnica é conhecida como memoização ou avaliação preguiçosa (*lazy evaluation*).
- Para fazer essa estrutura, precisaremos especificar o tipo da closure, pois os tipos dos campos de uma struct precisam ser explicitamente definidos.
- Cada closure tem seu próprio tipo anônimo, ou seja, mesmo que duas closures tenham a mesma assinatura, seus tipos são diferentes.
- Dessa forma, para definir a nossa estrutura, nós teremos que lançar mão de genéricos e trait bounds (tais como vistos no capítulo 10).
- A biblioteca-padrão do Rust provê três tipos de traits para funções: `Fn`, `FnMut` ou `FnOnce`. Nós vamos discutir as diferenças entre elas depois.
- Dessa forma, a nossa estrutura será definida conforme abaixo:

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

- Definimos o campo `calculation` com o tipo genérico `T`, que está limitado pela trait `Fn`. Isso indica que qualquer closure que quisermos guardar neste campo precisa ter uma assinatura que aceite um parâmetro `u32` e que retorne um valor `u32`.
- O campo `value` é do tipo `Option<u32>`, o que significa que pode ou não ter um valor calculado, ou seja, `Some(u32)` ou `None`. Quando iniciarmos a estrutura, ela não terá valor calculado, mas quando procurarmos calcular este valor, este campo guardará a variante `Some` com o valor.
- Implementaremos o cache da seguinte forma: se o valor já estiver calculado, retornarmos ele imediatamente; caso contrário, faremos o cálculo. Eis a implementação:

```rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // Se já tivermos calculado o valor, retorne-o
            Some(v) => v,
            // Caso contrário, calcule-o utilizando a closure configurada
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```
- Dessa maneira, encapsulamos a administração do valor dentro de `Cacher` (os campos são privados).
- O código abaixo mostra como podemos usar esta estrutura em nossa função `generate_workout`:
```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculando lentamente...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Hoje, faça {} flexões!", expensive_result.value(intensity));
        println!("Depois, faça {} agachamentos!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Descanse hoje! Lembre-se de manter-se hidratado.");
        } else {
            println!(
                "Hoje, corra por {} minutos!",
                expensive_result.value(intensity)
            );
        }
    }
}
```
- Observe que podemos chamar o método `value` da estrutura quantas vezes quisermos (ou nem chámá-lo), mas se chamarmos o cálculo só acontecerá no máximo uma vez.
##### Limitações da estrutura de memoização
- Sempre o mesmo valor: modificar a estrutura para usar um hashmap: as chaves são os valores passados a value e os valores são os calculados. O seguinte teste deve passar:

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.get_value(1);
    let v2 = c.get_value(2);
    let v3 = c.get_value(3);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 3);
}
```

- Só aceita closures com a assinatura `Fn(u32) -> u32`: introdução de mais tipos genéricos para parametrizar mais o trait bound. O seguinte teste deve passar:

```rust
#[test]
fn call_with_different_closures() {
    let mut c1 = Cacher::new(|x| x);
    let mut c2 = Cacher::new(|x: &str| x.len());
    let v1 = c1.get_value(1);
    let v2 = c2.get_value("abacate");

    assert_eq!(v1, 1);
    assert_eq!(v2, 7);
}
```

- Solução:
```rust
struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calc_fn: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calc_fn: T) -> Self {
        Cacher {
            calc_fn,
            value: HashMap::new(),
        }
    }
    fn get_value(&mut self, n: U) -> V {
        match self.value.get(&n) {
            Some(value) => *value,
            None => {
                let result = (self.calc_fn)(n);
                self.value.insert(n, result);
                result
            }
        }
    }
}
```

#### Capturando o ambiente com closures
- Diferente de funções, closures podem capturar o ambiente e ter acesso às variáveis do contexto em que foram definidas

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```
- "Quando uma closure captura um valor de seu ambiente, ela usa a memória para armazenar os valores para uso no corpo da closure. Esse uso de memória é uma sobrecarga que não queremos pagar em casos mais comuns em que queremos executar código que não captura seu ambiente. Como as funções nunca têm permissão para capturar seu ambiente, definir e usar funções nunca incorrerá nessa sobrecarga."
- Existem três formas de captura de valores para uma closure: pegando a propriedade, pegando emprestado com referência mutável ou pegando emprestado como referência imutável. Essas três formas correspondem às três traits `FnOnce`, `FnMut` e `Fn`.
- Quando você declara uma closure, o compilador automaticamente escolhe qual trait usar de acordo com o uso que a sua closure faz das variáveis.
- Se você quiser forçar uma closure a pegar a propriedade das variáveis de seu contexto, você pode usar a palavra-chave `move`:

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```
- Quando tiver que definir um trait bound para uma closure, na maior parte das vezes você pode começar com a trait `Fn`; o compilador irá te avisar se você precisar das outras traits com base no que acontecer com as variáveis no corpo da closure.

## Iteradores
- O padrão de código iterador permite que você execute algumas tarefas em uma sequência de itens por vez;
- Um iterador é responsável pela lógica de iterar sobre cada item e determinar quando a sequência termina;
- Ao usar iteradores já prontos do Rust, você não precisa reimplementar essa lógica sozinho;
- Iteradores no rust são preguiçosos: eles não têm efeito até que você chame algum método que os consuma;
- Exemplo:

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Consegui: {}", val);
    }
```

### Adaptadores consumidores
- Adaptadores consumidores: métodos que consomem iteradores
- Exemplo com sum, que toma posse do iterador e o consome:
```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

### Adaptadores de iterador
- Adaptadores de iterador: métodos que produzem outros iteradores, portanto permitem transformar um iterador num de tipo diferente
- Você pode interligar múltiplas chamadas a esses métodos para fazer transformações complexas com dados
- Exemplo com map:

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter()
        .map(|x| x + 1)
        .filter(|x| x % 2 == 0)
        .collect();
```
- O código acima retorna um erro, pois iteradores são preguiçosos, ou seja, precisamos consumi-los para que tenham efeito. Podemos fazer isso com o método `collect`, por exemplo, que transforma um iterador numa coleção:

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```
- Este é um bom exemplo de customização de comportamento com o uso de closures.

#### Usando closures que capturam seu ambiente (exemplo com filter)
```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 42,
                style: String::from("tênis"),
            },
            Shoe {
                size: 36,
                style: String::from("sandália"),
            },
            Shoe {
                size: 42,
                style: String::from("bota"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 42);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 42,
                    style: String::from("tênis")
                },
                Shoe {
                    size: 42,
                    style: String::from("bota")
                },
            ]
        );
    }
}

fn main() {}
```
### A trait `Iterator` e o método `next`
- Todos os iteradores implementam a trait `Iterator`:
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
- Temos aqui um tipo associado, que estudaremos em mais detalhes no capítulo 19. Por agora, tudo que você precisa saber é que ao definirmos um iterador, precisamos especificar o tipo de seu item, que será retornado, encapsulado num `Option`, pela função `next`.
- Definir esse tipo e a função `next` é tudo que precisamos fazer para tirar proveito das diversas funções de iteradores que Rust tem por padrão.

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

- Para usar `next` o iterador precisa ser declarado numa variável mutável, pois `next` altera o estado interno dele
- Dizemos que o código acima *consome* o iterador.
- Os valores que obtemos de `next` são por padrão referências imutáveis para os valores no iterador. Existem três formas de criar um iterador de uma sequência: TODO: exemplos?
  - `iter` produz um iterador sobre referências imutáveis
  - `into_iter` produz um iterador que se aproprie de v1 e retorne valores com propriedade
  - `iter_mut` produz um iterador com referências mutáveis


### Criando nossos próprios iteradores com a trait Iterator
- Você pode criar iteradores para os tipos embutidos de Rust, como, por-exemplo, um hashmap
- Você também pode criar iteradores para os seus tipos customizados ao implementar a trait `Iterator`
- (exemplo, iterador que conta a até 5)

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {}
```
- Ao implementar o método `next`, agora nós podemos usar todos os outros métodos para iteradores que o rust nos fornece, por exemplo:

```rust
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
```

## Melhorando nosso programa de linha de comando
- No capítulo anterior, nós construímos um clone do utilitário `grep` em linha de comando
- Com o conhecimento adquirido sobre closures e iteradores, nós vamos fazer algumas refatorações

### Removendo a necessidade do `clone()`
```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```
- Precisamos usar o `clone` aqui porque temos um string slice com elementos do parâmetro args, mas a função `new` não é dona de `args`. Para que `Config` pudesse ser a dona desses valores, tivemos que cloná-los.
- Alterações:
  - Nós vamos mudar a função `new` para que ela possa tomar posse do iterador no argumento `args` ao invés de pegar emprestado um slice.
  - Usaremos a funcionalidade de iterador em vez do código que verifica o tamanho da fatia e os índices em posições específicas.
  - Com a posse do iterador, nós poderemos mover os valores que nos interessam para dentro da struct `Config`, sem que seja necessário chamar `clone` e com isso fazer outra alocação de memória

#### Usando o iterador diretamente
- Atualizando a invocação de Config::new
De:
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```
para:
```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```
- Atualizando a assinatura de Config::new
```rust
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // --snip--
```
#### Usando os métodos da trait `Iterator` ao invés de indexação
```rust
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); //ignoreamos o primeiro valor

        // Pegamos os próximos dois, que devem ser query e filename
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

### Deixando o código limpo com Adaptadores Iteradores
- Podemos adotar um estilo de programação funcional em `search`
- O estilo de programação funcional prefere minimizar o uso de estados mutáveis para deixar o código mais claro
- Remover a etapa com o estado mutável pode nos permitir executá-lo em paralelo, pois não teremos que gerenciar o acesso ao vetor `results`:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```
- O código usa `filter` para deixar apenas as linhas que nos interessam.

## Performance: Loops vs Iteradores
- Iteradores, embora sejam uma abstração de alto nível, são compilados quase no mesmo código como se você mesmo tivesse escrito o código de baixo nível.
- Iteradores são uma das abstrações de custo zero de Rust, o que significa que o uso da abstração não impõe sobrecarga de tempo de execução adicional.
- Rust sabe que existem x iterações, então ele “desenrola” o loop. O desenrolar (unrolling) é uma otimização que remove a sobrecarga do código de controle do loop e, em vez disso, gera código repetitivo para cada iteração do loop. Todos os coeficientes são armazenados em registradores, o que significa que o acesso aos valores é muito rápido.

## Próximo capítulo
- No próximo capítulo falaremos sobre alguns funcionalidades adicionais da ferramenta `cargo` para o compartilhamento de nossos projetos com a comunidade.


## Referências

O que é Programação funcional?
https://medium.com/javascript-scene/master-the-javascript-interview-what-is-functional-programming-7f218c68b3a0#:~:text=Functional%20programming%20(often%20abbreviated%20FP,state%20flows%20through%20pure%20functions.

Rust é uma linguagem funcional?
https://www.fpcomplete.com/blog/2018/10/is-rust-functional/

Closure vs lambdas
https://stackoverflow.com/questions/220658/what-is-the-difference-between-a-closure-and-a-lambda

Closure Expressions
https://doc.rust-lang.org/reference/expressions/closure-expr.html

Rust - Closures and lambda expressions
https://sodocumentation.net/rust/topic/1815/closures-and-lambda-expressions

Avaliação preguiçosa
https://pt.wikipedia.org/wiki/Avalia%C3%A7%C3%A3o_pregui%C3%A7osa
