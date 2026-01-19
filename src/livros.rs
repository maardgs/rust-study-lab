#[derive(Debug)]
struct Livro {
    titulo: String,
    autor: String,
    ano: u16,
}

pub fn run() {
    let livros = vec![
        Livro {
            titulo: String::from("Clean Code"),
            autor: String::from("Robert C. Martin"),
            ano: 2008,
        },
        Livro {
            titulo: String::from("The Pragmatic Programmer"),
            autor: String::from("Andrew Hunt"),
            ano: 1999,
        },
        Livro {
            titulo: String::from("Rust Book"),
            autor: String::from("Steve Klabnik"),
            ano: 2019,
        },
        Livro {
            titulo: String::from("Design Patterns"),
            autor: String::from("GoF"),
            ano: 1994,
        },
    ];

    println!("📚 Todos os livros:");
    imprimir_livros(&livros);

    println!("\n📆 Livros publicados a partir dos anos 2000:");
    let livros_2000 = &livros[1..3]; // slice
    imprimir_livros(livros_2000);
}

fn imprimir_livros(livros: &[Livro]) {
    for livro in livros {
        println!(
            "📘 {} | Autor: {} | Ano: {}",
            livro.titulo, livro.autor, livro.ano
        );
    }
}
