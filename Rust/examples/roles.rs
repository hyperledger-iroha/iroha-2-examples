//! Shows how to work with permissions and roles.

use iroha::data_model::prelude::*;
use iroha_examples::*;
use iroha_executor_data_model::permission::asset_definition::CanRegisterAssetDefinition;
use iroha_executor_data_model::permission::domain::*;

fn main() -> Result<()> {
    let chess = Chess::id();
    let bob_in_chess = BobInChess::id();

    // define a role for managing chess
    let chess_manager: RoleId = "CHESS_MANAGER".parse()?;
    let new_chess_manager = Role::new(chess_manager.clone(), bob_in_chess.clone()).add_permission(
        CanModifyDomainMetadata {
            domain: chess.clone(),
        },
    );

    // grant the role to bob@chess
    let as_alice_in_wland = AliceInWonderland::client();
    as_alice_in_wland.submit_all_blocking::<InstructionBox>([
        Register::role(new_chess_manager).into(),
        Grant::account_role(chess_manager.clone(), bob_in_chess.clone()).into(),
    ])?;

    // bob@chess is now able to set key-values in chess
    let as_bob_in_chess = BobInChess::client();
    let key = "bob_key".parse::<Name>()?;
    as_bob_in_chess.submit_all_blocking::<InstructionBox>([
        SetKeyValue::domain(chess.clone(), key.clone(), "bob_value").into(),
        RemoveKeyValue::domain(chess.clone(), key).into(),
    ])?;

    // add permissions to an existing role
    as_alice_in_wland.submit_all_blocking([Grant::role_permission(
        CanRegisterAssetDefinition {
            domain: chess.clone(),
        },
        chess_manager.clone(),
    )])?;

    // bob@chess is now able to do more in chess
    let as_bob_in_chess = BobInChess::client();
    let chess_pawns = ChessPawns::id();
    let new_chess_pawns = AssetDefinition::numeric(chess_pawns.clone());
    as_bob_in_chess.submit_blocking(Register::asset_definition(new_chess_pawns))?;
    as_bob_in_chess.submit_blocking(Unregister::asset_definition(chess_pawns))?;

    // revoke the role from bob@chess
    as_alice_in_wland.submit_blocking(Revoke::account_role(chess_manager.clone(), bob_in_chess))?;
    // remove the role
    as_alice_in_wland.submit_blocking(Unregister::role(chess_manager))?;
    Ok(())
}
