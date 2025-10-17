// RPG Data Structures and Iterators Assignment

use std::{collections::HashMap, string};

/// A struct representing a playable character in the RPG.
#[derive(Debug, PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub attack: u32,
}

/// A struct representing an item that can be carried or found.
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub name: String,
    pub value: u32,
    pub is_legendary: bool,
}

// Helper Functions
fn get_sample_items() -> Vec<Item> {
    vec![
        Item {
            name: "Health Potion".to_string(),
            value: 10,
            is_legendary: false,
        },
        Item {
            name: "Cursed Amulet".to_string(),
            value: 500,
            is_legendary: true,
        },
        Item {
            name: "Silver Coin".to_string(),
            value: 1,
            is_legendary: false,
        },
        Item {
            name: "Excalibur".to_string(),
            value: 1000,
            is_legendary: true,
        },
    ]
}

fn get_sample_party() -> Vec<Character> {
    vec![
        Character {
            name: "Alistair".to_string(),
            level: 12,
            attack: 20,
        },
        Character {
            name: "Seraphina".to_string(),
            level: 13,
            attack: 15,
        },
    ]
}

// Question 1: Creating an Inventory
// -----------------------------------------------------------------------------
pub fn q1_create_inventory(unique_item: &str) -> Vec<String> {
    let mut inventory = Vec::new();

    // TODO: push() the starting items (Rusty Sword, Healing Potion) to the inventory vec
    inventory.push(String::from("Rusty Sword"));
    inventory.push(String::from("Healing Potion"));
    // TODO: push() the unique_item to the vec
    inventory.push(String::from(unique_item));

    inventory
}

#[test]
fn test_q1_create_inventory() {
    let inventory = q1_create_inventory("Torch");
    assert_eq!(inventory.len(), 3);
    assert_eq!(
        inventory,
        vec![
            "Rusty Sword".to_string(),
            "Healing Potion".to_string(),
            "Torch".to_string()
        ]
    );
}

// Question 2: Initializing Stats
// -----------------------------------------------------------------------------
pub fn q2_initialize_stats(name: &str) -> HashMap<String, u32> {
    let mut stats = HashMap::new();

    // TODO: Use insert() to insert each starting stat
    // "Health" should have value 100
    // "Stamina" should have value 50
    // "Mana" should have value 25

    stats.insert(String::from("Health"), 100);
    stats.insert(String::from("Stamina"), 50);
    stats.insert(String::from("Mana"), 25);

    stats
}

#[test]
fn test_q2_initialize_stats() {
    let stats = q2_initialize_stats("Kaelen");
    assert!(stats.contains_key("Health"));
    assert_eq!(*stats.get("Health").unwrap(), 100);
    assert_eq!(*stats.get("Stamina").unwrap(), 50);
    assert_eq!(*stats.get("Mana").unwrap(), 25);
}

// Question 3: Displaying Quests
// -----------------------------------------------------------------------------
pub fn q3_list_quests(quest_log: &Vec<String>) -> Vec<String> {
    // TODO: iterate over the quest_log and format! each quest with its corresponding quest number
    // The Vec should contain formatted strings like "1. The Lost Artifact".

    // Use .iter().enumerate() to get both the index (i) and a reference (&quest)
    // You can use a for loop OR try using .map(...).collect() (look at Vec documentation!)

    let mut formatted_quests = vec![];

    for (i,quest) in quest_log.iter().enumerate()
    {
      formatted_quests.push(format!("{}. {}",i+1,quest))
    }
    formatted_quests
}

#[test]
fn test_q3_list_quests() {
    let quest_log = vec![
        "Defeat the Goblin King".to_string(),
        "Retrieve the Sunstone".to_string(),
        "Deliver the Secret Note".to_string(),
    ];
    let numbered_quests = q3_list_quests(&quest_log);
    assert_eq!(numbered_quests.len(), 3);
    assert_eq!(numbered_quests[0], "1. Defeat the Goblin King");
    assert_eq!(numbered_quests[2], "3. Deliver the Secret Note");
    println!("Q3 Test Passed: Quests correctly numbered using iter() and enumerate().");
}

// Question 4: Buffing Party Members
// -----------------------------------------------------------------------------
pub fn q4_apply_buff(party: &mut Vec<Character>, buff_amount: u32) {

    // TODO: apply the same buff_amount to all characters in the party
    // Use iter_mut() to modify each character in the party_vec

    for char in party
    {
        char.attack += buff_amount
    }
}

#[test]
fn test_q4_apply_buff() {
    let mut party = get_sample_party();
    let buff_amount = 5;
    q4_apply_buff(&mut party, buff_amount);

    // Alistair's original attack was 20, now 25
    assert_eq!(party[0].attack, 25);
    // Seraphina's original attack was 15, now 20
    assert_eq!(party[1].attack, 20);
}

// Question 5: Consuming Loot
// -----------------------------------------------------------------------------
pub fn q5_process_loot(loot_pile: Vec<Item>) -> u32 {
    let mut total_value = 0;

    // TODO: calculate the total value of a vector of loot, comsuming the item list in the process
    // Use into_iter(), which moves ownership out of the Vec

    for loot in loot_pile.into_iter()
    {
        total_value += loot.value;
    }

    total_value
}

#[test]
fn test_q5_process_loot() {
    let loot_pile = get_sample_items();
    let expected_total_value = 10 + 500 + 1 + 1000;
    let total_value = q5_process_loot(loot_pile);
    assert_eq!(total_value, expected_total_value);
}

// Question 6: Tracking Enemy Kills
// -----------------------------------------------------------------------------
/// **Challenge:** Update a kill counter for a specific enemy. If the enemy is new, add it with a count of 1. If it exists, increment the count.
pub fn q6_track_kill(kill_counts: &mut HashMap<String, u32>, enemy_name: &str) {

    // TODO: Update a kill counter for a specific enemy.
    // If the enemy is new, add it with a count of 1.
    // If the enemy is already in the vector exists, increment the count.
    // To me this is ugly looking
    kill_counts.entry(String::from(enemy_name)).and_modify(|count|*count += 1).or_insert(1);
    // Use entry(). (Look at the HashMap documentation!)
    // and_modify() and or_insert() might also be helpful! :)
}

#[test]
fn test_q6_track_kill() {
    let mut kill_counts: HashMap<String, u32> = HashMap::new();
    q6_track_kill(&mut kill_counts, "Goblin"); // New kill
    q6_track_kill(&mut kill_counts, "Goblin"); // Second kill
    q6_track_kill(&mut kill_counts, "Orc"); // New enemy

    assert_eq!(*kill_counts.get("Goblin").unwrap(), 2);
    assert_eq!(*kill_counts.get("Orc").unwrap(), 1);
}

// Question 7: Calculating Total Power
// -----------------------------------------------------------------------------
/// **Focus:** Using `values()` to iterate over the values of a HashMap, ignoring the keys.
pub fn q7_calculate_total_power(active_party: &HashMap<String, Character>) -> u32 {
    // TODO: calculate the total combined attack power of all active_party members
    // Use values() to get an iterator over references to the Character structs.
    // You can use a for loop OR try map().sum()

    active_party.values().map(|character| character.attack).sum()
}

#[test]
fn test_q7_calculate_total_power() {
    // We use the root-level setup function to populate the HashMap
    let party_vec = get_sample_party();
    let mut active_party = HashMap::new();
    active_party.insert(party_vec[0].name.clone(), party_vec[0].clone());
    active_party.insert(party_vec[1].name.clone(), party_vec[1].clone());

    // Alistair (20) + Seraphina (15) = 35
    let total_power = q7_calculate_total_power(&active_party);
    assert_eq!(total_power, 35);
}

// Question 8: Finding Legendary Items
// -----------------------------------------------------------------------------
/// **Challenge:** Filter a list of item drops (stored as a Vec of (String, Item) tuples) and return only the names of the legendary items.
/// **Focus:** Iterating over a Vec of tuples, using `iter()`, and pattern matching in a `for` loop or using `filter`.
pub fn q8_find_legendary_items(drops: &Vec<(String, Item)>) -> Vec<String> {
    // TODO: use filter(...).map(...).collect() to filter out the legendary items from the drops
    // list and return only the names of the legendary items.
   drops.iter().filter(|(_,item)|item.is_legendary).map(|(_,item)|item.name.clone()).collect()
}

#[test]
fn test_q8_find_legendary_items() {
    // Setup data using the root-level helper function
    let sample_items = get_sample_items();
    let drops = vec![
        ("PlayerA".to_string(), sample_items[0].clone()), // Health Potion (not legendary)
        ("PlayerB".to_string(), sample_items[1].clone()), // Cursed Amulet (legendary)
        ("PlayerC".to_string(), sample_items[2].clone()), // Silver Coin (not legendary)
        ("PlayerA".to_string(), sample_items[3].clone()), // Excalibur (legendary)
    ];

    let legendaries = q8_find_legendary_items(&drops);
    assert_eq!(legendaries.len(), 2);
    assert!(legendaries.contains(&"Cursed Amulet".to_string()));
    assert!(legendaries.contains(&"Excalibur".to_string()));
}
