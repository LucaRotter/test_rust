mod models;
use models::Task;
use models::TaskError;
use std::f32::consts::E;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;
fn main() {
    let mut lista_task: Vec<Task> = match caricamento_da_file() {
        Ok(task) => {
            println!("Dati caricati con successo!");
            task
        },
        Err(e) => {
            eprintln!("Errore durante il caricamento delle task ");
            Vec::new()
        }
    };
    loop{
    //chiedi all'utente cosa vuole fare
    let comando: u32 = chiedi_comando_base();
    //match del comando scelto
    match comando {
        0 => aggiungi_task(&mut lista_task),
        1 => visualizza_task(&mut lista_task),
        2 => {
            println!("termine programma, arrivederci! :)"); 
            if let Err(e) = scrittura_su_file(&lista_task) {
                eprintln!("Errore durante la scrittura sul file: {:?}" , e);
            }
            break;
        },
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
            Ok(num) => break num,
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
        return;
    } else {
        println!("Ecco tutte le task:");
        for task in lista_task.iter() {
            println!("ID: {}, Testo: {}, Completato: {}", task.id, task.testo, task.completato);
        }
    }
    println!("");
    println!("vuoi modificare lo stato di qualche task? (s/n)");
    let mut risposta = String::new();
    io::stdin().read_line(&mut risposta).unwrap();
    match risposta.trim().to_lowercase().as_str() {
        "s" => modifica_stato_task(lista_task),
        "n" => {
            println!("Nessuna modifica alle task... torno al menu"); 
        },
        _ => println!("Risposta non valida, nessuna modifica alle task."),
    };
}
fn modifica_stato_task(lista_task: &mut Vec<Task>) {
    println!("inserisci l'ID della task da modificare:");
    let mut id_input_dacercare: u32=0;
    let mut trovato: bool = false;
    loop{
        let mut id_input = String::new();
        io::stdin().read_line(&mut id_input).unwrap();

        match id_input.trim().parse::<u32>() {
            Ok(num) => {
                for task in lista_task.iter(){
                    if task.id == num{
                        trovato = true;
                        id_input_dacercare = num;
                        break;
                    }
                }
                break;
            }
            Err(_) => println!("Errore: Inserisci un numero intero valido per l'ID, per favore."),
        }
    }
    if trovato == true {
        modifica_stato_taskID(id_input_dacercare, lista_task);
    }
}
fn modifica_stato_taskID(id_input_dacercare: u32, lista_task: &mut Vec<Task>)-> Result<(), TaskError>{
    println!("l'id da cercare è {}", id_input_dacercare);
    let mut flag: bool = false;
    for task in lista_task.iter_mut(){
        
        if task.id == id_input_dacercare {
            flag = true;
            let comp: bool = task.completato;
            if comp == false {
                println!("la task con ID {} è attualmente incompleta, vuoi completarla? (s/n)", task.id);
                let mut risposta: String = String::new();
                io::stdin().read_line(&mut risposta).map_err(TaskError::IoError)?;
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
                io::stdin().read_line(&mut risposta).map_err(TaskError::IoError)?;
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
Ok(())
}

fn caricamento_da_file() -> Result<Vec<Task>, TaskError> {
    let nome_file: &str = "tasks.json";
    let contenuto_file = fs::read_to_string(&nome_file).map_err(TaskError::IoError)?;
    let tasks: Vec<Task> = serde_json::from_str(&contenuto_file).map_err(TaskError::SerdeError)?;
    Ok(tasks)
}

fn scrittura_su_file(lista: &Vec<Task>)-> Result<(), TaskError>{
    // Il vettore di struct Task viene trasformato in una String (testo JSON)
    println!("**********************************");
    println!("aggiorno le task...");
    println!("**********************************");
    let json = serde_json::to_string_pretty(lista).map_err(TaskError::SerdeError)?;
    let mut file = File::create("tasks.json").map_err(TaskError::IoError)?;
    file.write_all(json.as_bytes()).map_err(TaskError::IoError)?;
    Ok(())
}

