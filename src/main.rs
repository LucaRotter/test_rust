mod models;
use models::Task;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;
fn main() {
    let mut lista_task: Vec<Task> = caricamento_da_file();
    loop{
    //chiedi all'utente cosa vuole fare
    let comando: u32 = chiedi_comando_base();
    //match del comando scelto
    match comando {
        0 => aggiungi_task(&mut lista_task),
        1 => visualizza_task(&mut lista_task),
        2 => {
            println!("termine programma, arrivederci! :)"); 
            scrittura_su_file(&lista_task);    
            break;},
        _ => println!("Comando non valido, ma il filtro nel loop dovrebbe averlo evitato!"),
    }
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

fn aggiungi_task(lista_task: &mut Vec<Task>) {
    let lunghezza_lista_iniziale: usize = lista_task.len();
    println!("inserisci il nome della task da aggiungere:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let nuova_task = Task{
        id: lista_task.len() as u32 + 1,
        testo: task.trim().to_string(),
        completato: false,
    };
    &mut lista_task.push(nuova_task);
        if lista_task.len() > lunghezza_lista_iniziale {
            println!("Task aggiunta con successo!");
        } else {
            println!("Errore: Task non aggiunta.");
        }
}
fn visualizza_task(lista_task: &mut Vec<Task>) {
    if lista_task.is_empty() {
        println!("Non ci sono task da visualizzare.");
    } else {
        println!("Ecco le task da fare:");
        for task in lista_task.iter() {
            println!("ID: {}, Testo: {}, Completato: {}", task.id, task.testo, task.completato);
        }
    }
    println!("vuoi modificare lo stato di qualche task? (s/n)");
    let mut risposta = String::new();
    io::stdin().read_line(&mut risposta).unwrap();
    match risposta.trim().to_lowercase().as_str() {
        "s" => modifica_stato_task(lista_task),
        "n" => println!("Nessuna modifica alle task."),
        _ => println!("Risposta non valida, nessuna modifica alle task."),
    };
}
fn modifica_stato_task(lista_task: &mut Vec<Task>) {
    println!("inserisci l'ID della task da modificare:");
    let mut id_input_dacercare: u32 = 0;

    loop{
        let mut id_input = String::new();
        io::stdin().read_line(&mut id_input).unwrap();

        match id_input.trim().parse::<u32>() {
            Ok(num) => {
                id_input_dacercare = num;
                break;
            }
            Err(_) => println!("Errore: Inserisci un numero intero valido per l'ID, per favore."),
        }
    }

    for task in lista_task.iter_mut(){
        let mut flag: bool = false;
        if task.id == id_input_dacercare {
            flag = true;
            let comp: bool = task.completato;
            if comp == false {
                println!("la task con ID {} è attualmente incompleta, vuoi completarla? (s/n)", task.id);ù
                let mut risposta: String = String::new();
                io::stdin().read_line(&mut risposta).unwrap();
                match risposta.trim().to_lowercase().as_str() {
                    "s" => {task.completato = true;
                        println!("Task con ID {} completata!", task.id);
                    },
                    "n" => println!("Nessuna modifica alla task con ID {}.", task.id),
                    _ => println!("Risposta non valida, nessuna modifica alla task con ID {}.", task.id),
                };
                break;
            }
            else{
                println!("la task con ID {} è attualmente completata, vuoi renderla incompleta? (s/n)", task.id);
                let mut risposta: String = String::new();
                io::stdin().read_line(&mut risposta).unwrap();
                match risposta.trim().to_lowercase().as_str() {
                    "s" => {task.completato = false;
                        println!("Task con ID {} resa incompleta!", task.id);
                    },
                    "n" => println!("Nessuna modifica alla task con ID {}.", task.id),
                    _ => println!("Risposta non valida, nessuna modifica alla task con ID {}.", task.id),
                };
                break;
            }
        }
    
    if flag == false {
        println!("Errore, Non esiste nessuna task con ID {}.", id_input_dacercare);
    }
}
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

