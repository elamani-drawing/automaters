use super::{State, Symbol, BTSet};
use std::fs;
use serde_json::{Value, from_str};


/// Machine à état fini 
#[derive(Debug, Clone)]
pub struct FSM {
    states: BTSet<State>, // set des states de la machine
    alphabet: BTSet<Symbol>,// set de symbole
    ends: BTSet<State>,// set des etats finaux de la machine
}

impl FSM {
    /// Creer une machine à etat fini
    /// 
    /// # Arguments
    ///
    /// * `_states` - Les états de la machine
    /// * `_alphabet` - L'alphabet de la machine
    /// * `_ends` - Les états finaux de la machine
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automaters::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let fsm : FSM = FSM::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let fsm2 : FSM = FSM::from_json(&content_json);
    ///     //creation depuis new
    ///     let fsm3 : FSM = FSM::new(fsm.get_states().clone(), fsm.get_alphabet().clone(), fsm.get_ends().clone());  
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FSM` - La machine à état fini correspondante
    /// 
    pub fn new(_states : BTSet<State>, _alphabet: BTSet<Symbol>, _ends: BTSet<State> ) -> Self {
        FSM{
            states : _states,
            alphabet : _alphabet,
            ends: _ends
        }
    }

    /// Creer une machine à etat fini depuis un json
    /// 
    /// # Arguments
    ///
    /// * `content_json` - Le contenu json
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automaters::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let fsm : FSM = FSM::from_json(&content_json); 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FSM` - La machine à état fini correspondante
    /// 
    pub fn from_json(content_json: &Value) -> Self {
        //creation de la machine à l'aide du content_json
        
        let mut alphabet: BTSet<Symbol> = BTSet::new();
        let mut symbol: Symbol;
        for elem in content_json["alphabet"].as_array().unwrap(){
            symbol = Symbol::new(elem.as_str().unwrap().to_string()); //Création du symbol
            alphabet.insert(symbol);//ajout du symbol 
        }
        // reccupere les differents etats
        let mut state: State; 
        let mut states: BTSet<State> = BTSet::new();
        for elem in content_json["states"].as_array().unwrap(){
            state = State::new(elem.as_str().unwrap().to_string());
            states.insert(state);
        }
        // reccupere les etats finaux
        let mut ends: BTSet<State> = BTSet::new();
        for elem in content_json["ends"].as_array().unwrap(){
            state = State::new(elem.as_str().unwrap().to_string());
            ends.insert(state);
        }

        FSM {
            alphabet : alphabet,
            states : states,
            ends: ends,
        }
    }

    /// Créer une machine à état fini depuis un chemin vers un fichier json
    /// 
    /// # Arguments
    ///
    /// * `path` - Le schemin vers le fichier json
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["q_0","q_1"],
    ///     "alphabet" : ["b","a"],
    ///     "ends" : ["q_0"],
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automaters::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     //creation depuis un lien
    ///     let fsm : FSM = FSM::from_json_file(link_file);   
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `FSM` - La machine à état fini correspondante
    /// 
    pub fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        FSM::from_json(&content_json)
    }

    /// Retourne les états de la machine
    pub fn get_states(&self) -> &BTSet<State> {
        &self.states
    }

    /// Retourne l'alphabet de la machine
    pub fn get_alphabet(&self) -> &BTSet<Symbol> {
        &self.alphabet
    }

    /// Retourne les états finaux de la machine
    pub fn get_ends(&self) -> &BTSet<State> {
        &self.ends
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_fsm() {
        let link_file: &str = "src/automates/DFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let fsm : FSM = FSM::from_json_file(link_file);  
        //creation depuis du json
        let fsm2 : FSM = FSM::from_json(&content_json);
        //creation depuis new
        let fsm3 : FSM = FSM::new(fsm.get_states().clone(), fsm.get_alphabet().clone(), fsm.get_ends().clone());  
        
        assert_eq!(fsm.get_states(), fsm2.get_states());
        assert_eq!(fsm.get_ends(), fsm2.get_ends());
        assert_eq!(fsm.get_alphabet(), fsm2.get_alphabet());
        assert_eq!(fsm.get_states(), fsm3.get_states());
        assert_eq!(fsm.get_ends(), fsm3.get_ends());
        assert_eq!(fsm.get_alphabet(), fsm3.get_alphabet());
    }
}