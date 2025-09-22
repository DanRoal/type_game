use palabras::LISTA;
use rand::seq::{IndexedRandom, SliceRandom};
use std::{io::stdin, time::Instant};

use crate::interfaz::*;

pub mod interfaz;
pub mod palabras;

pub fn elegir_palabras(num: usize) -> Vec<&'static str> {
    let mut rng = rand::rng();
    let mut selection: Vec<&str> = LISTA.choose_multiple(&mut rng, num).map(|s| *s).collect();
    selection.shuffle(&mut rng);
    selection
}

pub fn jugar(palabras: Vec<&str>, errores: &mut u32) {
    let mut i = 0;
    let mut input = String::new();
    while i < palabras.len() {
        imprimir_palabra(palabras[i]);
        stdin().read_line(&mut input).expect("Algo salio mal");
        if input.trim() == "salir" {
            mensaje_salida();
            std::process::exit(0);
        } else if input.trim() == palabras[i] {
            input.clear();
            i += 1;
            continue;
        } else {
            *errores += 1;
            input.clear();
        }
    }
}

pub fn juego_clasico() {
    let mut errores: u32 = 0;
    let cantidad: u32 = 30;
    let palabras = elegir_palabras(cantidad.try_into().unwrap());
    let inicio = Instant::now();
    jugar(palabras, &mut errores);
    let duracion = inicio.elapsed();
    mensaje_final(errores, duracion, cantidad);
    mensaje_salida();
    std::process::exit(0);
}

pub fn juego_personalizado() {
    let mut errores: u32 = 0;
    println!("Por favor ingresa la cantidad de palabras que quieres escribir:");
    let mut cantidad = String::new();
    loop {
        stdin()
            .read_line(&mut cantidad)
            .expect("Intentalo de nuevo");
        match cantidad.trim().parse::<u32>() {
            Ok(n) => {
                if (1..=LISTA.len() as u32).contains(&n) {
                    println!("Empezamos el juego con {cantidad} palabras!!");
                    break;
                } else {
                    println!("Por favor ingresa un número entre 1 y {}", LISTA.len());
                    continue;
                }
            }
            Err(_) => {
                println!("Eso no es un número");
                cantidad.clear();
                continue;
            }
        }
    }
    let cantidad = cantidad.trim().parse::<u32>().expect("Algo salio mal");

    let palabras = elegir_palabras(cantidad.try_into().unwrap());
    let inicio = Instant::now();
    jugar(palabras, &mut errores);
    let duracion = inicio.elapsed();
    mensaje_final(errores, duracion, cantidad);
    mensaje_salida();
    std::process::exit(0);
}

pub fn juego_todas_las_palabras() {
    let mut errores: u32 = 0;
    let cantidad: u32 = LISTA.len().try_into().unwrap();
    let palabras = elegir_palabras(cantidad.try_into().unwrap());
    let inicio = Instant::now();
    jugar(palabras, &mut errores);
    let duracion = inicio.elapsed();
    mensaje_final(errores, duracion, cantidad);
    mensaje_salida();
    std::process::exit(0);
}
