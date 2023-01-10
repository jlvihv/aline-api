CREATE TABLE IF NOT EXISTS accounts (
    address varchar(255) NOT NULL,
    created_at varchar(255) NOT NULL,
    app_id_index int NOT NULL,
    PRIMARY KEY (address)
);
CREATE TABLE IF NOT EXISTS apps (
    account varchar(50) NOT NULL,
    id int NOT NULL,
    name varchar(50) NOT NULL,
    description varchar(255) NOT NULL,
    chain varchar(50) NOT NULL,
    network varchar(50) NOT NULL,
    api_key varchar(50) NOT NULL,
    created_at varchar(255) NOT NULL,
    http_link varchar(100) NOT NULL,
    websocket_link varchar(100) NOT NULL,
    PRIMARY KEY (account, id)
);
CREATE TABLE IF NOT EXISTS chains (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name varchar(50) NOT NULL,
    network varchar(50) NOT NULL,
    http_address varchar(255) NOT NULL,
    websocket_address varchar(255) NOT NULL
);
-- 向 chains 表插入数据时，name 和 network 字段需要小写，不然怕是找不到数据
INSERT INTO chains (name, network, http_address, websocket_address)
VALUES (
        'ethereum',
        'mainnet',
        'http://34.232.105.81:9912/ethereum',
        'http://34.232.105.81:9912/ethereum-ws'
    ),
    (
        'sui',
        'mainnet',
        'http://34.232.105.81:9912/sui',
        'http://34.232.105.81:9912/sui-ws'
    ),
    (
        'avalanche',
        'mainnet',
        'http://34.232.105.81:9912/avalanche',
        ''
    ),
    (
        'optimism',
        'mainnet',
        'http://34.232.105.81:9912/optimism',
        'http://34.232.105.81:9912/optimism-ws'
    ),
    (
        'near',
        'mainnet',
        'http://34.232.105.81:9912/near',
        ''
    ),
    (
        'starkware',
        'mainnet',
        'http://34.232.105.81:9912/starknet',
        ''
    ),
    (
        'bsc',
        'mainnet',
        'http://34.232.105.81:9912/bsc',
        'http://34.232.105.81:9912/bsc-ws'
    ),
    (
        'aptos',
        'mainnet',
        'http://34.232.105.81:9912/aptos',
        ''
    ),
    (
        'polygon',
        'mainnet',
        'http://34.232.105.81:9912/polygon',
        ''
    );