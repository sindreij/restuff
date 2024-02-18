export async function rpcCall(call: string) {
    let res = await fetch(`/api/srpc/${call}`, { signal: AbortSignal.timeout(10000) });

    if (res.status !== 200) {
        throw new Error(`rpcCall: unexpected status code: ${res.status}`);
    }

    return await res.json();
}
