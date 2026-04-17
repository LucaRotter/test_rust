mod models;
use models::Task;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;
fn main() {
    let mut lista_task: Vec<Task> = caricamento_da_file();

    let comando: u32 = chiedi_comando_base();
    match comando {
        0 => aggiungi_task(),
        1 => visualizza_task(),
        2 => println!("termine programma, arrivederci! :)"),
        _ => println!("Comando non valido, ma il filtro nel loop dovrebbe averlo evitato!"),
    }
}

fn chiedi_comando_base() -> u32 {
    println!("Benvenuto nel programma! Inseririsci un INPUT:");
    println!("0 per aggiungere una task");
    println!("1 per visuallizzare le task da fare");
    println!("2 per uscire");
    loop {
        println!("scegli un opzione:");
        let mut com: String = String::new();
        io::stdin().read_line(&mut com).unwrap();
        match com.trim().parse::<u32>() {
            Ok(num) => return (num),
            Err(_) => println!("Errore: Inserisci un numero intero, per favore."),
        }
    }
}

fn aggiungi_task() {}
fn visualizza_task() {}
fn caricamento_da_file() -> Vec<Task> {
    let nome_file: &str = "tasks.json";
    //lettura normale del file in testo
    let contenuto_file = match fs::read_to_string(nome_file) {
        Ok(testo) => testo,
        Err(_) => {
            println!("File non trovato, inizializzo una lista vuota.");
            return Vec::new(); // Se il file non c'è, usciamo subito con un vettore vuoto
        }
    };
    // La String (testo JSON) viene analizzata e trasformata in un Vec<Task>
    let tasks: Vec<Task> = match serde_json::from_str(&contenuto_file) {
        Ok(lista) => lista,
        Err(e) => {
            println!("Errore nel formato JSON: {}", e);
            Vec::new() // In caso di file corrotto, restituiamo un vettore vuoto
        }
    };

    tasks
}

fn scrittura_su_file(lista: &Vec<Task>){
    // Il vettore di struct Task viene trasformato in una String (testo JSON)
    let json = serde_json::to_string_pretty(lista).expect("errore nella deserializzazione");
    let mut file = File::create("tasks.json").expect("Errore creazione file");
    let mut file = File::create("tasks.json").expect("Errore creazione file");
}

