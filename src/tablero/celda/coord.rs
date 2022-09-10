
#[derive(Debug)]
pub struct Coord {
    x1: i32,
    y1: i32,
}

impl Coord {
    /// Constructor
    /// Devolver Self es equivalente a devolver la clase que estoy "extendiendo"
    pub fn new(x: usize, y: usize) -> Self {
        let x1: i32 = x.try_into().expect("La coordenada debe ser positiva");
        let y1: i32 = y.try_into().expect("La coordenada debe ser positiva");
        Coord { x1, y1 }
    }

    pub fn obtener_xy(&self) -> (i32,i32) {
        (self.x1, self.y1)
    }

    // Devuelve true si la coordenada recibida esta entre las adyacentes
    pub fn esta_alrededor(&self, coordenada: &Coord) -> bool {
        let a1 = self.x1;
        let a2 = self.y1;
        let caso_a = (a1 + 1, a2);  // derecha
        let caso_b = (a1 - 1, a2);  // izq
        let caso_c = (a1 , a2 + 1);  // arrib
        let caso_d = (a1 , a2 - 1);  // abajo
        let caso_e = (a1 + 1, a2 + 1);  //arriba der
        let caso_f = (a1 - 1, a2 + 1);  // arriba izq
        let caso_g = (a1 + 1, a2 - 1);    // abajo der
        let caso_h = (a1 - 1, a2 - 1);   // abajo izq
        if coordenada.obtener_xy() == caso_a {
            return true
        }
        if coordenada.obtener_xy() == caso_a {
            return true
        }
        if coordenada.obtener_xy() == caso_b {
            return true
        }
        if coordenada.obtener_xy() == caso_c {
            return true
        }
        if coordenada.obtener_xy() == caso_d {
            return true
        }
        if coordenada.obtener_xy() == caso_e {
            return true
        }
        if coordenada.obtener_xy() == caso_f {
            return true
        }
        if coordenada.obtener_xy() == caso_g {
            return true
        }
        if coordenada.obtener_xy() == caso_h {
            return true
        }
        return false
        
        
    }
}