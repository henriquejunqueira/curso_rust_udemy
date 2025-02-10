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
