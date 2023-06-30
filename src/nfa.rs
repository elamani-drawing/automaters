use crate::interfaces::AutomateJsonIO;
use crate::{DeterministicFiniteAutomaton, AutomateTrait};

use super::{Transition, State,Symbol, FiniteStateMachine, BTSet};
use std::collections::{HashMap};
use std::{fs};
use serde_json::{Value, from_str};

/// Automate a état fini déterministe
#[derive(Debug, Clone)]
pub struct NonDeterministicFiniteAutomaton {
    starts: BTSet<State>,
    delta: HashMap<Transition<State>, BTSet<State>>,
    fsm: FiniteStateMachine, 
}

impl NonDeterministicFiniteAutomaton {    
    /// Créer un automate a état fini non déterministe
    /// 
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état fini
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
	///     ] 
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
    /// 
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     //creation depuis du json
    ///     let nfa2 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(&content_json);
    ///     let fsm: FiniteStateMachine = FiniteStateMachine::from_json(&content_json);
    ///     //creation depuis new
    ///     let nfa3 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::new(nfa.get_starts().clone(), nfa.get_delta().clone(), fsm.clone());  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate non déterministe à état fini correspondante
    /// 
    pub fn new(_starts : BTSet<State>, _delta : HashMap<Transition<State>, BTSet<State>>, _fsm : FiniteStateMachine) -> Self {
        NonDeterministicFiniteAutomaton{
            starts : _starts,
            delta : _delta,
            fsm: _fsm
        }
    }
      /// Applique une transition et renvoie un set d'etat (representant l'image de la transition)
      pub fn apply_delta(&self, transition : Transition<State>)-> Option<BTSet<State>>{
        if let Some(n) = self.get_delta().get(&transition) {
            return Some(n.clone());
        }
        return None;
    }

    /// Applique les transition et renvoie un set d'etat (representant l'image de la transition)
    pub fn apply_deltas(&self,set_transition : Transition<BTSet<State>>) -> Option<BTSet<State>>{
        let mut images : BTSet<State> = BTSet::new();
        let mut current: Option<BTSet<State>>;
        let mut transition : Transition<State>;
        let symbol :Symbol = set_transition.get_symbol().clone();
        for state in set_transition.get_content().clone().get(){
            transition = Transition::new(symbol.clone(), state.clone());
            current = self.apply_delta(transition);
            if current != None {
                images.insert_all(current.unwrap());
            }else{
            }
        }
        if images.is_empty() {
            return None;
        }
        return Some(images);
    }

}
impl AutomateJsonIO for NonDeterministicFiniteAutomaton{
    /// Créer un automate à état fini non détérministe depuis un chemin du json
    ///   
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état fini
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
	///     ] 
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
    /// 
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis du json
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(&content_json);
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate non déterministe à état fini correspondante
    /// 
    fn from_json(content_json: &Value) -> Self {
        //creation du NFA à l'aide du content_json
        let mut symbol: Symbol;
        let mut state: State;
        let mut image: State;
        let mut transition: Transition<State>;
        
        let mut alphabet: BTSet<Symbol> = BTSet::new();
        let mut states: BTSet<State> = BTSet::new();
        // réccuperation des states de départs
        let mut starts : BTSet<State> = BTSet::new();
        for start in content_json["starts"].as_array().unwrap(){
            state = State::new(start.as_str().unwrap().to_string());
            starts.insert(state.clone());
            states.insert(state);
        }
        // réccuperation du delta
        let mut delta: HashMap<Transition<State>, BTSet<State>> = HashMap::new();
        let mut transition_json: &Value;
        let mut images : BTSet<State> ;

        for element_delta in content_json["delta"].as_array().unwrap(){
            transition_json = element_delta;
            symbol = Symbol::new(transition_json["symbol"].as_str().unwrap().to_string());
            state = State::new(transition_json["state"].as_str().unwrap().to_string());
            // generation des images du state
            images = BTSet::new();
            for img in transition_json["images"].as_array().unwrap(){
                image = State::new(img.as_str().unwrap().to_string());
                states.insert(image.clone());
                images.insert(image);
            }

            transition = Transition::new(symbol.clone(), state.clone()); //création de la transition: sur l'etat state, la lecture de state par symbol mene à un set d'images
            delta.insert(transition, images.clone());
            
            states.insert(state);
            alphabet.insert(symbol);
        }
        // reccuperation des etats finaux
        let mut ends: BTSet<State> = BTSet::new();
        for elem in content_json["ends"].as_array().unwrap(){
            state = State::new(elem.as_str().unwrap().to_string());
            ends.insert(state.clone());
            states.insert(state);
        }
        
        //on aurait pus directement utiliser l'interfasse de FiniteStateMachine pour enumerer les etat, l'alphabet etc. mais par precaution on le fait mannuellement par apport au contenu des transitions
        //let fsm = FiniteStateMachine::from_json(content_json);
        let fsm : FiniteStateMachine = FiniteStateMachine::new(states, alphabet, ends);
        NonDeterministicFiniteAutomaton { 
            starts: starts, 
            delta: delta, 
            fsm: fsm
        }
    }

    /// Créer un automate à état fini détérministe depuis un chemin vers un fichier json
    ///  
    /// # Arguments
    ///
    /// * `_starts` - Les états initiaux de l'automate
    /// * `_delta` - Une HashMap decrivant les differentes transitions de l'automate
    /// * `_fsm` - Une machine à état plusieur état fini décrivant l'automate
    ///
    /// # Examples
    /// 
    /// Le contenu du json
    /// 
    /// ```json
    /// {
    ///     "states" : ["s","t", "u", "v"],
    ///     "alphabet" :  ["b","a"],
    ///     "ends" : ["q_0"],
	///     "starts" : ["s"], 
	///     "delta" : [
	///             {
    ///              "state" : "s",         
    ///              "symbol" : "a",         
    ///              "images" : ["s", "t", "u", "v"]        
    ///             },         
    ///             {
    ///              "state" : "t",         
    ///              "symbol" : "b",         
    ///              "images" : ["s"]        
    ///             }, 
    ///             {
    ///              "state" : "u",         
    ///              "symbol" : "b",         
    ///              "images" : ["t"]        
    ///             }, 
    ///             {
    ///              "state" : "v",         
    ///              "symbol" : "b",         
    ///              "images" : ["u"]        
    ///             }
	///     ] 
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
    /// 
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
    /// 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate déterministe à état fini correspondante
    /// 
    fn from_json_file(path: &str) -> Self {
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(path).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation de la machine
        NonDeterministicFiniteAutomaton::from_json(&content_json)
    }
}

impl AutomateTrait<BTSet<State>> for NonDeterministicFiniteAutomaton{
    /// Retourne les états de départ de l'automate
    fn get_starts(&self) -> &BTSet<State> {
        &self.starts
    }
    /// Aliases de self.get_starts
    fn get_start(&self) -> &BTSet<State> {
        &self.starts
    }
    /// Retourne la machine de l'automate
    fn get_fsm(&self) -> &FiniteStateMachine {
        &self.fsm
    }
    
    /// Retourne les transitions de l'automate
    fn get_delta(&self) -> &HashMap<Transition<State>, BTSet<State>> {
        &self.delta
    }

    /// Retournes les differents états de l'automate
    fn get_states(&self) -> &BTSet<State> {
        self.fsm.get_states()
    }

    /// Retourne l'alphabet de l'automate
    fn get_alphabet(&self) -> &BTSet<Symbol> {
        self.fsm.get_alphabet()
    }

    /// Retourne les états finaux de l'automate
    fn get_ends(&self) -> &BTSet<State> {
        self.fsm.get_ends()
    }
    
    /// indique si un mot est accepté dans la langue de l'automate
    fn accept(&self, _word : &str) -> bool {
        let mut symbol : Symbol;
        let mut currents : BTSet<State> = self.get_starts().clone();//etats de depart
        let mut transition : Transition<BTSet<State>>;
        let mut temp : Option<BTSet<State>> ;
        for lettre in _word.chars() {
            symbol = Symbol::new(String::from(lettre));
            transition = Transition::new(symbol, currents.clone());
            //execution de delta pour reccuperer l'image
            temp =self.apply_deltas(transition);
            if temp==None {
                //si aucune image n'a ete trouver, ca ne sert à rien de poursuitre
                return false;
            }
            currents =temp.unwrap();
        }
        for state in currents.get(){
            //si on trouve un etat qui fait parti des etats finaux de l'automate, on valide le mot
            if self.get_ends().contains(&state){
                return true;
            }
        }
        //aucun des etats de currents ne fait parti des etats finaux
        return false;
    }
    
    /// Convertit le NFA en DFA
    /// 
    /// ```
    /// use automaters::*;
    /// use std::fs;
    /// use serde_json::{Value, from_str};
    /// fn main() {
    /// 
    ///     let link_file: &str = "src/automates/NFA1.json";
    ///     let content_json: Value = {
    ///         // Charge le contenu du fichier en tant que String
    ///         let content : String = fs::read_to_string(link_file).unwrap();
    ///         // Parse le texte en structure Json
    ///         from_str::<Value>(&content).unwrap()
    ///     };
    ///     //creation depuis un lien
    ///     let nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
    ///     dbg!(nfa.to_dfa());
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `NonDeterministicFiniteAutomaton` - L'automate déterministe à état fini qui correspondante
    /// 
    fn to_dfa(&self) -> DeterministicFiniteAutomaton {
        // Un set des images que renvoie une transition
        let mut state_image : BTSet<State>;
        let _alphabet :BTSet<Symbol>  = self.get_alphabet().clone();
        let mut transition :Transition<BTSet<State>>;
        // les nouveaux states qui seront les states du nouvel automate
        let mut new_states : BTSet<BTSet<State>> = BTSet::new();
        let mut temp :Option<BTSet<State>>;
        // un set de state dont on ne connait pas les images
        let mut set_state_search_image : BTSet<BTSet<State>> =  BTSet::new();
        // la table de transition regroupant l'ensemble des transitions du nouvel automate, equivalent à delta
        let mut table_de_transition : HashMap<Transition<BTSet<State>>, BTSet<State>> = HashMap::new();
        // HashMap<Transition<State>, BTSet<State>> 
        let mut set_state_image : BTSet<BTSet<State>> = BTSet::new();
        // le set de states de departs de self sera le state de depart du nouvel automate
        let first_state : BTSet<State> = self.get_starts().clone();
        // ajoute le premier element dans les images
        new_states.insert(first_state.clone()); 
        set_state_search_image.insert(first_state.clone()); 
        let mut continuer : bool = true;
        // calculs des nouveaux etats, transitions, images
        while continuer {
            // parcour l'ensemble des states et des symboles pour creer des transitions et calculer les images
            for state in set_state_search_image.get() {
                for letter in _alphabet.get() {
                    // creation de la transition
                    transition = Transition::new(letter.clone(),state.clone());
                    // reccuperation de l'image
                    temp = self.apply_deltas(transition.clone());
                    if temp!=None {
                        // reccuperation du contenu
                        state_image =temp.unwrap();
                        // sauvegarde de la transition
                        table_de_transition.insert(transition, state_image.clone());
                        // on enregistre le state, plutard on pourra verifier si on le connaissais deja ou pas (si on ne le connaissais pas on l'ajoute dans set_state_search_image pour rechercher ses images au prochain tour)
                        set_state_image.insert(state_image);
                    }
                    // si aucune image n'a ete trouver, on ignore et passe au tour suivant
                }
            }
            // prepare la liste des states dont on ne connait pas les images
            set_state_search_image =  BTSet::new();
            for state in set_state_image.get(){
                // si le state est inconnu
                if !new_states.contains(state){
                    // on l'enregistre et on cherche ses transitions/images aux prochains tour
                    set_state_search_image.insert(state.clone());
                    new_states.insert(state.clone());
                }
            }
            // La liste de state dont on ne connait les images et transition est vide, donc on peut sarreter
            if set_state_search_image.len() == 0 {
                continuer = false;
            }
        }
        let name : String = "q_".to_string();
        let mut _states : BTSet<State> = BTSet::new();
        // listes des etats finaux
        let mut _ends : BTSet<State> = BTSet::new();
        let mut _deltas : HashMap<Transition<State>, State> = HashMap::new();
        // sauvegarde le nom de state de chaque BTSet<state>
        let mut _concordances  : HashMap<BTSet<State>, State> = HashMap::new();
        let mut i : usize = 0;
        let mut _state : State;
        // creations des states de l'automate
        for state in new_states.get() {
            // buffer = name+&i.to_string();
            _state = State::new( name.clone()+&i.to_string());
            _states.insert(_state.clone());
            _concordances.insert(state.clone(), _state.clone());
            i+=1;
        }
        let ends = self.get_ends().get();
        // buffer temporaire
        let mut symbol : Symbol ;
        let mut content : &BTSet<State> ;
        let mut value :  BTSet<State> ;
        // creation des transitions de l'automate
        for (_transition_key, _transition_val) in table_de_transition {
            symbol = _transition_key.get_symbol().clone();
            content = &_transition_key.get_content();
            value = _transition_val;
            _deltas.insert(Transition::new(symbol, _concordances.get(content).unwrap().clone()), _concordances.get(&value).unwrap().clone());
            
            //on parcour les etats finaux de l'automate NFA pour savoir si le state actuel est finaux ou pas
            for _state in ends {
                if content.contains(_state){
                    _ends.insert(_concordances.get(content).unwrap().clone());
                }
            }
        }
        let _fsm :FiniteStateMachine = FiniteStateMachine::new(_states, _alphabet, _ends);
        // création du DFA
        DeterministicFiniteAutomaton::new(_concordances.get(&first_state).unwrap().clone(), _deltas, _fsm)
    }
}
  


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_partial_eq_clone_accept() {      
        let mut link_file: &str = "src/automates/NFA1.json";
        let content_json: Value = {
            // Charge le contenu du fichier en tant que String
            let content : String = fs::read_to_string(link_file).unwrap();
            // Parse le texte en structure Json
            from_str::<Value>(&content).unwrap()
        };
        //creation depuis un lien
        let mut nfa : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
        //creation depuis du json
        let nfa2 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::from_json(&content_json);
        let fsm: FiniteStateMachine = FiniteStateMachine::from_json(&content_json);
        //creation depuis new
        let nfa3 : NonDeterministicFiniteAutomaton = NonDeterministicFiniteAutomaton::new(nfa.get_starts().clone(), nfa.get_delta().clone(), fsm.clone());  

        assert_eq!(nfa.get_starts(), nfa2.get_starts());
        assert_eq!(nfa.get_ends(), nfa2.get_ends());
        assert_eq!(nfa.get_delta(), nfa2.get_delta());
        assert_eq!(nfa3.get_states(), nfa.get_states());
        assert_eq!(nfa3.get_ends(), nfa.get_ends());
        assert_eq!(nfa3.get_alphabet(), nfa.get_alphabet());

        assert_eq!(nfa.accept("abbbb"), false);
        assert_eq!(nfa.accept("b"), false);
        assert_eq!(nfa.accept("aabb"), true);

        link_file = "src/automates/NFA2.json";
        //creation depuis un lien
        nfa = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
        assert_eq!(nfa.accept("00001"), true);
        assert_eq!(nfa.accept("0000000"), false);
        assert_eq!(nfa.accept("01"), false);

        link_file = "src/automates/NFA3.json";
        //creation depuis un lien
        nfa = NonDeterministicFiniteAutomaton::from_json_file(link_file);  
        assert_eq!(nfa.accept("bbaaaba"), true);
        assert_eq!(nfa.accept("abbaab"), false);
    }
}