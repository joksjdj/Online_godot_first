/*
Downloading mysql in bash

Update your package list to avoid installing an outdated version of mysql.
    =========================================
    sudo apt update
    =========================================

    =========================================
    sudo apt install mysql-server
    sudo mysql_secure_installation
    =========================================
or
    =========================================
    sudo apt install mariadb-server mariadb-client -y
    =========================================
depending on what OS you are using.


Checking if mysql is running
    =========================================
    sudo systemctl status mysql
    =========================================

Type:
    =========================================
    sudo mysql -u root -p
    =========================================
or
    =========================================
    sudo mysql
    =========================================
to use mysql
*/

SET GLOBAL validate_password.policy = LOW;
SET GLOBAL validate_password.length = 1;


CREATE DATABASE card_game;

/*
test is a placeholder and can be changed.
IDENTIFIED BY 'your_password'
*/
CREATE USER 'test'@'%' IDENTIFIED BY 'test';
GRANT ALL PRIVILEGES ON card_game.* TO 'test'@'%';
FLUSH PRIVILEGES;

USE card_game;

CREATE TABLE active_games (
    id INT AUTO_INCREMENT PRIMARY KEY,
    lobby_name VARCHAR(50) NOT NULL,
    lobby_password VARCHAR(50),

    playing INT DEFAULT 0,
    placed_cards JSON,
    untouched_cards JSON
);
DESCRIBE active_games;

CREATE TABLE players (
    id INT DEFAULT 0,
    name VARCHAR(50) NOT NULL,
    ip_address VARCHAR(50) NOT NULL,

    game_id INT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES active_games(id),
    cards JSON
);
DESCRIBE players;