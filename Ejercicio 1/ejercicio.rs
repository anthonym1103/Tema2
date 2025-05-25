use sha2::{Sha256, Digest};
use rand::Rng;
use std::time::SystemTime;

struct Nodo {
    partida: String,
    cuerpo: Vec<i32>,
    firma: String,
}

impl Nodo {
    fn new(partida: String, cuerpo: Vec<i32>) -> Self {
        let mut concat = partida.clone() + " ";
        for num in &cuerpo { concat += &num.to_string() + " "; }
        let mut hasher = Sha256::new();
        hasher.update(concat.as_bytes());
        let firma = format!("{:x}", hasher.finalize());
        Nodo { partida, cuerpo, firma }
    }
}

fn generar_lista(n: usize, k: usize) -> Vec<Nodo> {
    let mut rng = rand::thread_rng();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut partida_actual = format!("{:x}", Sha256::digest(now.to_string().as_bytes()));
    let mut lista = Vec::new();
    
    for _ in 0..n {
        let cuerpo: Vec<i32> = (0..k).map(|_| rng.gen_range(1..=100000)).collect();
        let nodo = Nodo::new(partida_actual.clone(), cuerpo);
        partida_actual = nodo.firma.clone();
        lista.push(nodo);
    }
    lista
}

fn main() {
    let lista = generar_lista(3, 4);
    for nodo in lista {
        println!("Partida: {}...\nCuerpo: {:?}\nFirma: {}...\n", 
            &nodo.partida[0..64], nodo.cuerpo, &nodo.firma[0..64]);
    }
}
