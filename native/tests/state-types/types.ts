import * as borsh from "borsh";

export class CounterAccount {
    count: BigInt;

    constructor({
        count,
    }: {
        count: BigInt
    }) {
        this.count = count;
    }
}

export const schema: borsh.Schema = {
    struct: {
        count: 'u64'
    }
};

export const COUNTER_SIZE = borsh.serialize(schema, new CounterAccount({
    count: BigInt(0)
})).length;