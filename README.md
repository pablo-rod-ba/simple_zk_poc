# Zero Knowldge Proof on Rust 

This program demonstrates a zero-knowledge proof of knowledge for the square root of a number using the Bellman library in Rust. It showcases the usage of the bellman crate to generate and verify proofs in a square root circuit.

# Circuit

In the context of zero-knowledge proofs, a circuit represents a computational model that defines a set of constraints and logical operations on input variables. It is used to express relationships between input and output variables, ensuring that certain conditions are satisfied.

The SquareRootCircuit struct in this program represents an arithmetic circuit for the square root. It defines the constraints for the circuit, ensuring that the assigned square_root squared equals the assigned number. The Circuit trait implementation defines the synthesize function, where the constraints are enforced using the provided constraint system (CS).

Circuits play a crucial role in zero-knowledge proofs as they define the rules and relationships that need to be proven without revealing any sensitive information. They provide a way to construct proofs that can convince a verifier of the validity of certain computations without disclosing the actual inputs.

# Proof Generation and Verification

The program generates random parameters using the generate_random_parameters function and prepares the verifying key. It then assigns a number and its square root to the circuit and creates a random proof using the create_random_proof function. Finally, it verifies the proof using the verify_proof function and asserts the validity of the proof.

Please note that this is a simplified example to demonstrate the usage of zero-knowledge proofs with the Bellman library. In a real-world scenario, more complex circuits and additional security considerations would be involved.

# Limitations
The current implementation has the following limitations:

- It only demonstrates a specific example of a square root circuit. It does not handle inputs dynamically.
- The program assumes that the number and square root are known beforehand. In a real-world scenario, the number and square root would be inputs to the proof.
- The program does not handle potential errors or exceptions during proof generation and verification. Proper error handling should be implemented for robustness.

# Next Steps
To further enhance the functionality and usability of the program, consider the following next steps:

- Extend the program to handle user input for the number and square root.
- Implement a more complex circuit with additional constraints to showcase the capabilities of zero-knowledge proofs.
- Improve error handling and provide informative error messages to enhance user experience.
- Explore optimizations and performance improvements to make the proof generation and verification process more efficient.
- Consider integrating the proof generation and verification process into a larger application or system where zero-knowledge proofs can be utilized for practical use cases.