extern crate bellman;
extern crate ff;
extern crate group;
extern crate rand;

use bellman::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use ff::Field;
use group::CurveProjective;
use rand::rngs::OsRng;
use zkgroup::bls12_381::{Bls12, Fr};

struct SquareRootCircuit<E: bellman::Engine> {
    pub number: Option<E::Fr>,
    pub square_root: Option<E::Fr>,
}

impl<E: bellman::Engine> Circuit<E> for SquareRootCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let number_value = self.number;
        let square_root_value = self.square_root;

        let number = cs.alloc(
            || "number",
            || number_value.ok_or(SynthesisError::AssignmentMissing),
        )?;
        let square_root = cs.alloc(
            || "square_root",
            || square_root_value.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.enforce(
            || "square_root * square_root = number",
            |lc| lc + square_root,
            |lc| lc + square_root,
            |lc| lc + number,
        );

        Ok(())
    }
}

fn main() {
    let rng = &mut OsRng::default();

    // Genera los parámetros de prueba y verificación
    let params = {
        let c = SquareRootCircuit::<Bls12> {
            number: None,
            square_root: None,
        };
        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    // Número y raíz cuadrada de ejemplo
    let number = Fr::from_str("16").unwrap();
    let square_root = Fr::from_str("4").unwrap();

    // Crea la prueba
    let proof = {
        let c = SquareRootCircuit {
            number: Some(number),
            square_root: Some(square_root),
        };
        create_random_proof(c, &params, rng).unwrap()
    };

    // Verifica la prueba
    assert!(verify_proof(&pvk, &proof, &[number]).unwrap());
}
