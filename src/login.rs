use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
enum LoginError {
    SenhaIncorreta,
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::SenhaIncorreta => write!(f, "Senha incorreta"),
        }
    }
}

pub fn run() {
    let usuarios = base_usuarios();

    println!("=== Sistema de Login (Aula 9) ===");

    let username = ler_input("Usuário: ");
    let senha = ler_input("Senha: ");

    let senha_armazenada = match usuarios.get(&username) {
        Some(s) => s,
        None => {
            println!("Erro: usuário não encontrado");
            return;
        }
    };

    match validar_senha(senha_armazenada, &senha) {
        Ok(()) => println!("✅ Bem-vinda, {}!", username),
        Err(e) => println!("Erro: {}", e),
    }
}

fn base_usuarios() -> HashMap<String, String> {
    let mut usuarios = HashMap::new();

    usuarios.insert("mari".to_string(), "1234".to_string());
    usuarios.insert("admin".to_string(), "rust".to_string());
    usuarios.insert("guest".to_string(), "guest".to_string());

    usuarios
}

fn validar_senha(senha_correta: &str, senha_digitada: &str) -> Result<(), LoginError> {
    if senha_digitada == senha_correta {
        Ok(())
    } else {
        Err(LoginError::SenhaIncorreta)
    }
}

fn ler_input(label: &str) -> String {
    let mut input = String::new();

    print!("{}", label);
    io::stdout().flush().unwrap(); 

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");

    input.trim().to_string()
}
