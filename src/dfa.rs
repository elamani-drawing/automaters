use crate::{NonDeterministicFiniteAutomaton, AutomateTrait, AutomateJsonIO};

use super::{Transition, State,Symbol, FiniteStateMachine, BTSet};
use std::collections::{HashMap};
use std::fs;
use serde_json::{Value, from_str};

/// Automate a état fini déterministe
#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomaton {
    start: State,
    delta: HashMap<Transition<State>, State>,
    fsm: FiniteStateMachine, 
}

impl DeterministicFiniteAutomaton {    
    /// Créer un automate a état fini déterministe
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini décrivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let dfa2 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(&content_json);
    ///     let fsm: FiniteStateMachine = FiniteStateMachine::from_json(&content_json);
    ///     //creation depuis new
    ///     let dfa3 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::new(dfa.get_start().clone(), dfa.get_delta().clone(), fsm.clone());  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    pub fn new(_start : State, _delta : HashMap<Transition<State>, State>, _fsm : FiniteStateMachine) -> Self {
        DeterministicFiniteAutomaton{
            start : _start,
            delta : _delta,
            fsm: _fsm
        }
    }
    
    pub fn apply_delta(&self,transition : Transition<State>) -> Option<&State>{
        self.get_delta().get(&transition)
    }

    /// Réalise la transposition de l'automate
    /// 
    /// ```
    /// use automate::*;
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
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     let nfa : NonDeterministicFiniteAutomaton = dfa.to_transpose();
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - Un NonDeterministicFiniteAutomaton correspondant a la transposition de self
    /// 
    pub fn  to_transpose(&self) -> NonDeterministicFiniteAutomaton {
        let mut _delta: HashMap<Transition<State>, BTSet<State>> = HashMap::new();
        let mut _set : BTSet<State>;
        let mut _transition : Transition<State>;
        // creation des transitions de l'automate
        for (_transition_key, _transition_val) in self.get_delta() {
            // inversion des fleches
            _transition = Transition::new(_transition_key.get_symbol().clone(), _transition_val.clone());
            // si la transition existe deja, nous ajoutons l'image au Set, sinon nous creons une nouvel element cle:valeur
            if _delta.contains_key(&_transition) {
                // ajout d'un element dans le set
                _set  = _delta.get(&_transition).unwrap().clone();
            }else{
                // creation du set
                _set = BTSet::new();
            }
            _set.insert(_transition_key.get_content().clone());
            _delta.insert(_transition, _set);
        }            // inversion des etats initiaux et des etats finaux
        let mut _ends : BTSet<State> = BTSet::new();
        let mut _starts : BTSet<State>;
        _ends.insert(self.get_start().clone());
        _starts = self.get_ends().clone();
        // creation du nouvel automate NFA
        let _fsm : FiniteStateMachine = FiniteStateMachine::new(self.get_states().clone(), self.get_alphabet().clone(), _ends);
        NonDeterministicFiniteAutomaton::new(_starts, _delta, _fsm)
    }
    
    /// Renvoie la version minimize de l'automate 
    /// 
    /// ```
    /// use automate::*;
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
    ///     let mut dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     // minimisation
    ///     dfa = dfa.to_minimize();
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - Le DeterministicFiniteAutomaton apres avoir été minimizé
    /// 
    /// 
    pub fn to_minimize(&self) -> DeterministicFiniteAutomaton {
        // min(A) = det(det(At)t)
        let mut current_dfa : DeterministicFiniteAutomaton = self.clone();
        let mut current_nfa : NonDeterministicFiniteAutomaton;
        // minimalisation
        for _i in 0..2 {
            // realise la transposition
            current_nfa = current_dfa.to_transpose();
            // determinise la transpose
            current_dfa = current_nfa.to_dfa();
        }
        current_dfa
    }
}
impl AutomateJsonIO for DeterministicFiniteAutomaton{
    /// Créer un automate à état fini détérministe depuis un chemin du json
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini décrivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(&content_json);
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    fn from_json(content_json: &Value) -> Self {
        //creation du DFA à l'aide du content_json
        // reccupere le state de depart
        let state_init :State = State::new(content_json["start"].as_str().unwrap().to_string());
        // buffers
        let mut symbol: Symbol;
        let mut state: State;
        let mut image: State;
        let mut transition: Transition<State>;
        
        let mut alphabet: BTSet<Symbol> = BTSet::new();
        let mut states: BTSet<State> = BTSet::new();
        // reccuperation de delta
        let mut delta: HashMap<Transition<State>, State> = HashMap::new();
        let mut transition_json: &Value;
        for element_delta in content_json["delta"].as_array().unwrap(){
            transition_json = element_delta;
            symbol = Symbol::new(transition_json["symbol"].as_str().unwrap().to_string());
            state = State::new(transition_json["state"].as_str().unwrap().to_string());
            image = State::new(transition_json["image"].as_str().unwrap().to_string());
            // création de la transition: sur l'etat state, la lecture de state par symbol mene à image
            transition = Transition::new(symbol.clone(), state.clone());
            delta.insert(transition, image.clone());
            // on aurait pus juste construire delta et delaisser l'alphabet et le state à la FiniteStateMachine mais on en profite pour les construires en meme temps
            // ca nous fais economiser du temps et on sait que le contenu de l'alphabet et la liste des states apparait au moin une fois dans delta 
            states.insert(state);
            states.insert(image);
            alphabet.insert(symbol);
        }
        // on ajoute egalement le state de depart dans la liste des states
        states.insert(state_init.clone());
        // on reccupere les etats finaux
        let mut ends: BTSet<State> = BTSet::new();
        for elem in content_json["ends"].as_array().unwrap(){
            state = State::new(elem.as_str().unwrap().to_string());
            ends.insert(state.clone());
            // on ajoute les etats finaux a la liste des states
            states.insert(state);
        }
        
        //on aurait pus directement utiliser l'interfasse de FiniteStateMachine pour enumerer les etat, l'alphabet etc. mais par precaution on le fait mannuellement par apport au contenu des transitions
        //let fsm = FiniteStateMachine::from_json(&content_json);
        let fsm : FiniteStateMachine = FiniteStateMachine::new(states, alphabet, ends);
        DeterministicFiniteAutomaton { 
            start: state_init, 
            delta: delta, 
            fsm: fsm
        }
    }

    /// Créer un automate à état fini détérministe depuis un chemin vers un fichier json
    /// 
    /// # Arguments
    ///
    /// * `_start` - L'état initial de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transition de l'automate
    /// * `_fsm` - Une machine à état fini décrivant l'automate
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
	///     "start" : "q_0", 
	///     "delta" : [
	///     	{
	///     		"state" : "q_0",         
	///     		"symbol" : "a",         
	///     		"image" : "q_1"         
	///     	},         
	///     	{         
	///     		"state" : "q_1",         
	///     		"symbol" : "b",         
	///     		"image" : "q_0"        
	///     	}
	///     ] 
    /// }
    /// 
    /// ```
    /// 
    /// Le chargement dans le code
    /// 
    /// ```
    /// use automate::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/DFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `DeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        DeterministicFiniteAutomaton::from_json(&content_json)
    }
}

impl AutomateTrait<State> for DeterministicFiniteAutomaton{
    /// Retourne l'état de départ de l'automate
    fn get_start(&self) -> &State {
        &self.start
    }
    /// Aliases of self.get_start
    fn get_starts(&self) -> &State {
        self.get_start()
    }
    /// Retourne la machine de l'automate
    fn get_fsm(&self) -> &FiniteStateMachine {
        &self.fsm
    }
    
    /// Retourne les transitions de l'automate
    fn get_delta(&self) -> &HashMap<Transition<State>, State> {
        &self.delta
    }

    /// Retournes les differents états de l'automate
    fn get_states(&self) -> &BTSet<State> {
        &self.fsm.get_states()
    }

    /// Retourne l'alphabet de l'automate
    fn get_alphabet(&self) -> &BTSet<Symbol> {
        &self.fsm.get_alphabet()
    }

    /// Retourne les états finaux de l'automate
    fn get_ends(&self) -> &BTSet<State> {
        &self.fsm.get_ends()
    }
    
    /// indique si un mot est accepté dans la langue de l'automate
    fn accept(&self, _word : &str) -> bool {
        let mut symbol : Symbol;
        let mut state : &State = self.get_start();//etat de depart
        let mut transition : Transition<State>;
        for lettre in _word.chars() {
            symbol = Symbol::new(String::from(lettre));
            transition = Transition::new(symbol, state.clone());
            //execution de delta pour reccuperer l'image
            state = if let Some(image) = self.apply_delta(transition){
                image
            }else {
                return false;
            }
        }
        //si l'etat est dans la liste des etats finaux 
        self.get_ends().contains(state)
    }
    /// renvoie un clone de l'automate actuel puisqu'il est déjà determinist
    fn to_dfa(&self) -> DeterministicFiniteAutomaton{
        self.clone()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_clone_accept() {      
        let mut link_file: &str = "src/automates/DFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let mut dfa : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json_file(link_file);  
        //creation depuis du json
        let dfa2 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::from_json(&content_json);
        let fsm: FiniteStateMachine = FiniteStateMachine::from_json(&content_json);
        //creation depuis new
        let dfa3 : DeterministicFiniteAutomaton = DeterministicFiniteAutomaton::new(dfa.get_start().clone(), dfa.get_delta().clone(), fsm.clone());  

        assert_eq!(dfa.get_start(), dfa2.get_start());
        assert_eq!(dfa.get_ends(), dfa2.get_ends());
        assert_eq!(dfa.get_delta(), dfa2.get_delta());
        assert_eq!(dfa3.get_states(), dfa.get_states());
        assert_eq!(dfa3.get_ends(), dfa.get_ends());
        assert_eq!(dfa3.get_alphabet(), dfa.get_alphabet());
        
        assert_eq!(dfa.accept("aaab"), false);
        assert_eq!(dfa.accept("abab"), true);
        assert_eq!(dfa.accept(""), true);

        link_file = "src/automates/DFA2.json";
        //creation depuis un lien
        dfa = DeterministicFiniteAutomaton::from_json_file(link_file);  
        assert_eq!(dfa.accept("00011"), true);
        assert_eq!(dfa.accept("000"), false);

        link_file = "src/automates/DFA3.json";
        //creation depuis un lien
        dfa = DeterministicFiniteAutomaton::from_json_file(link_file);  
        assert_eq!(dfa.accept("b"), true);
        assert_eq!(dfa.accept("aaa"), false);
        assert_eq!(dfa.accept("bbababbb"), false);
        
        // transposition d'un DFA
        link_file = "src/automates/DFA1.json";
        //creation depuis un lien
        dfa = DeterministicFiniteAutomaton::from_json_file(link_file);  
        let nfa : NonDeterministicFiniteAutomaton  = dfa.to_transpose();
        
        let mut transition :Transition<State> = Transition::new(Symbol::new("b".to_string()),State::new("q_0".to_string()));
        let mut bt : BTSet<State> = BTSet::new();
        bt.insert(State::new("q_1".to_string()));
        assert_eq!(nfa.get_delta().get(&transition).unwrap().difference(bt.clone()).len(), 0);

        transition = Transition::new(Symbol::new("a".to_string()),State::new("q_1".to_string()));
        bt = BTSet::new();
        bt.insert(State::new("q_0".to_string()));
        assert_eq!(nfa.get_delta().get(&transition).unwrap().difference(bt.clone()).len(), 0);

        dfa.to_minimize();
    }
}