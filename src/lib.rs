// #![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn gcd_test() {
        use crate::math::fractions::*;

        let f2over3 = Frac32 {
            numerator: 2,
            denominator: 3,
        };

        let f6over9 = Frac32 {
            numerator: 6,
            denominator: 9,
        };

        assert_eq!(f2over3.gcd(f6over9), 3);
    }

    #[test]
    fn lcm_test() {
        use crate::math::fractions::*;

        let f2over3 = Frac32 {
            numerator: 2,
            denominator: 3,
        };

        let f6over9 = Frac32 {
            numerator: 6,
            denominator: 9,
        };

        assert_eq!(f2over3.lcm(f6over9), 9);
    }

    #[test]
    fn item_creation_test() {
        use crate::itemsys::*;

        let stone = Item::new("stone", None)
                    .add_tag("stone")
                    .add_tag("smeltable");

        let coal = Item::new("coal", None)
                    .add_tag("fuel");
        
        let iron_ore = Item::new("iron_ore", None)
                    .add_tag("smeltable")
                    .add_tag("ore");
        
        
        
        let lst: ItemList<u64> = ItemList::new()
                                .add_item(&stone, 0)
                                .add_item(&coal, 0)
                                .add_item(&iron_ore, 0);

        assert_eq!(lst, ItemList {
            contents: vec![
                (Item {
                    name: "stone".to_string(),
                    img_path: None,
                    tags: vec!["stone".to_string(), "smeltable".to_string()],
                }, 0),
                (Item {
                    name: "coal".to_string(),
                    img_path: None,
                    tags: vec!["fuel".to_string()],
                }, 0),
                (Item {
                    name: "iron_ore".to_string(),
                    img_path: None,
                    tags: vec!["smeltable".to_string(), "ore".to_string()],
                }, 0),
            ]
        })
    }

    #[test]
    fn item_editing_test() {
        use crate::itemsys::*;

        let stone = Item::new("stone", None).add_tag("stone");
        let no_exist = Item::new("", None);

        let mut storage = ItemList::new().add_item(&stone, 0u64);

        storage.edit_value(&stone, *storage.get_val(&stone) + 10);

        assert_eq!(storage.get_val(&stone), &10);
        assert_eq!(storage.try_get_val(&no_exist), None);
    }
}


/// Various, useful math operations and number formats
pub mod math {
    /// Returns the larger of two items
    /// 
    /// num1 and num2 must implement PartialOrd
    /// 
    /// Will return num2 if they are equal
    pub fn greater<T>(num1: T, num2: T) -> T
            where T: PartialOrd  {
        if num1 > num2 {
            num1
        } else {
            num2
        }
    }

    /// Returns the smaller of two items
    /// 
    /// num1 and num2 must implement PartialOrd
    /// 
    /// Will return num2 if they are equal
    pub fn lesser<T>(num1: T, num2: T) -> T
            where T: PartialOrd  {
        if num1 < num2 {
            num1
        } else {
            num2
        }
    }

    /// A fraction is a number represented as a division. 0.5 is 1/2 as a fraction because: 1 divided by 2 = 0.5
    /// 
    /// The benifit of using fractions is that they are more precice. 1/3 + 2/3 = 1/1, always.
    /// There is no roundoff error like there are with floating point numbers where 1/3 + 2/3 = 0.999999...1, most of the time.
    pub mod fractions {


        
        /// A number represented as a fraction
        pub struct Frac32 {
            pub numerator: i32,
            pub denominator: u32,
        }

        impl Frac32 {
            /// returns the greatest common divisor of the two demoninators
            pub fn gcd(&self, num: Frac32) -> u32 {
                let mut _output = 0u32;
                let less = super::greater(self.denominator, num.denominator);
                for n in 0..less {
                    if ((self.denominator % (less - n)) == 0) & ((num.denominator % (less - n)) == 0) {
                        return less-n;
                    }
                };
                _output
            }

            /// returns the least common multiple of the two denominators
            pub fn lcm(&self, num: Frac32) -> u32 {
                (self.denominator * num.denominator) / self.gcd(num)
            }

            ///
            // pub fn from<T>(num: T) -> Frac32 {
            //     Frac32 {
            //         numerator: 0,
            //         denominator: 0,
            //     }
            // }

            ///
            pub fn mul(&mut self, num: Frac32) {
                self.numerator *= num.numerator;
                self.denominator *= num.denominator;
            }


            pub fn div(&mut self, num: Frac32) {
                self.numerator *= num.numerator;
                self.denominator *= num.denominator;
            }
        }

        
    }
}


/// A system to represen items in games
pub mod itemsys {
    #![allow(unused)]   // temporary!-----------------------------------==
    
    /// Like a template that represents an item
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
    pub struct Item {
        pub name: String,
        pub img_path: Option<String>,
        pub tags: Vec<String>,
    }
    impl Item {
        pub fn new(name: &str, img_path: Option<&str>) -> Item {
            Item {
                name: String::from(name),
                img_path: match img_path {Some(v) => Some(String::from(v)), None => None},
                tags: vec![]
            }
        }

        pub fn add_tag(mut self, tag: &str) -> Item {
            self.tags.push(tag.to_string());
            self
        }
    }



    /// A list of Items with a value tied to each of them
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ItemList<T> {
        pub contents: Vec<(Item, T)>
    }
    impl<T> ItemList<T> {
        pub fn new() -> ItemList<T> {
            let c: Vec<(Item, T)> = vec![];
            ItemList {
                contents: c
            }
        }

        /// Add an item to the list
        /// 
        /// Panics if it's already in the list
        pub fn add_item(mut self, item: &Item, val: T) -> ItemList<T> {
            if self.contains(&item) {
                panic!("item {:#?} was already in the list!", item);
            }
            self.contents.push((item.clone(), val));
            self
        }

        /// Attemps to add an item to the list
        pub fn try_add_item(&mut self, item: &Item, val: T) -> bool {
            if self.contains(&item) {
                self.contents.push((item.clone(), val));
                true
            } else {
                false
            }
        }

        /// Overwrites the value of item with val
        /// 
        /// Panics if item doesn't exist
        pub fn edit_value(&mut self, item: &Item, val: T) {
            if self.contains(&item) {
                let pos = self.contents.binary_search_by_key(&item, |(a, b)| a).unwrap();
                self.contents[pos].1 = val;
            } else {
                panic!("No item {:#?} in ItemList", item);
            }
        }

        /// Returns true if the ItemList contains item
        pub fn contains(&self, item: &Item) -> bool {
            self.contents.binary_search_by_key(item, |(a, b)| a.clone()).is_ok()
        }

        /// Returns a reference to the value tied to an item
        /// 
        /// Panics if item doesn't exist
        pub fn get_val(&self, item: &Item) -> &T {
            let pos = self.contents.binary_search_by_key(&item, |(a, b)| a).unwrap();
            &self.contents[pos].1
        }

        /// Returns an Option of a reference to the value tied to an item
        /// 
        /// Returns None if item doesn't exist
        pub fn try_get_val(&self, item: &Item) -> Option<&T> {
            let pos = match self.contents.binary_search_by_key(&item, |(a, b)| a) {
                Ok(v) => v,
                Err(e) => return None
            };
            Some(&self.contents[pos].1)
        }

        
    }
    impl ItemList<u64> {
        pub fn craft(&mut self, recipie: Recipie, amount: u64) {
            for input in recipie.inputs {
                if *self.get_val(&input.0) >= input.1 * amount {
                    
                }
            }
        }
    }



    /// A recipie consisting of inputs and outputs
    /// 
    /// Can be performed on an ItenList<u64>
    pub struct Recipie {
        name: String,
        inputs: Vec<(Item, u64)>,
        outputs: Vec<(Item, u64)>,
    }
    impl Recipie {
        pub fn new(name: &str, inputs: Vec<(Item, u64)>, outputs: Vec<(Item, u64)>) -> Recipie {
            Recipie {
                name: name.to_string(),
                inputs: inputs,
                outputs: outputs,
            }
        }
    }
}