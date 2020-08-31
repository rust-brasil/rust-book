# #03 - Conceitos comuns de programação

Como os conceitos comuns de programação funcionam no Rust, abordagem do ponto de vista do Rust.

## Convenções

Convenções são padrões que são comumente adotados pela comunidade ou grupo em comum.

Convenções visa a organização do seu código de uma forma geral, melhor entendimento a pessoas que não participaram ativamente de um projeto A, B ou C, convenção não tornará seu código rápido ou lento, errado ou certo.

## Palavras reservadas

São palavras que recomenda-se não utilizar em nomes de variáveis, funções e etc.  
Algumas palavras chaves tem funcionalidade e outras não tem e estão reservadas para uso futuro.  
Lista completa no Apêndice A do livro.

## 3.1 - Variáveis e Imutabilidade

### Imutabilidade

No Rust as variáveis são imutáveis por padrão. por questões de segurança e para prover facilidades quando estiver trabalhando com códigos em cenários em que a concorrência é algo a se considerar.

Imutáveis por padrão quer dizer que você pode optar por não ser imutável em algum momento;

`mut` par tornar uma variável mutável.

### Constantes

_As constantes não são apenas imutáveis ​​por padrão - elas são sempre imutáveis._

Ao declarar uma constante é obrigatório declarar o seu tipo e utilizar somente valores literais.

```rust
const NOME_COMPLETO: &str = "FULANO DE TAL BELTRANO JUNIOR";
```

### Convenção

nome de constantes e variáveis, utilizar "snake case"
nome de constantes todo em maiúsculo

Exemplo:

- `let ultimo_nome = "araujo";`
- `const VALOR_PI = 3.14;`

para valores numéricos, sublinhados podem ser utilizados nos valores para melhor legibilidade

Exemplo:

- `let mil = 1_000;`
- `let dez_mil = 10_000;`
- `let cem_mil = 100_000;`
- `let milhao = 1_000_000;`

C# .net core recentemente aderiu a esse formato também.

### Shadowing

Sombreamento é literalmente um facilitador para: reutilizar o nome da variável podendo ou não trocar o seu tipo.

_...você pode declarar uma nova variável com o mesmo nome que uma variável anterior..._  
_...o valor da segunda variável é o que aparece quando a variável é usada..._  
_...Podemos sombrear uma variável usando o mesmo nome da variável e repetindo o uso da palavra let..._

Exemplo:

```rust
fn main() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x);
}
```

### Exemplo com contexto

Digamos que nosso programa solicite a um usuário que: mostre quantos espaços ele deseja em algum texto
inserindo caracteres de espaço, mas realmente queremos armazenar essa entrada como um número

```rust
let spaces = "   ";
let spaces = spaces.len();
```

A primeira variável `spaces` é do tipo texto, a segunda variável `spaces` é do tipo numérico.

O sombreamento nos poupa de ter que inventar nomes diferentes, como `spaces_str` e `spaces_num`
em vez disso, podemos reutilizar o nome `spaces` que é mais simples.

No entanto, se tentarmos usar `mut` como mostrado aqui, obteremos um erro em tempo de compilação:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

_error[E0308]: mismatched types_  
_expected `&str`, found `usize`_

## 3.2 - Tipos de dados

Exitem dois subconjuntos de tipos de dados em Rust: escalar e composto.  
Rust é uma linguagem de tipo estático, ou seja, não existe tipagem dinâmica.  
Rust deve conhecer os tipos de todas as variáveis ​​em tempo de compilação.  
O compilador geralmente pode inferir que tipo queremos usar com base no valor e como o usamos.  
Nos casos em que muitos tipos são possíveis precisamos adicionar uma anotação de tipo, como esta:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Se não adicionarmos a anotação de tipo, o Rust exibirá o um erro informando que o compilador precisa de mais informações para saber qual tipo queremos usar.

### Tipos escalares

Tipo escalar, também chamado de literal, representa um valor único, informado pelo desenvolvedor, que não foi gerado através de algo computado.  
O Rust possui quatro tipos valores escalares principais:

- números inteiros
- números de ponto flutuante
- booleanos
- caracteres

#### Números Inteiros

Um número inteiro é um número sem a parte fracionária.  
Tipo padrão do Rust para números inteiros é o `i32`, esse tipo geralmente é o mais rápido, mesmo em sistemas de 64 bits.

| comprimento | Assinado | Não assinado |
|-------------|----------|--------------|
| 8 bits      | i8       | u8           |
| 16 bits     | i16      | u16          |
| 32 bits     | i32      | u32          |
| 64 bits     | i64      | u64          |
| 128 bits    | i128     | u128         |
| arch        | isize    | usize        |

Assinado e não assinado refere-se à possibilidade de o número ser negativo ou positivo.  
Os tipos `isize` e `usize` dependem do tipo de computador em que seu programa está sendo executado: 64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se você estiver em uma arquitetura de 32 bits.

A principal situação na qual você usaria `isize` ou `usize` é ao indexar algum tipo de coleção.

| Literais numéricos | Exemplo     |
|--------------------|-------------|
| Decimal            | 98_222      |
| Hex                | 0xff        |
| Octal              | 0o77        |
| Binário            | 0b1111_0000 |
| Byte ( u8apenas)   | b'A'        |

Comentar sobre estouro de valor, diferença entre compilar como debug ou release.

### Os tipos de ponto flutuante

O tipo "ponto flutuante" em rust nada mais é que números com casas decimais, exemplo de valores:

- 0.12
- 1.45
- 1_001.951
- 1_753_183.927

#### Declaração

`let pf1 = 0.12;`

O tipo padrão é `f64`, porque nos processadores modernos ele tem quase a mesma velocidade do `f32`, mas oferece mais precisão. Então a declaração acima é o mesmo que:

`let pf1: f64 = 0.12;`

O Rust tem dois tipos primitivos para números de ponto flutuante. Esses tipos são a implementação dos formatos `binary32` e `binary64` da especificação `IEEE-754 (IEC 60559)` (em detalhes abaixo): o tipo `f32` de 32 bits (precisão única) e o tipo `f64` de 64 bits ([precisão dupla](https://pt.wikipedia.org/wiki/Dupla_precis%C3%A3o_no_formato_de_ponto_flutuante)), respectivamente.

Uma das primeiras linguagens de programação a fornece ponto flutuante com precisão única e dupla foi o Fortran (1957, 63 anos atrás).

A precisão dupla fornece uma relativa precisão de cerca de 16 dígitos decimais e intervalo de 10 elevado a -308 a 10 elevado a +308 (308 zeros pra lá (positivo) e pra cá (negativo)).

Precisão dupla é conhecida como `double` na linguagem C, C++ e Java.
Precisão única é conhecida como `float` na linguagem C, C++ e Java.

Processadores modernos processam números decimais de 64 bits com a mesma velocidade de números decimais de 32 bits.

O termo precisão dupla não é referente a ter o dobro de precisão, mas referente a usar o dobro de bits de um ponto flutuante regular.

Precisão simples ou dupla, não esta ligada a arquitetura do processador, ou seja, um processador com arquitetura de 32 bits consegue utilizar e processar números de precisa dupla.

#### Padrão IEEE-754 (IEC 60559)
Documento que define uma implementação padronizada para a representação de números de ponto flutuante em computadores.
Antes desse padrão, cada linguagem e hardware usava uma representação própria, o que resultava em problemas de interoperabilidade.

Precisão Única

- O primeiro bit é o bit de sinal (informa se o número é negativo ou positivo), S,
- os próximos oito bits são os bits do expoente , 'E' e
- os 23 bits finais são a fração 'F':
- aproximadamente 7 casas decimais de precisão

```
S EEEEEEEE FFFFFFFFFFFFFFFFFFFFFFF
0 1      8 9                    31
```

Precisão Dupla

- O primeiro bit é o bit de sinal (informa se o número é negativo ou positivo), S,
- os próximos onze bits são os expoentes , 'E' e
- os 52 bits finais são a fração 'F':
- aproximadamente 15 casas decimais de precisão

```
S EEEEEEEEEEE FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
0 1        11 12                                                63
```

Ou seja, precisão dupla utilizará mais memoria RAM que precisão simples, memoria RAM hoje em dia não é mais um problema tão grande assim, mas dependendo do projeto/cenário (IoT por exemplo) saber a diferença entre esses dois tipos pode fazer diferença.

#### Informações sobre o tema

Indicação de vídeos que aborda o assunto e sobre erros de ponto flutuante (imprecisão):

- [em português](https://www.youtube.com/watch?v=dkaB4kGtb7s)
- [em inglês](https://www.youtube.com/watch?v=ILKjj30pb9E)

Site:

- [O que todo programador tem que saber sobre pontos flutuantes [em inglês]](https://floating-point-gui.de/) 


### Operações numéricas

O Rust suporta as operações matemáticas básicas que você esperaria de todos os tipos de números: adição, subtração, multiplicação, divisão e resto. O código a seguir mostra como você usaria cada um em uma instrução `let`:

```rust
fn main() {
    // addition - soma
    let sum = 5 + 10;

    // subtraction - substração
    let difference = 95.5 - 4.3;

    // multiplication - multiplicação
    let product = 4 * 30;

    // division - divisão
    let quotient = 56.7 / 32.2;

    // remainder - resto
    let remainder = 43 % 5;
}
```

Cada expressão nessas instruções usa um operador matemático e avalia como um único valor, que é então vinculado a uma variável. 

O Apêndice B do livro contém uma lista de todos os operadores que o Rust fornece.

Não é possível misturar inteiros com números de ponto flutuante, você precisa convertê-los antes:

```rust
    // conversão de inteiro para ponto flutuante com safe casting
    // https://doc.rust-lang.org/nomicon/casts.html
    let x = 14;
    let y = 2.8;
    let z = (x as f64)/y; // 5
    // conversão de ponto-flutuante para inteiro
    // Rust simplesmente descarta a parte fracionária (.8)
    let z = x+(y as i32); // 16
```

A filosofia por trás desta exigência (de trabalhar com variáveis do mesmo tipo em expressões numéricas) é que a autoconversão é algo perigoso: o programador pode se enganar com relação às expectativas.

### O tipo booleano

### O Tipo Caractere (char)

- Tipo alfabético mais primitivo
- Literais são definidos com um apóstrofo (`'`), ao contrário dos literais de `string`, que são definidos com aspas (`"`):

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let str = "Z";
    let heart = '❤';
    let heart_eyed_cat = '😻';
    let sparkling_heart = '\u{1F496}'; // 💖 definição via codepoint unicode em hexadecimal
}
```
- O tipo `char` tem 4 bytes (32 bits) de tamanho e representa um valor escalar Unicode (ou seja, um `codepoint` entre 0 a D7FF e entre E000 a 10FFFF, isto é, todos os valores unicode exceto os pares substitutos, `surrogate pairs`)
  - Pode ser usado para representar muito mais do que os caracteres ascii
  - Valor escalar unicode: valores de 21 bits que são a unidade básica do Unicode
    - Conceito de caractere não está bem definido no padrão Unicode
      - Problema: o que chamamos de caractere pode ser constituído por mais de valor escalar Unicode, por exemplo:
        - https://emojipedia.org/emoji/%E2%9D%A4/ é a soma de
          - https://charbase.com/2764-unicode-heavy-black-heart com
          - https://charbase.com/fe0f-unicode-variation-selector
        - um exemplo de par substituto
        - ou seja, são necessários dois chars para representá-lo:

        ```rust
        fn main() {
            let a: char = '🖤';
            println!("{}", a.to_string());

            let b: char = '❤️'; //ERRO, char só pode representar um codepoint
            println!("{}", b.to_string());
        }
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a6fd5757faf214087f7d53eddc5f1d1a
      ```
- Referências:
  - https://cheats.rs/#textual-types-ref
  - https://andybargh.com/unicode/
  - https://www.christianfscott.com/rust-chars-vs-go-runes/

### Tipos compostos

Os tipos compostos podem agrupar vários valores em um tipo. Rust tem dois tipos de compostos primitivos: tuplas e matrizes.

#### O tipo de tupla

Uma sequência heterogênea **FINITA**

heterogênea: tipos diferentes

As tuplas são frequentemente usadas como um tipo de retorno quando você deseja retornar mais de um valor:

As tuplas são finitas. Em outras palavras, uma tupla tem um length/comprimento. Aqui está uma tupla de comprimento 3:

`( "olá" , 5 , 'c' );`

'Length'/'Comprimento' também é às vezes chamado de 'arity'/'aridade' aqui; cada tupla de comprimento diferente é um tipo diferente e distinto.

Na matemática a aridade de uma função ou operação é o número de argumentos ou operandos tomados. A aridade de uma relação é o número n de elementos que compõem as n-uplas ordenadas pertencentes à relação. [Wikipédia](https://pt.wikipedia.org/wiki/Aridade)

As tuplas são heterogêneas. Isso significa que cada elemento da tupla pode ter um tipo diferente. 

Tuplas são uma sequência. Isso significa que eles podem ser acessados ​​por posição; isso é chamado de 'indexação de tupla'.

Uma tupla é uma maneira geral de agrupar vários valores com uma variedade de tipos em um tipo composto. As tuplas têm um comprimento fixo: uma vez declaradas, elas não podem aumentar ou diminuir de tamanho.

Criamos uma tupla escrevendo uma lista de valores separados por vírgulas entre parênteses. Cada posição na tupla tem um tipo e os tipos dos diferentes valores na tupla não precisam ser os mesmos. Adicionamos anotações de tipo opcionais neste exemplo:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

A variável tup se liga a toda a tupla, porque uma tupla é considerada um único elemento composto. Para obter os valores individuais de uma tupla, podemos usar a correspondência de padrões para desestruturar um valor de tupla, como este:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Este programa primeiro cria uma tupla e a vincula à variável tup. Em seguida, usa um padrão com let para pegar tup e transformá-lo em três variáveis separadas, x, y e z. Isso é chamado de desestruturação, porque divide a tupla única em três partes. Finalmente, o programa imprime o valor de y, que é 6,4.

Além da desestruturação por meio da correspondência de padrões, podemos acessar um elemento de tupla diretamente usando um ponto (.) Seguido pelo índice do valor que desejamos acessar. Por exemplo:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Este programa cria uma tupla, x, e então cria novas variáveis para cada elemento usando seus respectivos índices. Como acontece com a maioria das linguagens de programação, o primeiro índice em uma tupla é 0.

https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type  
https://doc.rust-lang.org/std/primitive.tuple.html  
[código fonte - exemplos](/capitulo_03/rust-tuple/src/main.rs)

#### [O tipo de matriz](./Arrays.ipynb)
