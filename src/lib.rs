//! An implementation of the [`Groth-Maller`] simulation extractable zkSNARK.
//!
//! [`Groth-Maller`]: https://eprint.iacr.org/2017/540
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![allow(clippy::many_single_char_names, clippy::op_ref)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate bench_utils;

/// Reduce an R1CS instance to a *Square Arithmetic Program* instance.
pub mod r1cs_to_sap;

/// Data structures used by the prover, verifier, and generator.
pub mod data_structures;

/// Generate public parameters for the GM17 zkSNARK construction.
pub mod generator;

/// Create proofs for the GM17 zkSNARK construction.
pub mod prover;

/// Verify proofs for the GM17 zkSNARK construction.
pub mod verifier;

#[cfg(test)]
mod test;

pub use self::data_structures::*;
pub use self::{generator::*, prover::*, verifier::*};

use ark_ec::PairingEngine;
use ark_relations::r1cs::{ConstraintSynthesizer, SynthesisError};
use ark_snark::*;
use ark_std::marker::PhantomData;
use rand::RngCore;

/// The SNARK of [[GrothMaller17]](https://eprint.iacr.org/2017/540).
pub struct GM17<E: PairingEngine> {
    e_phantom: PhantomData<E>,
}

impl<E: PairingEngine> SNARK<E::Fr> for GM17<E> {
    type ProvingKey = ProvingKey<E>;
    type VerifyingKey = VerifyingKey<E>;
    type Proof = Proof<E>;
    type ProcessedVerifyingKey = PreparedVerifyingKey<E>;
    type Error = SynthesisError;

    fn circuit_specific_setup<C: ConstraintSynthesizer<E::Fr>, R: RngCore>(
        circuit: C,
        rng: &mut R,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error> {
        let pk = generate_random_parameters::<E, C, R>(circuit, rng)?;
        let vk = pk.vk.clone();

        Ok((pk, vk))
    }

    fn prove<C: ConstraintSynthesizer<E::Fr>, R: RngCore>(
        pk: &Self::ProvingKey,
        circuit: C,
        rng: &mut R,
    ) -> Result<Self::Proof, Self::Error> {
        create_random_proof::<E, _, _>(circuit, pk, rng)
    }

    fn process_vk(
        circuit_vk: &Self::VerifyingKey,
    ) -> Result<Self::ProcessedVerifyingKey, Self::Error> {
        Ok(prepare_verifying_key(circuit_vk))
    }

    fn verify_with_processed_vk(
        circuit_pvk: &Self::ProcessedVerifyingKey,
        x: &[E::Fr],
        proof: &Self::Proof,
    ) -> Result<bool, Self::Error> {
        Ok(verify_proof(&circuit_pvk, proof, &x)?)
    }
}

impl<E: PairingEngine> CircuitSpecificSetupSNARK<E::Fr> for GM17<E> {}
