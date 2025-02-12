# Repositório do curso Programação Rust Completo : do Zero ao Pleno! (2025)

### Configurando o ambiente:

- Iniciando o gerenciador de pacotes: `$ cargo init`
- Rodando o projeto: `$ cargo run`

### Conteúdo aprendido no curso:

- Módulo 01: Configuração Inicial até Desafios de Controle de Fluxo e Funções

### Anotações:

#### Variáveis:

- Variáveis em Rust são imutáveis;
- Para tornar uma variável modificável é necessário adicionar a palavra "mut" após o "let", assim:

```rust
    let mut name: &str = "Henrique";
    name = "João";
```

- As chaves {} dentro do print equivale à variável:

```rust
    let mut name: &str = "Henrique";
    name = "João";
    println!("Hello {}!", name);

    Saída: "Hello João!"
```

#### Tipos de dados:

- Para modificar o tipo de inteiro que queremos utilizar, temos que especificar na declaração da variável adicionando o i (signed (assinado)):

```rust
    //let x: i32 = 23;
    let x: i64 = 23;
```

- Inteiro padrão: i32 (32bits)
- Tipos de inteiros são relacionados com a capacidade de bits de armazenamento. Ex:

  - i8 (8bits)
  - i16 (16bits)
  - i32 (32bits)
  - i64 (64bits)
  - i128 (128bits)

- Para declarar que uma variável não pode receber valor negativo, é utilizado o "u" (unsigned (sem sinal)):

```rust
    let x: u64 = 23;
```

- Float padrão: f64 (64 bits)
- Tipos de valores de ponto flutuante são declarados com "f" (float (flutuante)):

```rust
    // let f: f32 = 6.7;
    let f: f64 = 6.7;
```

- Tipos de valores booleanos são declarados com "bool" (boolean (booleano)):

```rust
    // let b: bool = false;
    let b: bool = true;
```

#### Controle de fluxo:

- Uma expressão if permite ramificar o código dependendo das condições.
- É fornecida uma condição e então declarado, "Se esta condição for atendida, execute este bloco de código. Se a condição não for atendida, não execute este bloco de código."

```rust
    let number1 = 24;
    let number2 = 42;
    if number1 > number2{
        println!("{} > {}", number1, number2)
    }else {
        println!("{} <= {}", number1, number2)
    }
```

#### Entrada de dados:

- Para utilizar a entrada de dados é necessário importar a biblioteca io:

```rust
    use std::io;
```

- use std::io; Importa o módulo de entrada e saída (std::io), necessário para capturar a entrada do usuário via teclado.

- Para criar uma função é necessário utilizar a seguinte sintaxe:

```rust
    fn convert_to_int(data_input: & String) -> i32{
        let x = data_input.trim().parse::<i32>().unwrap();
        x
    }
```

- fn é o mesmo que a palavra reservada function de outras linguagens
- convert_to_int é o nome da função
- data_input é a variável que recebe os dados de entrada
- & String define que o tipo de dados recebidos serão string
- -> i32 indica que o resultado esperado é um inteiro de 32 bits
- O método .trim() remove espaços em branco extras no começo e no final da string
- O método .parse::<i32>() tenta converter a string em um número inteiro (i32)
- O método .unwrap() força a extração do valor convertido
- Em Rust, a última expressão de uma função sem ; (ponto e vírgula) é automaticamente retornada. Ou seja, x será retornado sem precisar de return x;

- Para entrar com os dados é feita da seguinte forma:

```rust
    fn main(){
        let mut number1 = String::new();
        io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
        let mut number2 = String::new();
        io::stdin().read_line(&mut number2).expect("Erro ao ler number2");
    }
```

- let mut number1 = String::new(); Declara uma variável mutável (mut) chamada number1 e a inicializa como uma string vazia (String::new())
- É necessário ser mutável porque o valor será modificado quando o usuário digitar algo.
- io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
  - io::stdin() acessa a entrada padrão do sistema (teclado).
  - .read_line(&mut number1) lê a entrada do usuário e armazena na variável number1.
  - Como read_line retorna um Result, expect("Erro ao ler number1") é usado para lidar com erros.
  - Se a leitura falhar, o programa será encerrado com a mensagem de erro.

---

#### Revisando todo o código

```rust
use std::io; // Essa linha importa a biblioteca "io" do Rust, que fornece funções para entrada e saída de dados

// Essa linha define uma função chamada "convert_to_int" que recebe uma referência para uma string e retorna um valor inteiro (i32)
fn convert_to_int(data_input: & String) -> i32{
    // Essa linha cria uma variável local chamada "x" e atribui a ela o resultado da chamada de método "parse()" na string referenciada por "data_input"
    // O método "parse()" tenta converter o conteúdo da string em um tipo especificado entre os sinais "::<>", nesse caso, um i32
    // O método "unwrap()" é chamado em seguida para obter o valor armazenado na variante "Ok" do tipo "Result" retornado pelo método "parse()"
    // Se o método "parse()" retornar a variante "Err", o método "unwrap()" causará um panic
    let x = data_input().parse::<i32>().unwrap();
    x // Essa linha retorna o valor da variável "x"
}

// Essa linha define a função principal do programa, que é executada quando o programa é iniciado
fn main() {

    // Essa linha cria uma variável mutável chamada "number1" do tipo "String" e a inicializa com uma string vazia
    let mut number1 = String::new();

    // Essa linha chama o método "read_line()" da biblioteca "io", passando a referência para "number1" como argumento
    // O método "read_line()" lê uma linha de entrada do usuário até encontrar um caractere de nova linha
    // O método "expect()" é chamado em seguida para tratar qualquer erro que possa ocorrer durante a chamada ao método "read_line()"
    // Se ocorrer um erro, o método "expect()" causará um panic com a mensagem de erro passada como argumento
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    // Essa linha é um bloco de código que é executado se a condição for verdadeira
    // A condição é avaliada chamando a função "convert_to_int()" com as referências para "number1" e "number2" como argumentos e comparando se o valor retornado pela primeira chamada é maior que o valor retornado pela segunda
    if convert_to_int(&number1) > convert_to_int(&number2){
        // Se a condição for verdadeira, a mensagem "O número {} é maior que {}" será impressa.
        println!("O numero {} eh maior que {}", number1, number2);
    } else{
        // Caso contrário, a mensagem "O número {number1} é menor ou igual a {number2}" é impressa
        println!("O numero {} eh menor ou igual que {}", number1, number2);

    }
}
```

---

#### Código fatorial:

```rust
use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial");
    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial);

    while entrada_int > 1 {
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int - 1;
    }

    println!("O fatorial eh {}", fatorial);
}
```

- use std::io: importa a biblioteca io para acessar funções de leitura e escrita de dados.

- fn convert_to_int(data_input: & String) -> i32: declara a função convert_to_int que recebe uma string e retorna um inteiro. A string é passada como referência (&) para a função para evitar uma cópia desnecessária dos dados.

- let x = data_input.trim().parse::<i32>().unwrap(): essa linha converte a string em um inteiro. O método trim remove os espaços em branco no início e no final da string. Em seguida, o método parse tenta realizar a conversão para o tipo especificado entre os sinais de maior e menor <>, no caso i32 (um inteiro de 32 bits). O método unwrap retorna o valor da operação caso ela tenha sido bem sucedida, ou lança um erro caso contrário.

- let mut entrada_fatorial = String::new(): cria uma variável mutável (mut) chamada entrada_fatorial com valor inicial vazio (String::new()).

- io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial"): lê uma linha de entrada do usuário e armazena em entrada_fatorial. O método expect é usado para tratar possíveis erros na leitura dos dados.

- let mut fatorial = 1: cria uma variável mutável fatorial com valor inicial 1.

- let mut entrada_int = convert_to_int(&entrada_fatorial): cria uma variável mutável entrada_int e chama a função convert_to_int passando a string entrada_fatorial como argumento e armazena o resultado na variável mutável entrada_int.

- while entrada_int > 1 Inicia um loop que continua enquanto entrada_int for maior que 1.

- fatorial = fatorial \* entrada_int; Atualiza o valor de fatorial para o valor atual de fatorial multiplicado por entrada_int.

- entrada_int = entrada_int - 1; Diminui o valor de entrada_int em 1.

- println!("O fatorial eh {}", fatorial); Imprime a mensagem "O fatorial é" seguida pelo valor atual de fatorial.

---

#### Função Parte 1

```rust
fn dobro(num: i32) -> i32{
    return 2*num;
}

fn maior(a: i32, b: i32) -> i32
{
    if a >= b{
        return a
    } else{
        return b;
    }
}
fn main() {

    println!("O maior numero entre 5 e 4 eh {}", maior(5,4) );
}
```

- O código acima tem três funções: dobro, maior e main.
- A função dobro tem como parâmetro um inteiro chamado "num" e retorna o dobro desse número, multiplicando-o por 2.
- A função maior tem como parâmetros dois inteiros, "a" e "b". Ela compara os dois números e retorna o maior deles. Se "a" for maior ou igual a "b", retorna "a". Caso contrário, retorna "b".
- A função main é a função principal do código. Ela é executada automaticamente quando o código é rodado. Nela, é chamada a função maior, passando os números 5 e 4 como parâmetros. O resultado da chamada da função é impresso na tela com a mensagem "O maior número entre 5 e 4 é 5".

---
