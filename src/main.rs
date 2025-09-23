use std::io::stdin;
use type_game::{interfaz::*, juego_clasico, juego_personalizado, juego_todas_las_palabras};

fn main() {
    //Empezamos con un menu de bienvenida
    let mut opcion = String::new();
    loop {
        menu_inicial();

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
                    break;
                }
                "2" => {
                    juego_personalizado();
                    break;
                }
                "3" => {
                    juego_todas_las_palabras();
                    break;
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
        loop {
            opcion.clear();
            println!(
                "Si quieres volver a jugar, presiona Enter. Si quieres salir, escribe 'salir'"
            );
            stdin()
                .read_line(&mut opcion)
                .expect("Algo salio mal, intenta de nuevo");
            match opcion.trim() {
                "" => break,
                "salir" => std::process::exit(0),
                _ => continue,
            }
        }
    }
}
