-- Your SQL goes here
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    name VARCHAR(255),
    student_id INT
);


CREATE TABLE classes (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    class_name VARCHAR(255) NOT NULL,
    start_time DATETIME,
    end_time DATETIME,
    note TEXT
);


CREATE TABLE users_classes (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    user_id INT,
    class_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (class_id) REFERENCES classes(id)
);
