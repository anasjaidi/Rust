import * as borsh from '@coral-xyz/borsh'
const BN = require('bn.js')

const equipPlayerSchema = borsh.struct([
  borsh.u8('variant'),
  borsh.u16('playerId'),
  borsh.u128('itemId')
])

const buffer = Buffer.alloc(1000)
equipPlayerSchema.encode({ variant: 2, playerId: 1435, itemId: new BN(737498) }, buffer)

const instructionBuffer = buffer.slice(0, equipPlayerSchema.getSpan(buffer))
console.log(instructionBuffer.length)
console.log(instructionBuffer.toJSON())