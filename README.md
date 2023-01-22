Tool to get all outputs from a full stardust snapshot and write them into a .json file.

For more information about the snapshot file please read https://github.com/iotaledger/tips/blob/main/tips/TIP-0035/tip-0035.md.

Download a new full snapshot from https://files.testnet.shimmer.network/snapshots/latest-full_snapshot.bin or another source (maybe need to rename it to `latest-full_snapshot.bin`) and then run it with `cargo run --release`.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.66.0)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.66.0)

Output looks like `0snapshot.json` for a private tangle or like the following for the testnet
```JSON
{
  "ledgerIndex": 3490534704,
  "outputIdsWithOutput": {
    "0x0000702f9a4c97cabda0f0dbdf42cf96f2352c11721c6c60f7a2165e266e48a70000": {
      "amount": "100000000",
      "type": 3,
      "unlockConditions": [
        {
          "address": {
            "pubKeyHash": "0x28513f689866ba1507764405a89deac9eefc82c159d8e813162d2e9a32bd14b3",
            "type": 0
          },
          "type": 0
        }
      ]
    },
    "0x000072a6f80d1fff0354dd05c250d9562e98c62b9de31898c81a318361e664110000": {
      "amount": "49300",
      "type": 3,
      "unlockConditions": [
        {
          "address": {
            "pubKeyHash": "0xe68c39e00324a555323473172e723dc2fdbe761058c44bd4594112031a90cdc6",
            "type": 0
          },
          "type": 0
        },
        {
          "amount": "46800",
          "returnAddress": {
            "pubKeyHash": "0xd027b3c85eab8d347393dd1a96f53806c5f0db10d1f56f2cf0c78534cb7f69ec",
            "type": 0
          },
          "type": 1
        }
      ]
    },

    ...

    "0xffffd4f26740a670bf55b86427f9ac6d48cbc3c4ae7ce2087b2b8201ea138e7a0200": {
      "amount": "99100",
      "immutableFeatures": [
        {
          "address": {
            "nftId": "0x25cbec33b01198f17463f29825e32111e4d9872e912e6cf60376a77102af6ba2",
            "type": 16
          },
          "type": 1
        },
        {
          "data": "0x7b227374616e64617264223a224952433237222c2276657273696f6e223a2276312e30222c2274797065223a22696d6167652f706e67222c22757269223a22697066733a2f2f6261666b726569617078376b637a6866756b7833346c646833707868646970356b677668323337646c687035356b6f65666a6f36747975706e6a34222c226e616d65223a224e4654206261626261222c226465736372697074696f6e223a226261626261222c226973737565724e616d65223a22536f6f6e617665727365222c22636f6c6c656374696f6e4964223a22307832356362656333336230313139386631373436336632393832356533323131316534643938373265393132653663663630333736613737313032616636626132222c22636f6c6c656374696f6e4e616d65223a22436f6c6c656374696f6e2041222c2261747472696275746573223a7b2270726f7073223a7b7d2c227374617473223a7b7d7d2c22726f79616c74696573223a7b22726d7331717173307a773471703632393078797a7470396b72687173726c66723333796533743263717577393437637337356a6a72366b6c78716c6576397a223a302e367d2c22736f6f6e6176657273654964223a22307838316461373466383130313737313035646563303333316232316461346330643532636437313764227d",
          "type": 2
        }
      ],
      "nftId": "0x0000000000000000000000000000000000000000000000000000000000000000",
      "type": 6,
      "unlockConditions": [
        {
          "address": {
            "pubKeyHash": "0xc39397505808f1df0c7e34e52118a5f13e77358b0764973b0bafd28cea266e2f",
            "type": 0
          },
          "type": 0
        }
      ]
    }
  },
  "treasuryOutputAmount": 0
}
```
