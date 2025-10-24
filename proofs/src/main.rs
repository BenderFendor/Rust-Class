use std::thread;

struct PublicParams {
    id: u32,
    curve_data: String,
}

impl PublicParams {
    fn new(id: u32) -> Self {
        Self {
            id,
            curve_data: format!("Generated data set {}", id),
        }
    }
}

struct ProverKey <'a>{
    params: &'a PublicParams,
    name: String,
}

impl<'a> ProverKey <'a> {
    fn new(pp: &'a PublicParams) -> Self { 
        Self {
            params: pp,
            name: String::from("Main Prover"),
        }
    }

    fn check_params(&self) {
        println!("[Prover] Using shared data: {}", self.params.curve_data);
    }

    fn prove(&self) {
        println!("[Prover] Generating proof with key named: {}", self.name);
    }
}

struct VerifierKey <'a>{
    params:&'a PublicParams,
}

impl<'a> VerifierKey <'a>{
    fn new(pp: &'a PublicParams) -> Self {
        Self { params: pp }
    }

    fn verify(&self) {
        println!("[Verifier] Verifying with ID: {}", self.params.id);
    }
}

fn main() {
    let initial_params = PublicParams::new(42);

    // TODO: This code doesn't compile! `initial_params` is moved.
    // Change the ProverKey/VerifierKey structs to take a reference to `initial_params` instead
    // You will need to use lifetimes. Don't clone!
    let prover_key = ProverKey::new(&initial_params);
    let verifier_key = VerifierKey::new(&initial_params);

    prover_key.check_params();
    prover_key.prove();

    verifier_key.verify();

    // Leave commented - we will talk about this when the assignment is complete
    
     let verifier_handle = thread::spawn(move || {
         verifier_key.verify();
     });

    
}
