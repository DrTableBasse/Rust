// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



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
