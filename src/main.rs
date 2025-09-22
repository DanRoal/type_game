use std::io::stdin;
use type_game::{interfaz::*, juego_clasico, juego_personalizado, juego_todas_las_palabras};

fn main() {
    //Empezamos con un menu de bienvenida
    menu_inicial();

    let mut opcion = String::new();

    //Creamos la variable opcion y hacemos un bucle infinito que este checando siempre el input del usuario
    loop {
        stdin().read_line(&mut opcion).expect("Inténtalo de nuevo");
        match opcion.trim() {
            "1" => {
                menu_secundario();
                opcion.clear();
                break;
            }
            "2" => {
                mensaje_salida();
                std::process::exit(0)
            }
            _ => {
                println!("Opción inválida, por favor intente de nuevo");
                opcion.clear();
                continue;
            }
        }
    }
    //Empezamos otro loop que chece el input del usuario aunque este sera justo despues del menu secundario
    //Ya limpiamos la variable en el loop anterior por lo que no necesitamos hacerlo ahora.
    loop {
        stdin().read_line(&mut opcion).expect("Inténtalo de nuevo");
        match opcion.trim() {
            "1" => {
                juego_clasico();
            }
            "2" => {
                juego_personalizado();
            }
            "3" => {
                juego_todas_las_palabras();
            }
            "4" => {
                mensaje_salida();
                std::process::exit(0)
            }
            _ => {
                println!("Opción inválida, por favor intente de nuevo");
                opcion.clear();
                continue;
            }
        }
    }
}
