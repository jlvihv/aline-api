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