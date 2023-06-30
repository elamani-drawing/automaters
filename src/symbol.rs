use std::str::FromStr;
// string n'implemente pas copy, donc on peut juste utiliser clone
/// Un symbole 
#[derive(Debug, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Symbol {
    // la valeur du symbol
    value: String, 
}


impl Symbol {
    
    /// Créer un Symbol
    ///
    /// # Arguments
    ///
    /// * `_value` - La valeur du symbole de Self
    ///
    /// # Example
    ///
    /// ```
    /// use automaters::Symbol;
    /// fn main() {
    ///     let string_one : String = String::from("symbol");
    ///     let Symbol_one : Symbol = Symbol::new(string_one);
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `Symbol` - Le Symbol qui a été créer
    ///
    pub fn new(_value : String) -> Self {
        Symbol { value: _value }
    }

    pub fn from_str(_value : &str) -> Self {
        Symbol { value: String::from_str(_value).unwrap() }
    }

    /// Retourne la valeur du Symbol
    ///
    /// # Example
    ///
    /// ```
    /// use automaters::Symbol;
    /// fn main() {
    ///     let string_one : String = String::from("Symbol");
    ///     let symbol_one : Symbol = Symbol::new(string_one.clone());
    ///     println!("{}", symbol_one.get_value());
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `&String` - Le symbol de selfs
    ///
    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.get_value() == other.get_value()
    }

    fn ne(&self, other: &Symbol) -> bool {
        self.get_value() != other.get_value()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_copy() {      
        let string_one : String = String::from("Symbol");
        let string_two : String = String::from("Symbol two");

        let symbol_one : Symbol = Symbol::new(string_one.clone());
        let symbol_one_bis : Symbol = Symbol::from_str(string_one.clone().as_str());
        let symbol_two : Symbol = Symbol::new(string_two);
        
        assert_eq!(symbol_one, symbol_one_bis);
        assert_eq!(symbol_one==symbol_two, false);
        assert_eq!(symbol_one.get_value(), &(string_one));
    }
}