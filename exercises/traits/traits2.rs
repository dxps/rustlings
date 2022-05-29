/*
traits2.rs

Your task is to implement the trait `AppendBar' for a vector of strings.
To implement this trait, consider for a moment what it means to 'append "Bar"'
to a vector of strings.

No boiler plate code this time, you can do this!
*/

/*
----------------------------------------------------------------------------------------

Notes about why this works although there is `mut` in the implementation method argument,
although this is not stated in the trait function signature.

The Reference for associated functions (ref 1) says:

    The identifier is the name of the function. The generics, parameter list, return type,
    and where clause of the associated function must be the same as the associated function declarations's.

Matching the parameter list means matching the number and types of parameters.
The Reference for functions (ref 2) explains the structure of a parameter:

    FunctionParam : OuterAttribute* Pattern : Type

Where Pattern in this case is an Identifier Pattern (ref 3):

    IdentifierPattern : ref? mut? IDENTIFIER (@ Pattern ) ?

This induces that mut is part of the pattern, not part of the type which is the reason why mut (unlike &mut) is not part of the Signature at all, so thats why you are allowed to use it.

It's important to note here that mut self vs self is not the same as &self vs &mut self. As with other parameters, the mut in mut self is just an annotation on the binding of self, not the type.

Refs:
(1) https://doc.rust-lang.org/stable/reference/items/associated-items.html?highlight=asso#associated-functions-and-methods
(2) https://doc.rust-lang.org/stable/reference/items/functions.html
(3) https://doc.rust-lang.org/stable/reference/patterns.html

----------------------------------------------------------------------------------------
*/

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
