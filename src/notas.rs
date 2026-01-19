pub fn run() {
    let notas = vec![7.5, 8.0, 4.5, 6.0, 9.0, 3.5, 10.0, 5.0, 6.5, 2.0];

    let media = calcular_media(&notas);
    let maior = maior_nota(&notas);
    let menor = menor_nota(&notas);
    let (aprovadas, reprovadas) = classificar_notas(&notas);

    println!("Média: {:.2}", media);
    println!("Maior nota: {}", maior);
    println!("Menor nota: {}", menor);
    println!("Aprovadas: {}", aprovadas);
    println!("Reprovadas: {}", reprovadas);
}

fn calcular_media(notas: &Vec<f64>) -> f64 {
    let soma: f64 = notas.iter().sum();
    soma / notas.len() as f64
}

fn maior_nota(notas: &Vec<f64>) -> f64 {
    *notas.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

fn menor_nota(notas: &Vec<f64>) -> f64 {
    *notas.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

fn classificar_notas(notas: &Vec<f64>) -> (usize, usize) {
    let mut aprovadas = 0;
    let mut reprovadas = 0;

    for nota in notas {
        if *nota > 5.0 {
            aprovadas += 1;
        } else {
            reprovadas += 1;
        }
    }

    (aprovadas, reprovadas)
}
