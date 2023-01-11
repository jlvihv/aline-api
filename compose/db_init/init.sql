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
        'https://ethereum.api.hamsternet.io',
        'wss://ethereum-ws.api.hamsternet.io'
    ),
    (
        'ethereum',
        'testnet-goerli',
        'https://ethereum-goerli.api.hamsternet.io',
        'wss://ethereum-goerli-ws.api.hamsternet.io'
    ),
    (
        'sui',
        'mainnet',
        'https://sui.api.hamsternet.io',
        'wss://sui-ws.api.hamsternet.io'
    ),
    (
        'avalanche',
        'mainnet',
        'https://avalanche.api.hamsternet.io',
        ''
    ),
    (
        'optimism',
        'mainnet',
        'https://optimism.api.hamsternet.io',
        'wss://optimism-ws.api.hamsternet.io'
    ),
    (
        'near',
        'mainnet',
        'https://near.api.hamsternet.io',
        ''
    ),
    (
        'starkware',
        'mainnet',
        'https://starkware.api.hamsternet.io',
        ''
    ),
    (
        'bsc',
        'mainnet',
        'https://bsc.api.hamsternet.io',
        'wss://bsc-ws.api.hamsternet.io'
    ),
    (
        'aptos',
        'mainnet',
        'https://aptos.api.hamsternet.io',
        ''
    ),
    (
        'polygon',
        'mainnet',
        'https://polygon.api.hamsternet.io',
        ''
    );