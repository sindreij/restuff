export const client = {
    get_first_user: async () => rpc_call('get_first_user'),
    get_second_user: async () => rpc_call('get_second_user'),
};
