import { z } from 'zod';

export async function rpcCall<Schema extends z.ZodTypeAny>(
    call: string,
    schema: Schema,
    queryParams?: Record<string, string>
): Promise<z.infer<Schema>> {
    let urlSearchParams = new URLSearchParams(queryParams);

    let res = await fetch(`/api/srpc/${call}` + urlSearchParams.toString(), {
        signal: AbortSignal.timeout(10000),
    });

    if (res.status !== 200) {
        throw new Error(`rpcCall: unexpected status code: ${res.status}`);
    }

    let data = await res.json();

    return schema.parse(data);
}
