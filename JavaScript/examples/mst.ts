import { crypto } from '@iroha2/crypto-target-node'
import { freeScope } from '@iroha2/crypto-core'
import { datamodel, sugar } from '@iroha2/data-model'
import {
  Client,
  makeSignedTransaction,
  makeTransactionPayload,
  Signer,
  signTransaction,
  Torii,
  type ToriiRequirementsForApiHttp
} from '@iroha2/client'
import { expect } from 'vitest'

// 1.

declare const adminClient: Client
declare const torii: ToriiRequirementsForApiHttp

const keyPair1 = crypto.KeyPair.generate()
const keyPair2 = crypto.KeyPair.generate()

const accountId = sugar.accountId('mad_hatter', 'wonderland')
const assetDefinitionId = sugar.assetDefinitionId('camomile', 'wonderland')

const registerAccount = sugar.instruction.register(
  sugar.identifiable.newAccount(accountId, [
    freeScope(() => keyPair1.publicKey().toDataModel())
  ])
)
const setSignatureCondition = sugar.instruction.mint(
  datamodel.Value(
    'SignatureCheckCondition',
    datamodel.SignatureCheckCondition(
      'AllAccountSignaturesAnd',
      datamodel.VecPublicKey([
        freeScope(() => keyPair2.publicKey().toDataModel())
      ])
    )
  ),
  datamodel.IdBox('AccountId', accountId)
)
const registerAssetDefinition = sugar.instruction.register(
  sugar.identifiable.newAssetDefinition(
    assetDefinitionId,
    datamodel.AssetValueType('Quantity'),
    { mintable: datamodel.Mintable('Infinitely') }
  )
)

await adminClient.submitExecutable(
  torii,
  sugar.executable.instructions([
    registerAccount,
    setSignatureCondition,
    registerAssetDefinition
  ])
)

// ...wait for submitted

// 2.

const madHatterClient = new Client({
  signer: new Signer(accountId, keyPair1)
})

const quantity = 42
const assetId = sugar.assetId(accountId, assetDefinitionId)
const mintAsset = sugar.instruction.mint(
  sugar.value.numericU32(quantity),
  datamodel.IdBox('AssetId', assetId)
)

const transaction = makeSignedTransaction(
  makeTransactionPayload({
    executable: sugar.executable.instructions(mintAsset),
    accountId
  }),
  madHatterClient.signer
)

await Torii.submit(torii, transaction)

// ...wait

// 3.

{
  const asset = await madHatterClient.requestWithQueryBox(
    torii,
    sugar.find.assetById(assetId)
  )

  expect(() =>
    asset.as('Err').enum.as('QueryFailed').enum.as('Find').enum.as('Asset')
  ).not.toThrow()
}

// 4.

const newSigner = new Signer(accountId, keyPair2)
transaction.enum
  .as('V1')
  .signatures.push(
    signTransaction(transaction.enum.as('V1').payload, newSigner)
  )
await Torii.submit(torii, transaction)

// ...wait

// 5.

{
  const asset = await madHatterClient.requestWithQueryBox(
    torii,
    sugar.find.assetById(assetId)
  )

  expect(
    asset
      .as('Ok')
      .batch.enum.as('Identifiable')
      .enum.as('Asset')
      .value.enum.as('Quantity')
  ).toEqual(quantity)
}
