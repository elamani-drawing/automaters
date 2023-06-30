use std::{collections::{BTreeSet, btree_set::Difference}, hash::Hash};

/// Une abstraction d'un HashSet realiser avec un BTreeSet
#[derive(Debug, Clone, Hash ,Eq,PartialEq, Ord, PartialOrd)]
pub struct BTSet<T : Clone+ Eq +PartialEq+ Ord+ PartialOrd> {
    set: BTreeSet<T>,
}

impl<T : Clone+ Eq +PartialEq+ Ord+ PartialOrd + Hash> BTSet<T> {  
    pub fn new() -> Self{
        let mut _set : BTreeSet<T> = BTreeSet::new();
        BTSet{
            set : _set
        }
    }
    
    pub fn from_vect(&self, v: Vec<T>) -> BTSet<T> {
        let mut btset :BTSet<T>= BTSet::new();
        // recopie de l'element
        for elem in v {
            btset.insert(elem);
        }
        return btset;
    }

    // liaison entre l'interface de BTreeSet et BTSet
    pub fn get(&self) -> &BTreeSet<T>{
        &self.set 
    }

    pub fn insert(&mut self, value : T) -> bool{
        self.set.insert(value)
    }
    
    pub fn contains(&self, value : &T) -> bool{
        self.set.contains(value)
    }

    pub fn is_empty(&self) -> bool{
        self.set.is_empty()
    }

    // renvoie la difference entre self et other
    pub fn difference<'a>(&self, other :BTSet<T>) -> BTSet<T>{
        let _other:&BTreeSet<T> = &other.get(); // &other.get().clone();
        let _difference:Difference<T> = self.set.difference(_other); 
        let _vect : Vec<T>= _difference.cloned().collect();
        // make new BTSet
        self.from_vect(_vect)
    }
    
    pub fn len(&self) -> usize{
        self.set.len()
    }

    // clone tout les elements de new dans self
    pub fn insert_all(&mut self, new: BTSet<T>)-> bool{
        for state in new.get(){
            self.set.insert(state.clone());
        }
        true
    }
}
