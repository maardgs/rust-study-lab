pub fn run() {

    let s = String::from("hello rust world");
    consumir_string(s);

    let mut nome = String::from("mari");
    adicionar_sufixo(&mut nome);
    println!("Depois da modificação: {}", nome);

    let frase = String::from("rust é brabo");
    let primeira = primeira_palavra(&frase);
    println!("Primeira palavra: {}", primeira);
}

fn consumir_string(texto: String) {
    println!("Consumindo (move): {}", texto);
} 

fn adicionar_sufixo(texto: &mut String) {
    texto.push_str("_dev");
}

fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
