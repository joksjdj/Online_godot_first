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

/* This is optional */
SET GLOBAL validate_password.policy = LOW;
SET GLOBAL validate_password.length = 1;


CREATE DATABASE first_godot_project;

/*
test is a placeholder and can be changed.
IDENTIFIED BY 'your_password'
*/
CREATE USER 'test'@'%' IDENTIFIED BY 'test'; 
GRANT ALL PRIVILEGES ON first_godot_project.* TO 'test'@'%';
FLUSH PRIVILEGES;

USE first_godot_project;

CREATE TABLE players (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(50) NOT NULL,
    CONSTRAINT password_not_empty CHECK (password <> ''),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    highscore INT DEFAULT 0,
    last_game INT DEFAULT 0
);
DESCRIBE players;


/* Testing */
INSERT INTO players (username, password)
VALUES ("test", "test");