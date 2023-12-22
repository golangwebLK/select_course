-- Your SQL goes here
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    username VARCHAR(255),
    password VARCHAR(255),
    name VARCHAR(255),
    student_id INT,
    class_id INT
);


CREATE TABLE classes (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    class_name VARCHAR(255) NOT NULL,
    start_time DATETIME,
    end_time DATETIME,
    note TEXT,
    speciality_id INT,
    class_id INT
);


CREATE TABLE users_classes (
    id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    user_id INT,
    class_id INT
);


create table specialities
(
    id         INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    name       longtext    not null,
    enable     tinyint(1)  not null,
    note       longtext    null,
    speciality_id INT NOT NULL
);
