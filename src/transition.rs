use super::Symbol;

/// Une transition
#[derive(Debug, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Transition<T: Clone> {
    symbol: Symbol,
    content: T
}

impl<T: Clone> Transition<T> {
    /// Créer une Transition
    ///
    /// # Argument
    ///
    /// * `_symbol` - Le symbole de Self
    /// * `_content` - Le contenu de Self
    ///
    /// # Examples
    /// 
    /// Créer une transition avec un set de States
    /// ```
    /// use automaters::*;
    /// use std::collections::HashSet;
    /// fn main() {
    ///     //un set d'etat
    ///     let mut hash_states : HashSet<State> = HashSet::new();
    ///     //crée un symbole
    ///     let symbole : Symbol = Symbol::new(String::from("o"));
    ///     //crée des etats
    ///     let state_1 : State = State::new(String::from("state_1"));
    ///     let state_2 : State = State::new(String::from("state_1"));
    ///     let state_3 : State = State::new(String::from("state_3"));
    ///     //ajoutes les etats dans le set
    ///     hash_states.insert(state_1);
    ///     hash_states.insert(state_2);
    ///     hash_states.insert(state_3);
    ///     //création d'une transition
    ///     let transition_states : Transition<HashSet<State>> = Transition::new(symbole, hash_states);
    ///     dbg!(transition_states); 
    /// 
    /// }
    /// ```
    /// 
    /// Créer une transition avec un state
    /// ```
    /// use automaters::*;
    /// fn main() {
    ///     //crée un symbole
    ///     let symbole : Symbol = Symbol::new(String::from("o"));
    ///     //crée des etats
    ///     let state_1 : State = State::new(String::from("state_1"));
    ///     //ajoutes les etats dans le set
    ///     //création d'une transition
    ///     let transition_states : Transition<State> = Transition::new(symbole, state_1);
    ///     dbg!(transition_states); 
    /// 
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `Transition<T>` - La Transition qui à été créer
    ///
    pub fn new(_symbol : Symbol, _content : T) -> Self {
        Transition { symbol: _symbol, content: _content}
    }

    /// Retourne le Symbol de Self
    ///
    /// # Example
    ///
    /// ```
    /// use automaters::*;
    /// fn main() {
    ///     let symbole : Symbol = Symbol::new(String::from("o"));
    ///     //crée des etats
    ///     let state_1 : State = State::new(String::from("state_1"));
    ///     //ajoutes les etats dans le set
    ///     //création d'une transition
    ///     let transition_states : Transition<State> = Transition::new(symbole, state_1);
    ///     dbg!(transition_states.get_symbol()); 
    /// }
    /// ```
    ///
    /// # Return
    ///
    /// * `&Symbol` - Le symbol de selfs
    ///
    pub fn get_symbol(&self) -> &Symbol {
        &self.symbol
    }
    /// Retourne le contenu de Self
    ///
    /// # Example
    ///
    /// ```
    /// use automaters::*;
    /// fn main() {
    ///     let symbole : Symbol = Symbol::new(String::from("o"));
    ///     //crée des etats
    ///     let state_1 : State = State::new(String::from("state_1"));
    ///     //ajoutes les etats dans le set
    ///     //création d'une transition
    ///     let transition_states : Transition<State> = Transition::new(symbole, state_1);
    ///     dbg!(transition_states.get_content()); 
    /// }
    /// ```
    /// 
    /// # Return
    ///
    /// * `&T` - Le contenu de self
    ///
    pub fn get_content(&self) -> &T {
        &self.content
    }
}

impl<T> PartialEq for Transition<T>
    where
        T: PartialEq,
        T: Clone
    {
    fn eq(&self, other: &Self) -> bool {
        self.get_symbol() == other.get_symbol() && self.get_content() == other.get_content()
    }
    fn ne(&self, other: &Self) -> bool {
        self.get_symbol() != other.get_symbol() || self.get_content() != other.get_content()
    }
}

#[cfg(test)]
mod test {
    use super::super::{State, Symbol, Transition};
    use std::collections::HashSet;

    #[test]
    fn creation_partial_eq_copy() {
        //un set d'etat
        let mut hash_states : HashSet<State> = HashSet::new();
        //crée un symbole
        let symbole : Symbol = Symbol::new(String::from("o"));
        //crée des etats
        let state_1 : State = State::new(String::from("state_1"));
        let state_2 : State = State::new(String::from("state_1"));
        let state_3 : State = State::new(String::from("state_3"));
        //ajoutes les etats dans le set
        hash_states.insert(state_1.clone());
        hash_states.insert(state_2.clone());
        hash_states.insert(state_3.clone());
        //créer une transition
        let transition_states : Transition<HashSet<State>> = Transition::new(symbole.clone(), hash_states.clone());
        // verifie le contenu
        assert_eq!(transition_states.get_symbol(), &symbole);
        assert_eq!(transition_states.get_content(), &hash_states);
    }
}