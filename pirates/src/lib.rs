// Q1: The Captain's Logbook

// The Captain's Logbook must record details of each voyage.
// A logbook entry should contain:
//  - the `ship_name (a String)
//  - the `captain_name` (a String)
//  - the `total_gold_plundered` (u32).

// TODO: define the VoyageLog struct

#[derive(Clone)]
pub struct VoyageLog 
{
    pub ship_name: String,
    pub captain_name: String,
    pub total_gold_plundered: u32,
}



// Whenever a pirate Captain get's new plunder, he destroys his old records
// You don't want records of your plundering around when the royal navy comes knocking

// TODO: Write a function record_plunder
// that takes ownership of a VoyageLog and returns a new one with updated plunder
// (old plunder + new plunder)



pub fn record_plunder(log: VoyageLog, new_plunder: u32) -> VoyageLog {
    VoyageLog { 
        ship_name: (log.ship_name), captain_name: (log.captain_name), total_gold_plundered: (new_plunder) 
    }
}



// Q2. Navigational Hazards
// Pirates often encounter various Navigational Hazards at sea.
// These hazards are either:
//  - a Whirlpool (which has a strength rating, u8)
//  - an EnemyShip (which has a name, String)
//  - or simply a ClearSailing day.

#[derive(Debug, PartialEq)]
pub enum Hazard {
    Whirlpool(u8),     // Strength rating
    EnemyShip(String), // Ship's name
    ClearSailing,
}

// TODO: impl describe_hazard
// use a match expression on a Hazard reference to return a description.
// Look at the test cases for what each case should return
pub fn describe_hazard(hazard: &Hazard) -> String {
    match hazard
    {
        Hazard::Whirlpool(Strength) => format!("This is a whirlpool is {}",Strength),
        Hazard::EnemyShip(Name) => format!("This is an Enemyship named{}",Name),
        Hazard::ClearSailing => format!{"We are clear sailing"}
        
    }
}

// Q3: The Treasure Chest
// A Treasure Chest can hold any type of loot (gold doubloons, ancient relics, casks of grog).
// We need a generic way to represent and transfer this treasure.

// TODO: Define a generic struct named TreasureChest that hash:
// - `loot` of any type T
// - a u8 representing the `lock_difficulty`.
#[derive(Debug)]
pub struct TreasureChest<T> {
    pub loot: T,
    pub lock_difficulty: u8,
}

// TODO: impl transfer_loot,
// which updates the chest's loot using a mutable reference and returns the old loot.
// you should use std::mem::replace (go look at the documentation!)

// std::mem::replace swaps the old value (chest.loot) with the new one (new_loot)
// and returns the old value,
pub fn transfer_loot<T>(chest: &mut TreasureChest<T>, new_loot: T) -> T {
   std::mem::replace(&mut chest.loot,new_loot) 
}

// Q4: The Sea Shanty Scrutiny
// Sea Shantys should be short and memorable! Let's invent a way to check if our loot or
// ships have easily memorable names

/// Trait to check if something has an easily memorable name.
pub trait Memorable {
    fn is_short_name(&self) -> bool;
}

// TODO: Implement Memorable for String
// If the string is less than 8 characters long, it IS memorable



   impl Memorable for String {
    fn is_short_name (&self) -> bool
    {
        if self.len() < 8
        {
            true
        }
        else {
            false
        }

    }
}



// TODO: Implement Memorable for VoyageLog
// If BOTH the Captain's name and the Ship's name is less than 8 characters,
// if IS memorable


    impl Memorable for VoyageLog {
        fn is_short_name(&self) -> bool {
            if self.captain_name.len() & self.ship_name.len() < 8
            {
                true
            }
            else
            {
                false
            }
        }
}



/// Generic function that checks for memorability using a trait bound.
pub fn inspect_for_memorability<T: Memorable>(item: &T) -> String {
    if item.is_short_name() {
        String::from("Easily memorable!")
    } else {
        String::from("Too long for a shanty!")
    }
}

// Q5: The Shared Map
// The Secret Treasure Map is too valuable to give away (transfer ownership),
// but the crew needs to mark the location of buried treasure.

#[derive(Debug, PartialEq)]
pub struct TreasureMap {
    pub location_notes: String,
}

// TODO: impl read_map, which takes an IMMUTABLE reference to a treasure map,
// and prints the location_notes



pub fn read_map(map: &TreasureMap) 
{
    println!("The location notes are at {}",map.location_notes)
}



// TODO: finish the implementation of mark_x_on_map (by filling in the type)
// which arks 'X' on the map using a mutable reference.

pub fn mark_x_on_map(map: &mut TreasureMap) {
        map.location_notes.push_str(" X Marks The Spot!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_plunder_and_ownership() {
        let original_log = VoyageLog {
            ship_name: String::from("The Salty Siren"),
            captain_name: format!("Edward Teach"),
            total_gold_plundered: 500,
        };

        // Pass by ownership (consumes original_log)
        let updated_log = record_plunder(original_log.clone(), 250);

        assert_eq!(updated_log.total_gold_plundered, 750);
        assert_eq!(updated_log.ship_name, "The Salty Siren");

        // *** CHALLENGE DEMONSTRATION: Uncommenting the next line *must* cause a compile error. ***
        // println!("Original log ship: {}", original_log.ship_name);
        // The above line is commented out to allow the code to compile, proving the ownership transfer.
    }

    #[test]
    fn test_describe_hazard() {
        let hazard1 = Hazard::Whirlpool(9);
        let hazard2 = Hazard::EnemyShip(String::from("The Vicious Viper"));
        let hazard3 = Hazard::ClearSailing;

        // Use references to the hazards
        assert_eq!(
            describe_hazard(&hazard1),
            "Beware! A Whirlpool of strength 9 is ahead!"
        );
        assert_eq!(
            describe_hazard(&hazard2),
            "A fearsome foe! The Vicious Viper is sighted!"
        );
        assert_eq!(
            describe_hazard(&hazard3),
            "Smooth seas and fair winds, no danger in sight!"
        );
    }

    #[test]
    fn test_transfer_loot() {
        let old_loot = 500u32;
        let new_loot = 1000u32;
        let mut chest = TreasureChest {
            loot: old_loot,
            lock_difficulty: 10,
        };

        // Transfer new_loot into the chest and get the old_loot back
        let returned_old_loot = transfer_loot(&mut chest, new_loot);

        assert_eq!(chest.loot, new_loot);
        assert_eq!(returned_old_loot, old_loot);

        // Test with a different generic type (String)
        let mut string_chest = TreasureChest {
            loot: String::from("Ancient Relic"),
            lock_difficulty: 5,
        };
        let new_string_loot = String::from("Cask of Grog");
        let returned_string_loot = transfer_loot(&mut string_chest, new_string_loot.clone());

        assert_eq!(string_chest.loot, "Cask of Grog");
        assert_eq!(returned_string_loot, "Ancient Relic");
    }

    #[test]
    fn test_inspect_for_memorability() {
        let short_name = String::from("Gold"); // length 4
        let long_name = String::from("Doubloons"); // length 9

        let original_log = VoyageLog {
            ship_name: String::from("The Salty Siren"),
            captain_name: format!("Edward Teach"),
            total_gold_plundered: 500,
        };

        assert_eq!(inspect_for_memorability(&short_name), "Easily memorable!");
        assert_eq!(
            inspect_for_memorability(&long_name),
            "Too long for a shanty!"
        );
        assert_eq!(
            inspect_for_memorability(&original_log),
            "Too long for a shanty!"
        );
    }

    #[test]
    fn test_shared_map_references() {
        let mut map = TreasureMap {
            location_notes: String::from("Hidden beach near Tortuga."),
        };
        let initial_notes = map.location_notes.clone();

        // 1. Read map (immutable reference)
        // Note: We can't capture println output easily in unit tests, so we'll check the internal state.
        // read_map(&map); // This would print the output

        // 2. Mark X on map (mutable reference)
        mark_x_on_map(&mut map);

        // 3. Check the change
        let expected_notes = format!("{} X Marks The Spot!", initial_notes);
        assert_eq!(map.location_notes, expected_notes);

        // 4. Read map again to show the change
        // read_map(&map); // This would print the output

        // 5. Final check: Proving ownership is retained
        let final_notes = map.location_notes.clone(); // The map itself is still owned by the test function.
        assert_eq!(final_notes, expected_notes);
    }
}
