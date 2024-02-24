// This file is generated by srpc-derive

import { rpcCall } from './rpcClient';
import { z } from 'zod';

export const getFirstUserSchema = z.object({
    id: z.number(),
    name: z.string(),
    foo: z.array(z.string()),
});

export const getSecondUserSchema = z.object({
    id: z.number(),
    name: z.string(),
    foo: z.array(z.string()),
});

export const getUserSchema = z.object({
    id: z.number(),
    name: z.string(),
    foo: z.array(z.string()),
});

export const createUserSchema = z.object({
    id: z.number(),
    name: z.string(),
    foo: z.array(z.string()),
});

export const getUsersSchema = z.array(
    z.object({
        id: z.number(),
        name: z.string(),
        foo: z.array(z.string()),
    })
);

type GetUserParams = {
    id: number;
};

type CreateUserParams = {
    name: string;
    age: number;
};

export const client = {
    getFirstUser: async () => rpcCall('get_first_user', getFirstUserSchema),
    getSecondUser: async () => rpcCall('get_second_user', getSecondUserSchema),
    getUser: async (params: GetUserParams) => rpcCall('get_user', getUserSchema, params),
    createUser: async (params: CreateUserParams) =>
        rpcCall('create_user', createUserSchema, params),
    getUsers: async () => rpcCall('get_users', getUsersSchema),
};
