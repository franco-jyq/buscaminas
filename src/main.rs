use std::env;
use std::fs;

const SALTO_DE_LINEA: u8 = 10;
mod tablero;
use tablero::Tablero;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    
    println!("In file {}", file_path);

    let  contents =
        fs::read_to_string(file_path).expect("No se pudo leer el archivo");
 
    // Convierto el String en slice de bytes
    let bytes = contents.as_bytes();

    let tupla = formatear_entrada(bytes);
    let mut tablero: Tablero = Tablero::crear_tablero(tupla.1, tupla.2,tupla.0);
    tablero.resolver_tablero();
    tablero.imprimir_tablero();
    tablero.escribir_archivo(&"tablero2.txt".to_string());
      
}

fn formatear_entrada(slice: &[u8]) -> (Vec<u8>,usize, usize){
    let mut resultado: Vec<u8> =  Vec::new();
    let mut cont_columnas = 0;
    let mut cont_filas = 0;
    for &elemento in slice{
        cont_columnas += 1;
        if elemento != SALTO_DE_LINEA {
            if elemento != 42 && elemento != 46 {
                panic!("Tablero invalido");
            }
            resultado.push(elemento);

        }else{       
            cont_filas += 1;
            cont_columnas = 0;
        }
    }
    cont_filas += 1;   

    (resultado, cont_filas,cont_columnas)
}
