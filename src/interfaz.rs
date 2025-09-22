use std::time::Duration;

pub fn menu_inicial() {
    println!("==============================");
    println!("        Type game :D           ");
    println!("==============================");
    println!("");
    println!("1) Iniciar juego");
    println!("2) Salir");
    println!("");
    println!("==============================");
    println!("Elige una opción: ");
}

pub fn menu_secundario() {
    println!("==============================");
    println!("        Type game :D          ");
    println!("==============================");
    println!("Escoge un modo de juego:");
    println!("1) Modo clásico (30 palabras)");
    println!("2) Personalizado");
    println!("3) Todas las palabras");
    println!("4) Salir");
    println!("==============================");
    println!("Elige una opción: ");
}

pub fn imprimir_palabra(word: &str){
    println!("============================================================");
    println!("                Escribe la siguiente palabra:          ");
    println!("============================================================");
    println!("");
    println!("");
    println!("                           \x1b[31m{}\x1b[0m", word);
    println!("");
    println!("");
    println!("");
    println!("Escribe 'salir' para salir");
    println!("============================================================");
    println!("");
}

pub fn mensaje_final(err: u32, tiempo: Duration, cantidad_palabras: u32){
    println!("============================================================");
    println!("                ¡¡FELICIDADES, YA ACABASTE!!          ");
    println!("============================================================");
    println!("");
    println!("Terminaste en: {:?}", tiempo);
    println!("");
    println!("Tuviste {err} errores");
    println!("");
    println!("Escribiste {cantidad_palabras} palabras");
    println!("");
    println!("============================================================");
    println!("");

}

pub fn mensaje_salida() {
    println!("Muchas gracias por haber estado aquí!!");
}
