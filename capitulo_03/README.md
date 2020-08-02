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

### O tipos ponto flutuante

O tipo "ponto flutuante" em rust nada mais é que números com casas decimais, exemplo de valores:

- 0.12
- 1.45
- 1_001.951
- 1_753_183.927

#### Declaração

`let pf1 = 0.12;`

O tipo padrão é `f64`, então a declaração acima é o mesmo que:

`let pf1: f64 = 0.12;`

O tipo `f32` 32 bits tem precisão única, `f64` 64 bits tem [precisão dupla](https://pt.wikipedia.org/wiki/Dupla_precis%C3%A3o_no_formato_de_ponto_flutuante).

Uma das primeiras linguagens de programação a fornece ponto flutuante com precisão única e dupla foi o Fortran (1957, 63 anos atrás).

A precisão dupla fornece uma relativa precisão de cerca de 16 dígitos decimais e intervalo de 10 elevado a -308 a 10 elevado a +308 (308 zeros pra lá (positivo) e pra cá (negativo)).

Precisão dupla é conhecida como `double` na linguagem C, C++ e Java.

Processadores modernos processam números decimais de 64 bits com a mesma velocidade de números decimais de 32 bits.

O termo precisão dupla não é referente a ter o dobro de precisão, mas referente a usar o dobro de bits de um ponto flutuante regular.

Precisão simples ou dupla, não esta ligada a arquitetura do processador, ou seja, um processador com arquitetura de 32 bits consegue utilizar e processar números de precisa dupla.

#### Padrao IEEE

Precisão Única

- O primeiro bit é o bit de sinal (informa se o número é negativo ou positivo), S,
- os próximos oito bits são os bits do expoente , 'E' e
- os 23 bits finais são a fração 'F':

```
S EEEEEEEE FFFFFFFFFFFFFFFFFFFFFFF
0 1      8 9                    31
```

Precisão Dupla

- O primeiro bit é o bit de sinal (informa se o número é negativo ou positivo), S,
- os próximos onze bits são os expoentes , 'E' e
- os 52 bits finais são a fração 'F':

```
S EEEEEEEEEEE FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
0 1        11 12                                                63
```

Ou seja, precisão dupla utilizará mais memoria RAM que precisão simples, memoria RAM hoje em dia não é mais um problema tão grande assim, mas dependendo do projeto/cenário (IoT por exemplo) saber a diferença entre esses dois tipos pode fazer diferença.

### Operações numéricas

### O tipo booleano

### O Tipo Caracter

### Tipos compostos

#### O tipo de tupla

#### O tipo de matriz
