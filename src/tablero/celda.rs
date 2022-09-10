pub(crate) mod coord;
use coord::Coord;
const TIENE_BOMBA: u8 = 42; // ascii: * = 42
const CARACTER_DE_BOMBA_PARA_TERMINAL: &str= "*";

#[derive(Debug)]
pub struct Celda {
    coordenada: Coord,
    tiene_bomba: bool,
    bombas_adyacentes: u8,
}

impl Celda {

    pub fn new(tipo:u8, coordenada: Coord) -> Self {
        Celda{
            tiene_bomba: { tipo == TIENE_BOMBA },
            coordenada,
            bombas_adyacentes: 0,
        }
    }

    pub fn tiene_bomba(&self) -> bool{
            self.tiene_bomba
    }

    pub fn imprimir_celda(&self){
        if self.tiene_bomba() {
            print!("{}",CARACTER_DE_BOMBA_PARA_TERMINAL);
        }else{
            print!("{}",self.bombas_adyacentes);
        }
        
    }

    pub fn obtener_cantidad_de_bombas_adyacentes(&self) -> String{
        self.bombas_adyacentes.to_string()
    }

    pub fn sumar_bomba(&mut self){
        self.bombas_adyacentes += 1;
    }

    pub fn obtener_coordenada(&self) -> &Coord{
        &self.coordenada
    }

}