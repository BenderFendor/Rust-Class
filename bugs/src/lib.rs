// Your task is to add the necessary lifetime annotations (e.g., <'a>)
// to the struct definitions, function signatures, and method signatures
// so that the code compiles and all tests pass.

// Give the Pest struct a lifetime to pass to name
// -----------------------------------------------
struct Pest<'a> {
    name: &'a str,
}

impl<'a> Pest<'a> {
    fn new(name: &'a str) -> Pest<'a> {
        Pest { name }
    }
}

// Determine which of two bugs is "more venomous" (based on length),
// and return a reference to the string name of the longer one.
// The output reference must be valid as long as the shortest-lived input is valid
// -------------------------------------------------------------------------------
fn most_venomous<'a>(bug_a: &'a str, bug_b: &'a str) -> &'a str {
    if bug_a.len() > bug_b.len() {
        bug_a
    } else {
        bug_b
    }
}

// A BugReport must reference both the name of the witness and the
// description of the bug. These references may come from different scopes,
// but both must live long enough for the report to be valid.
// ----------------------------------------------------------
struct BugReport<'a, 'b> {
    witness: &'a str,
    description: &'b str,
}

impl<'a, 'b> BugReport<'a, 'b> {
    fn generate(w: &'a str, d: &'b str) -> BugReport<'a, 'b> {
        BugReport {
            witness: w,
            description: d,
        }
    }
}

// A Spider struct needs a method to check if it can attack a target.
// It takes a reference to 'self' and a reference to the 'target_name'.
// The output is a reference to the Spider's own name, so its lifetime
// depends on the Spider instance's lifetime.
// ------------------------------------------
struct Spider {
    name: String,
}

impl Spider {
    fn check_attack<'a, 'b>(&'a self, target_name: &'b str) -> &'a str {
        if self.name.len() > target_name.len() {
            // Assume longer name means a bigger threat for this silly bug game
            &self.name
        } else {
            "Not attacking this one."
        }
    }
}

// A function that returns a reference to a hardcoded, immutable,
// globally available string constant. This requires the special `'static` lifetime.
// ---------------------------------------------------------------------------------
const ANCIENT_SCARAB_NAME: &str = "Ancient Scarab";

fn get_ancient_bug() -> &'static str {
    ANCIENT_SCARAB_NAME
}

// The 'molting' process returns a reference to the same bug that went in.
// The output reference lifetime must be tied to the input reference lifetime.
// ---------------------------------------------------------------------------
fn molt<'a>(bug: &'a str) -> &'a str {
    bug // Returns a reference to the input data
}

// The function takes two references (give them DIFFERENT lifetimes!),
// but only one determines the output lifetime.
// This is a case where the lifetime elision rules are ambiguous, requiring
// explicit lifetime annotations to clarify which input the output is borrowing from.
// ----------------------------------------------------------------------------------
fn crossbreed<'a, 'b>(bug_parent_a: &'a str, environment_name: &'b str) -> &'a str {
    // The resulting bug is named after bug_parent_a, ignoring the environment
    bug_parent_a
}

// Don't need to modify tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pest_struct_lifetime() {
        let pest_name = String::from("Aphid");
        let pest = Pest::new(&pest_name);
        assert_eq!(pest.name, "Aphid");
        // Test passes if the Pest struct compiles with correct lifetime.
    }

    #[test]
    fn most_venomous_lifetime() {
        let bug1 = "Mosquito";
        let bug2 = "Flea";
        assert_eq!(most_venomous(bug1, bug2), "Mosquito");

        let longer_lived_scope = String::from("Tarantula");
        {
            let shorter_lived_scope = "Wasp";
            let result = most_venomous(&longer_lived_scope, shorter_lived_scope);
            assert_eq!(result, "Tarantula");
            // The result reference must live at least as long as the shortest input (shorter_lived_scope)
        }
    }

    #[test]
    fn bug_report_multiple_lifetimes() {
        let witness_scope = String::from("Dr. Entomologist");
        let description_scope = String::from("It had too many legs.");
        let report = BugReport::generate(&witness_scope, &description_scope);
        assert_eq!(report.witness, "Dr. Entomologist");
        assert_eq!(report.description, "It had too many legs.");
        // Test passes if BugReport handles two independent reference lifetimes.
    }

    #[test]
    fn spider_method_lifetime() {
        let spider = Spider {
            name: String::from("Black Widow"),
        };
        let target1 = "Fly";
        let target2 = "Black Widow-like thing with a long name";

        // 'self' lifetime is used for the output
        assert_eq!(spider.check_attack(target1), "Black Widow");
        assert_eq!(spider.check_attack(target2), "Not attacking this one.");
    }

    #[test]
    fn static_lifetime() {
        let ancient_bug_name = get_ancient_bug();
        assert_eq!(ancient_bug_name, ANCIENT_SCARAB_NAME);
        // Test passes if get_ancient_bug compiles and uses the correct 'static lifetime.
    }

    #[test]
    fn molt_passthrough() {
        let juvenile = "Caterpillar";
        let adult = molt(juvenile);
        assert_eq!(adult, "Caterpillar");
        // The output reference must be tied to the input reference.
    }

    #[test]
    fn elision_failure() {
        let parent_name = String::from("Mutant Beetle");
        let environment = "Polluted Swamp";
        // The output borrows only from parent_name, requiring explicit lifetime annotation.
        let cross = crossbreed(&parent_name, environment);
        assert_eq!(cross, "Mutant Beetle");
    }
}
