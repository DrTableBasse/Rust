# Write Up

## Exercice 00_Intro

### Intro 2
Changer le `printline!` en `println!` pour corriger l'erreur

```rust
fn main() {
    println!("Hello there!");
}
```

## Exercice 01_Variables

### Variables 6

Avec une constante, on est obligé de typer les variables. C'est pas comme en python...

```rust
const NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
```

## Exercice 02_Functions

### Functions 5

Pour retourner une valeur, on a deux possibilité :
- `return num*num;`
- `num*num` \
les deux font la même chose...

```rust
fn square(num: i32) ->  i32 {
    num * num
}
```
## Exercice 03_If

### If 3

Ici, on avait une sorte de switch case de faire dans la fonction public `animal_habiat` (ligne 8). Dans `let identifier` il est obligatoire de rajouter un 0 pour la quatrième solution (la valeur de défaut / sortie).

```rust
pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0 
    };
```

## Exercice 04_Primitives_Types

### Primitive Types 6

Pour accéder à un index du tuple `numbers`, il suffit d'ajouter le nom du tuple `.` le numéro de l'index. \
Exemple `NomDuTuple.<valeur>`

```rust
#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
```

## Exercice 05_Vecs

### Vec 2

Pour multiplier un vecteur par 2, il faut mettre un pointeur sur la variable `element`. Ca nous permettra de différencier la valeur stockée à celle de l'adresse mémoire.

```rust
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
```

## Exercice 06_Move_Semantics

### Move Semantics 6

Il faut clonner la partie data pour ensuite la manipuler comme on le souhaite. Sinon ce n'est juste pas possible et ça ne compile pas 😊

```rust
fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}
```

## Exercice 07_Structs

### Structs 3

Dans la première fonction `is_international`, on veut pouvoir retourner un booléen si le `sender_coountry` est différent de `recipient_country` (grosso modo ça retourne soit un 1 si c'est vrai ou un 0 si c'est faux)

```rust
    //Just a boolen function to verify if the sender and recipient countries are different
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }
```

Dans la seconde fonction `get_fees`, on veut juste calculer les frais en fonction du poids du colis. 

```rust
    //Function to calculate the fees based on the weight of the package and the cents per gram
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        self.weight_in_grams * cents_per_gram
    }
}
```

## Exercice 08_Enums

### Enums 3

Dans la partie `enum Message{..}`, il suffit d'énumérer les différentes State qui nous sont présentés dans le code.

```rust
enum Message {
    ChangeColor(u8, u8, u8),
    Quit,
    Echo(String),
    Move(Point),
}
```
Pour la seconde fonction `process`, il suffit clairement de suivre ce qui est demandé en se liant à `enum Message{..}` vu plus haut. \
On a le résultat suivant : 

```rust
    //Refer to impl State for the struct State. So just add which type of State we have (Color = 3 bytes of u8, Quit = bool, Echo = String, Move = Point)
    //Basically we want to save the message in the State struct and then we can print it out in the test
    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            Message::Quit => self.quit(),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
        }
    }
```

## Exercice 09_Strings

### Strings 4

De ce que j'ai pu comprendre avec mes quelques heures de Rust, à partir du moment où on souhaite modifier une chaîne de caractère, il faut forcément utiliser `string()`. Sinon, `println!`suffit pour les chaînes de caractères non mutable.

```rust
fn main() {
    println!("blue");
    string("red".to_string()); 
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string(String::from("abc")[0..1].to_string());
    string("  hello there ".trim().to_string());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```

## Exercice 10_Modules

### Modules 3
Ici, juste l'import de `SystemTime` et `UNIX_EPOCH` du module `sdt::time` permettent d'afficher correctement le temps en secondes depuis le 1 janvier 1970.

```rust
// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH}; 
//Import the SystemTime and UNIX_EPOCH from the std::time module
//Btw the compiler said to import the SystemTime and UNIX_EPOCH from the std::time module

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```


## Exercice 11_Hashmaps

### Hashmaps 3
Ici, on va utiliser des hashmaps qui sont grosso modo des dictionnaires. Le problème ici c'est qu'on ne réactualisait pas les points de chaques équipes à chaque buts. Donc pour résoudre ceci, j'ai tout simplement ajouter à chaque team le score qu'ils ont à chaque nouveau but.
Bien sûr on oublie pas de cloner à chaque fois pour pas être embêté avec les équipes (`team_1/2_name`)

```rust
fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        
        // Update goals scored and conceded for team 1
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // Update goals scored and conceded for team 2
        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team { goals_scored: 0, goals_conceded: 0 });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}
```

## Exercices 12_Options

### Options 3

À l'origine, nous avons le code suivant : 

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
     y; // Fix without deleting this line.
}
```
Cependant, à la compilation, la ligne 19 nous envoie boulet et ne compile pas. Pour résoudre ce problème, il suffit de rajouter un `let _ = ` pour que le code compile.\
On peut se demander ce que le `_` fait ici. C'est une variable par défaut, Rust ne se souci donc plus de cette dernière et n'émet pas des erreurs de compilations. Ca nous sert dans les cas où la variable n'est pas utilisée comme ici après le match.

Voici le code final : 
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    let _ = y; // Fix without deleting this line.
}
```

## Exercice 13_Error_Handling

### Errors 6

Ici l'objectif est d'avoir autre chose qu'un panick lors de la compilation. Pour se faire, il va falloir afficher les erreurs que retournent les différentes fonctions `from_creation`et `from_parseint`.\
Donc voici les deux fonctions que j'ai créé pour convertir une erreur liée au `ParseIntError` en `ParsePosNonzeroError`

```rust
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
    //On ajoute juste une fonction supplémentaire pour convertir une erreur liée au 
    //ParseIntError en ParsePosNonzeroError.
}
fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?; //Horrible cette ligne 💀
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}
//On utilise la fonction map_err pour convertir l'erreur liée au ParseIntError en ParsePosNonzeroError.
```

## Exercice 14_Generics
Maintenant partie marrante que sont les Wrappers. L'objectif ici va être de modifier la structure du Wrapper en une structure générique. Le fait d'inclure `<T>`permet de contenir n'importe quel type de valeur (str,int,float...) donc ça a un intérêt vraiment conséquent.\
Le code parle de lui même : 

```rust
struct Wrapper<T> { //On transforme la structure Wrapper en une structure générique.
                    //Avec le <T> pour contenirs des valeurs de n'importe quel type. (int, str, etc...)
    value: T,
}

impl<T> Wrapper<T> { // On modifie en conséquence les fonctions d'implémentation de la structure Wrapper pour coller avec la structure.
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
```
## Exercice 15_Traits

### Traits 5

Je n'ai pas trouvé de façon plus propre qu'en ajoutant d'autre ligne de code pour ajouter `SomeTrait` et `OtherTrait` pour pas avoir de plantage. Donc concrètement ici, je ne sais pas si il y a une meilleure façon de faire ou non...\
Ce que je peux dire, en revanche, c'est que la foncitone `some_func` prend en paramètres générique, `<T>` et retourne un booléen

```rust
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait, // Trait bound requiring T to implement SomeTrait and OtherTrait
{
    item.some_function() && item.other_function()
}
```

## Exercice 16_LifeTime

### Lifetime 3
Chose très marrante ici car les lifetimes (durée de vie) parle littéralement de variables qui meurent dans le programme *insérer rire diabolique 😈*\
Concrètement, une variable qui est dans un scope a une durée de vie propre au scope (une fonction est un scope par exemple). Une fois sortie de ce dernier, impossible de savoir pour Rust l'état de la variable ni même de sa valeur.\
Heureusement, on peut prolonger sa durée de vie avec l'exemple suivant : 
```rust
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
```

## Exercice 17_Tests

### Test 4 
Partie un peu obscure que sont les tests mais pour faire une briève explications, chaque fonctions peuvent être soumises à des tests unitaire (?) pour voir le bon fonctionnement du programme suivant les différentes entrées.\
Le but ici est de tests la fonction `correct_width_and_height`.

```rust
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }
}
```

## Exercice 18_Iterators

### Iterators 5
Le but d'un iterateur (iterator) est de parcourir une liste de données donner de façon séquentielle. C'est assez bien à utilisé lorsqu'on a des données à traiter comme ceux d'un tableau par exemple.

Voici les deux fonctions que j'ai dû traiter pour que le code compile totalement : 

```rust
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.values().filter(|&&val| val == value).count()
}
```
```rust
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    collection.iter().map(|map| count_iterator(map, value)).sum()
}
```

## Exercice 19_Smart_Pointers

### RC 1

*Quel enfers ces pointeurs, ils nous suivront toute notre vie...*\
Pour faire une rapide explication, un pointeur permet de pointer (whoua merci Sam) une valeur vers une adresse.  J'ai pas trop d'explications à donner pour cet exercice hormis de relire en boucle le code et débugger petit à petit car trop *chiant*

```rust
#[test]
fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
```

## Exercice 20_Thread

### Threads 3

Un thread est un composant logique qui permet de faire un calcul (code exécuté, jeux lancé etc..). Certains programmes peuvent faire du multithreading en faisant plusieurs calculs sur différents threads. Le problème est que si on ne gère pas les variables sur plusieurs threads, cela peut causer divers problèmes. Notamment un programme qui compile pas.\
C'est pour cela qu'après le débuggage j'ai juste décidé de faire des unwraps pour attendre que chaque thread finisse son calcul sinon *program goes boom boom*

```rust
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let tx1 = tx.clone(); // Clone the sender for the first closure
    let first_half_handle = thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone(); // Clone the sender for the second closure
    let second_half_handle = thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    first_half_handle.join().unwrap();
    second_half_handle.join().unwrap();
}
```

## Exercice 21_Macros

### Macros 4

Vraiment, RTFM pour comprendre ce qu'est une macro, le débuggage est pas compliqué donc je mettrais juste un avant-après

#### Avant :
```rust
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
```

#### Après : 
```rust
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
```
*si tu as trouvé l'erreur, mets une étoile au répo !*

## Exercice 22_Clippy 📎

### Clippy 3

Clippy en rust est un utilitaire que l'on peut directement lancé dans le terminal dans le répertoire que l'on souhaite. Il va nous aider à débugger d'avantage le code si le compilateur (cargo) ne dit rien d'intéressant.
Pour le lancer il suffit de faire : 
```bash
cargo clippy
```
Voici le type d'erreur qu'il a soulevé et m'a aidé à corriger :
```rust

warning: this let-binding has unit value
  --> clippy3.rs:23:5
23 |     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `vec![1, 2, 3, 4, 5].resize(0, 5);`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_unit_value
   = note: `#[warn(clippy::let_unit_value)]` on by default

warning: `clippy3` (bin "clippy3") generated 1 warning (run `cargo clippy --fix --bin "clippy3"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```
Enfin bref l'outil est vraiment intéressant et permet vraiment un débuggage un peu plus en profondeur donc voici tout de même la correction avec une explication :

---
Ici, en suivant l'instruction et en regardant ce que clippy nous disait, on pouvait corriger le problème qui n'en était pas vraiment un. Juste des virgules qui manquaient dans le tableau. Après un peu de magie noire, voici le avant-après (comme le code n'est pas trop long je me permets)

#### Avant : 
```rust
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    value_a = value_b;
    value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
```
#### Après : 
```rust
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
    };

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    vec![1, 2, 3, 4, 5].resize(0, 5);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); 
    println!("value a: {}; value b: {}", value_a, value_b);
}
```

## Exercice 23_Conversions

### Using_As

Rien de bien compliqué ici. L'exercice nous demande juste de corriger le code pour éviter le problème à la compilation. Un simple `as f64` résoud la chose (à voir pour remplacer par f32 voir plus petit si besoin est)
Voici le avant-après (comme le code n'est pas trop long je me permets)

#### Avant : 
```rust
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() //Ici qu'il faut regarder
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
```
#### Après : 
```rust
///J'ai ajouté  `as f64` pour que le résultat de la division soit un f64 vu que la fonction déjà retourne un f64 MAIS qu'en plus on fait une division avec des nombres à virgules.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64 //Ici qu'il faut regarder
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
```

# Quiz

<p align="center">
  <img src="tenor.gif" alt="alt text">
</p>


## Quiz 1

J'ai pas d'explication à donné, suivez ce qui est marqué et faites...

#### Avant : 
```rust

// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
```
#### Après : 
```rust
    //I think the function is logicall...
    fn calculate_price_of_apples(quantity: u32) -> u32 {
        if quantity <= 40 {
                quantity * 2
        } else {
                quantity
        }
    }


    // Don't modify this function!
    #[test]
    fn verify_test() {
        let price1 = calculate_price_of_apples(35);
        let price2 = calculate_price_of_apples(40);
        let price3 = calculate_price_of_apples(41);
        let price4 = calculate_price_of_apples(65);

        assert_eq!(70, price1);
        assert_eq!(80, price2);
        assert_eq!(41, price3);
        assert_eq!(65, price4);
    }
```

## Quiz 2

Encore une fois, lire et faire ce qui est demandé...

#### Avant : 
```rust
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: ???) -> ??? {
        // TODO: Complete the output declaration!
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
```

#### Après : 
```rust
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // Ajout d'un match pour déterminer quelle action appliquer à la chaîne de caractères
            match command {
                Command::Uppercase => {
                    // Appliquer la commande Uppercase
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    // Appliquer la commande Trim
                    output.push(string.trim().to_string());
                }
                Command::Append(n) => {
                    // Appliquer la commande Append
                    let appended_string = string.clone() + &"bar".repeat(*n);
                    output.push(appended_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
```

## Quiz 3 

On applique et on continue, suffit de lire l'énoncé.

#### Avant : 
```rust
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
```

#### Après : 
```rust
use std::fmt;

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(), // Changed grade to "A+"
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
```

# Et ensuite ?

Maintenant que vous avez terminé il vous suffit de faire la commande suivante 

```bash
rustlings verify
```
Et vous aurez le magnifique résultat suivant : 

```bash
★ All exercises completed! ★

+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                           \/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒

We hope you enjoyed learning about the various aspects of Rust!
If you noticed any issues, please don't hesitate to report them to our repo.
You can also contribute your own exercises to help the greater community!

Before reporting an issue or contributing, please read our guidelines:
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md
```