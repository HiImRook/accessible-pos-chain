// examples/sign_tx.js
// Node.js example using tweetnacl and tweetnacl-util to create an ed25519 signature over a canonical JSON payload.
// Install deps: npm i tweetnacl tweetnacl-util bs58
const nacl = require('tweetnacl');
const naclUtil = require('tweetnacl-util');
const fs = require('fs');

// === CONFIG: replace these with test keys for local use ===
// Use a 32-byte hex seed or a 64-byte secret key in hex.
const SENDER_PRIVATE_HEX = process.env.SENDER_PRIVATE_HEX || 'REPLACE_WITH_64_OR_32_BYTE_HEX';
const SENDER_PUBLIC_HEX = process.env.SENDER_PUBLIC_HEX || 'REPLACE_WITH_64_HEX_PUBLIC_KEY';
const RECIPIENT_PUBLIC_HEX = process.env.RECIPIENT_PUBLIC_HEX || 'REPLACE_WITH_RECIPIENT_PUBLIC_KEY_HEX';

function hexToUint8(hex) {
  if (hex.startsWith('0x')) hex = hex.slice(2);
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = parseInt(hex.substr(i * 2, 2), 16);
  }
  return bytes;
}

function uint8ToBase64(u8) {
  return naclUtil.encodeBase64(Buffer.from(u8));
}

function canonicalSerialize(txObj) {
  // MUST match the node's expected canonical order. Here: from,to,amount,nonce
  const canonical = {
    from: txObj.from,
    to: txObj.to,
    amount: txObj.amount,
    nonce: txObj.nonce
  };
  return JSON.stringify(canonical);
}

const unsignedTx = {
  from: SENDER_PUBLIC_HEX,
  to: RECIPIENT_PUBLIC_HEX,
  amount: 100,
  nonce: 1
};

const serialized = canonicalSerialize(unsignedTx);
const privateKeyBytes = hexToUint8(SENDER_PRIVATE_HEX);
let keyPair;
if (privateKeyBytes.length === 32) {
  keyPair = nacl.sign.keyPair.fromSeed(privateKeyBytes);
} else if (privateKeyBytes.length === 64) {
  try {
    keyPair = nacl.sign.keyPair.fromSecretKey(privateKeyBytes);
  } catch (e) {
    keyPair = nacl.sign.keyPair.fromSeed(privateKeyBytes.slice(0, 32));
  }
} else {
  throw new Error('Private key must be 32-byte seed or 64-byte secretKey (hex)');
}

const messageBytes = naclUtil.decodeUTF8(serialized);
const sig = nacl.sign.detached(messageBytes, keyPair.secretKey);
const sigBase64 = uint8ToBase64(sig);

const signedTx = {
  ...unsignedTx,
  sig: sigBase64
};

fs.writeFileSync('examples/example_tx.signed.json', JSON.stringify(signedTx, null, 2));
console.log('Wrote examples/example_tx.signed.json');
console.log('Serialized payload (what was signed):', serialized);
console.log('Signature (base64):', sigBase64);

