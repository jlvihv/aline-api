CREATE TABLE IF NOT EXISTS account(
                address varchar(255) NOT NULL,
                created_at varchar(255) NOT NULL,
                app_id_index int NOT NULL,
                PRIMARY KEY (address)
            );

CREATE TABLE IF NOT EXISTS app (
                id int NOT NULL,
                account varchar(255) NOT NULL,
                name varchar(255) NOT NULL,
                description varchar(255) NOT NULL,
                chain varchar(255) NOT NULL,
                network varchar(255) NOT NULL,
                api_key varchar(255) NOT NULL,
                created_at varchar(255) NOT NULL,
                today_requests int NOT NULL,
                total_requests int NOT NULL,
                http_link varchar(255) NOT NULL,
                websocket_link varchar(255) NOT NULL,
                PRIMARY KEY (account, id)
            );