# AUTOMATERS
   
Implémentation d'automate en Rust. 

## Fonctionnalités     
Actuellement:
- [**State**](/src/state.rs): Un état de l'automate.
- [**Symbol**](/src/symbol.rs): Un symbole de l'automate.
- [**Transition**](/src/transition.rs):  Une transition de l'automate.
- [**FSM**](/src/fsm.rs): Une machine a état fini.
- [**DFA**](/src/dfa.rs):  Un automate déterministe à état fini.
- [**NDFA**](/src/nfa.rs):  Un automate non déterministe à état fini.
- [**NDFAEpsilon**](/src/nfae.rs):  Un automate non déterministe a état fini avec epsilon clausure.

La documentation est disponnible [ici.](https://docs.rs/automaters/0.1.0/automaters/)

## Installation
Pour utiliser Morseus dans votre projet, ajoutez la dépendance suivante à votre fichier Cargo.toml :
```toml
[dependencies]
automaters = "0.1.0"
```

## Contributions
Les contributions sont les bienvenues! Si vous souhaitez améliorer automaters, veuillez ouvrir une pull request sur GitHub.

## License
Ce projet est sous [``licence MIT``](LICENSE). Veuillez consulter le fichier [``LICENSE``](LICENSE) pour plus d'informations.