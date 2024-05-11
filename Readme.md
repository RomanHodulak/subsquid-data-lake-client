# A library to stream on-chain data

Develop a Rust library utilizing the [EVM API](https://docs.subsquid.io/subsquid-network/reference/evm-api/) exposed by the data lake. 

The library should produce a continous stream ofraw data batches matching the filters provided by the user. You may take inspiration from the following typescript-based code snippet 
(but feel free to design your own interface):

```ts
const dataStream = new DataStream()
    .setDataSource({
        subsquid: 'https://v2.archive.subsquid.io/network/ethereum-mainnet'
    })
    .select({
        logs: {
            topics: true,
            data: true,
        },
        transaction: {
            hash: true,
        }
    })
    .addLog({
        where: {
            range: {from: 6_082_465},
            address: ['0xabcd'],
            topic0: ['Burn(address,int24,int24,uint128,uint256)', 'Initialize(uint160,int24)']
        },
        join: {
            transaction: true
        }
    })
    .addTx({
        where: {
            to: ['0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045']
        },
        join: {
            logs: true,
            stateDiffs: true,
            traces: true
        }
    })
    .addTx({
        where: {
            from: ['0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045']
        }
    })
```

The lib should be suitable as a data source for downstream decoding, or piping the output to external sources such as Kafka

## Discussion

We will discuss during the call how the following features may be implemented. It is not required to implement them.

The library should be designed so that it can be extended to address the following features:
- Restarts. The library clients should be able to resume the data stream from a block of choice. 
- Real-time processing. The data lake currently have an offset of about 1000-2000 blocks from the chain tip. It should be possible to support an EVM RPC endpoint as an additional data source for the "hot blocks" not yet present in the data lake
- Reorg support. Think how the library can support chain reorgs and stream this data to the downstream clients
- Parallel queries. Think how one can speed up the data ingestion by submitting parallel requests to different block ranges. What would be a sensible interface for that? What are the trade-offs?
- How would you design the library so that the consumer can decode the log and tx data as fast as possible (e.g. in multiple threads)?
- What extra queries and/or interfaces do you think the library should support in the future? How would you design a query interface of the library?

## Evaluation criteria

Overall we at Subdquid strive to deliver the best developer experience. That's why having an intuitive and robust interface for our tooling is a key. We will evaluate the ergonomics of the library and how you would plan the roadmap for the future development of the sdk. 

