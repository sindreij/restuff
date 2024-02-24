import { z } from 'zod';

export async function rpcCall<Schema extends z.ZodTypeAny>(
    call: string,
    schema: Schema,
    input?: unknown
): Promise<z.infer<Schema>> {
    let urlSearchParams = `?input=${encodeURIComponent(JSON.stringify(input))}`;

    let res = await fetch(`/api/srpc/${call}` + urlSearchParams.toString(), {
        signal: AbortSignal.timeout(10000),
    });

    if (res.status !== 200) {
        throw new Error(`rpcCall: unexpected status code: ${res.status}`);
    }

    let data = await res.json();

    return schema.parse(data);
}
