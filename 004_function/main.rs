use std::io;

fn main() {
    pergunta();
}

fn pergunta(){
    println!("Informe seu nome: ");
    let mut nome = String::new();
    
    io::stdin().read_line(&mut nome)
        .expect("Erro");
    
    println!("Seu nome é: {}", nome);
}
