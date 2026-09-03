//! This module contains implementations for witness rules in the ATC blockchain.
//!
//! It includes:
//! - `WitnessAction`: Represents the action to be taken (Allow or Deny).
//! - `WitnessCondition`: Represents various conditions for witness rules.
//! - `WitnessRule`: Combines an action and a condition to form a complete rule.
//!
//! This module provides structures and implementations for creating, serializing,
//! and deserializing witness rules used in ATC smart contracts.
//!
//! # Usage
//!
//! To use witness rules in your ATC blockchain transactions:
//!
//! 1. Import the necessary types:
//!    ```rust
//!    use atipicial::atipicial_builder::{WitnessAction, WitnessCondition, WitnessRule};
//!    ```
//!
//! 2. Create a witness rule:
//!    ```rust
//!    use atipicial::atipicial_builder::{WitnessAction, WitnessCondition, WitnessRule};
//!    
//!    let condition = WitnessCondition::CalledByEntry;
//!    let rule = WitnessRule::new(WitnessAction::Allow, condition);
//!    ```
//!
//! 3. Use the rule with a transaction signer:
//!    ```rust
//!    use atipicial::atipicial_builder::{WitnessAction, WitnessCondition, WitnessRule};
//!
//!    // Create a witness rule
//!    let condition = WitnessCondition::CalledByEntry;
//!    let rule = WitnessRule::new(WitnessAction::Allow, condition);
//!
//!    // The rule can be attached to a signer when building transactions
//!    assert_eq!(rule.action, WitnessAction::Allow);
//!    ```
//!
//! 4. Serialize or deserialize witness rules as needed:
//!    ```rust
//!    use atipicial::atipicial_builder::{WitnessAction, WitnessCondition, WitnessRule};
//!    use atipicial::atipicial_codec::{Decoder, AtipicialSerializable};
//!    
//!    let condition = WitnessCondition::CalledByEntry;
//!    let rule = WitnessRule::new(WitnessAction::Allow, condition);
//!    
//!    let serialized = rule.to_array();
//!    let mut decoder = Decoder::new(&serialized);
//!    let deserialized = WitnessRule::decode(&mut decoder).unwrap();
//!    ```
//!
//! Remember to handle errors and consider the implications of different witness conditions
//! and actions for your specific use case.

pub use witness_action::*;
pub use witness_condition::*;
pub use witness_rule::*;

mod witness_action;
mod witness_condition;
mod witness_rule;
