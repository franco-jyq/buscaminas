mod celda;
use celda::Celda;
use celda::coord::Coord;
use std::fs::File;
use std::io::{Write};
const CARACTER_DE_BOMBA: &str= "*";


pub struct Tablero{
    matriz: Vec<Vec<Celda>>,
    filas: usize,
    columnas: usize,
}

impl Tablero {
    
    pub fn crear_tablero(filas:usize, columnas:usize,celdas: Vec<u8>)->Self{
        Tablero {
            matriz : Self::llenar_tablero(celdas,filas,columnas),
            filas,
            columnas,
        }
    }   


    fn llenar_tablero(celdas:Vec<u8>,filas:usize,columnas:usize)-> Vec<Vec<Celda>> {
        let mut matriz: Vec<Vec<Celda>> =  Vec::new();
        let mut celdas_iter = celdas.iter();
        for  i in 0..filas {
            matriz.push(Vec::new());
            for j in 0..columnas {
                let siguiente_celda = *celdas_iter.next().expect("Tablero invalido");
                let  celda = Celda::new(siguiente_celda,Coord::new(i,j));
                matriz[i].push(celda);
            }
        }
        matriz
    }

    pub fn imprimir_tablero(&self){
        let mut columnas = 0;
        for vector in &self.matriz{
            if columnas == self.columnas {
                println!("");
                columnas = 0;
            }
            for celda in vector{
                celda.imprimir_celda();
                columnas += 1
            }
        }
        println!("");
    }

    fn actualizar_vecinos(&mut self, coordenada_con_bomba:  Coord) {
        for vector in &mut self.matriz{
            for celda in vector{
                if celda.tiene_bomba(){
                    continue;
                }
                if coordenada_con_bomba.esta_alrededor(&celda.obtener_coordenada()) {
                    celda.sumar_bomba();
                }
            }
        }
    }
        
    pub fn resolver_tablero(&mut self){
        
        for f in  0..self.filas {
            for c in 0..self.columnas{
                if (&self.matriz[f][c]).tiene_bomba(){
                    self.actualizar_vecinos( Coord::new(f,c));
                }
            }
        }
    }

    pub fn escribir_archivo(self,file_path:&String){
        let mut output = File::create(file_path).expect("Archivo destino invalido");
        let mut columnas = 0;
        for vector in &self.matriz{
            if columnas == self.columnas {
                write!(output, "\n").expect("No se pudo escribir");
                //println!("");
                columnas = 0;
            }
            for celda in vector{
                if celda.tiene_bomba() {
                    write!(output, "{}", CARACTER_DE_BOMBA).expect("No se pudo escribir");
                }
                else{ 
                    write!(output, "{}", celda.obtener_cantidad_de_bombas_adyacentes()).expect("No se pudo escribir");
                }
                columnas += 1
            }
        }
    }

}






