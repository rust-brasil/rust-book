use std::io;
use rand::Rng; // crate externo, fora da biblioteca padrão
use std::cmp::Ordering; // incorpore Ordering ao escopo, pois utilizaremos suas enumerações abaixo

fn main() {
    println!("Adivinhe o número!"); //impressão de uma mensagem na tela

    /* 1. Defina a variável secret_number, o número secreto que o usuário deverá adivinhar.
     * 2. A função associada thread_rng() inicia um gerador de números aleatórios para esta thread.
     * 3. A função .gen_range() gera um número entre 1 e 100 (o limite na parte de cima é ignorado). */
    let secret_number = rand::thread_rng()
        .gen_range(1, 101); // entre 1 e 100 efetivamente

    loop {
        /* Define a variável guess, que armazenará o palpite do usuário.
         * A variável foi definida como mutável com a palavra-chave mut,
         * pois o seu conteúdo será alterado depois.
         * O valor atribuído à variável é uma instância de String
         * (uma string vazia).*/
        let mut guess = String::new(); //variável declarada como mutável

        println!("Por-favor, digite o seu palpite.");

        /* Leia uma linha digitada pelo usuário e
         * coloque o valor digitado na variável guess,
         * passada como referência (&) para a função read_line(). */
        io::stdin() // utilizando a entrada do console
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        /* Redeclare a variável guess (shadowing).
         * Se a conversão para u32 acontecer, defina como
         * valor da variável o número convertido.
         * Se a conversão falhar, reinicie o loop.
         * Isso efetivamente ignorará valores digitados pelo
         * usuário que não possam ser convertidos para um
         * número u32. */
        let guess: u32 = match guess // (3) avalie a enumeração retornada por parse, se...
            .trim() // (1) apague os espaços no começo e no final da string
            .parse() // (2) converta a string para u32
            {
               Ok(num) => num, // (4a) ... a conversão for um sucesso, retorne o valor convertido
               Err(_) => continue, //(4b) ... houver erro, reinicie o loop
            };

        /* Imprima o número digitado pelo usuário
         * interpolando o valor da variável guess
         * na mensagem através do placeholder ({}). */
        println!("Você supôs: {}", guess);

        /* Compare as duas variáveis guess e secret_number e
         * indique se o usuário acertou o palpite ou se o palpite
         * está abaixo/acima do número gerado (secret_number).
         * O loop (e o programa) só terminará depois que o usuário
         * acertar o número. */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break; // saia do loop e efetivamente termine o programa
            }
        }
    }
}