use crate::Parameters;
use ark_ec::PairingEngine;
use ark_poly::GeneralEvaluationDomain;
use ark_relations::r1cs::{ConstraintSynthesizer, SynthesisError};
use rand::Rng;

pub mod generic;

/// Generates a random common reference string for
/// a circuit.
#[inline]
pub fn generate_random_parameters<E, C, R>(
    circuit: C,
    rng: &mut R,
) -> Result<Parameters<E>, SynthesisError>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<E::Fr>,
    R: Rng,
{
    self::generic::generate_random_parameters::<E, C, GeneralEvaluationDomain<E::Fr>, R>(
        circuit, rng,
    )
}

/// Create parameters for a circuit, given some toxic waste.
#[inline]
pub fn generate_parameters<E, C, R>(
    circuit: C,
    alpha: E::Fr,
    beta: E::Fr,
    gamma: E::Fr,
    g: E::G1Projective,
    h: E::G2Projective,
    rng: &mut R,
) -> Result<Parameters<E>, SynthesisError>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<E::Fr>,
    R: Rng,
{
    self::generic::generate_parameters::<E, C, GeneralEvaluationDomain<E::Fr>, R>(
        circuit, alpha, beta, gamma, g, h, rng,
    )
}