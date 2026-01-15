// Coq, Idrus, Agda - języki do robienia dowodów

use std::collections::HashMap;
use std::hash::Hash;

#[allow(unused_macros)]
macro_rules! string {
    ($x: literal) => {
        String::from($x)
    }
}

pub trait StateMachine<S> {
    fn step(&self, state: S) -> Option<S>;
}

macro_rules! impl_state_machine {
    (
        $name: ident, 
        [$($p:tt -> $q:tt;)+]

    ) => {
        struct $name {
            map: HashMap<i32, i32>
        }

        #[allow(dead_code)]
        impl $name {
            pub fn default() -> Self {
                const END: i32 = -1;
                let map: HashMap<i32, i32> = [
                    $(($p, $q)),+
                ].into();

                Self { map }
            }
        }

        impl StateMachine<i32> for $name {
            fn step(&self, state: i32) -> Option<i32> {
                const END: i32 = -1;

                self.map
                    .get(&state)
                    .copied()
                    .map(|new_state| {
                        self.map
                            .contains_key(&new_state)
                            .then_some(new_state)
                            .and_then(|state| {
                                state
                                    .ne(&END)
                                    .then_some(state)
                            })
                    })
                    .flatten()
            }
        }
    }
}

impl_state_machine!(MyIntMachine, [
    1 -> 2;
    2 -> 3;
    3 -> END;
]);


impl<S> StateMachine<S> for HashMap<S, S>
where
    S: Clone + Eq + Hash  
{
    fn step(&self, state: S) -> Option<S> {
        self
            .get(&state)
            .cloned()
            .and_then(|new_state| {
                self
                    .contains_key(&new_state)
                    .then_some(new_state)
            })
    }
}

pub fn join_machines<'a, S, M1, M2>(x: M1, y: M2) -> Vec<Box<dyn StateMachine<S> + 'a>>
where
    M1: StateMachine<S> + 'a,
    M2: StateMachine<S> + 'a,
{
    vec![Box::new(x), Box::new(y)]
}


#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_macro() {
        let s1 = string!("napis");
        assert_eq!(s1, "napis");
    }

    #[test]
    fn int_machine() {
        let machine = MyIntMachine::default();
        let mut state: i32 = 1;
        let mut counter = 0;
        loop {
            let s = machine.step(state);
            counter += 1;
            if let Some(inner) = s {
                state = inner;
            }
            else{
                break;
            }
        }

        assert_eq!(counter, 3);
    }

    #[test]
    fn some_other_test() {
        let map1: HashMap<u32, u32> = [
            (1, 2), (2, 5), (5, 1)
        ].into();
        let map2: HashMap<u32, u32> = [
            (2, 3), (5, 3), (3, 12353), (12353, 5)
        ].into();

        let v = join_machines(map1, map2);
        v
            .iter()
            .for_each(|m| {
                println!("state: {:?}", m.step(2));
            });
    }
}
