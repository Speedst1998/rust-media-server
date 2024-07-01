CREATE TABLE WatchedFolders (
    path varchar(255) not null unique,
    folder_name varchar(255) not null
);

CREATE TABLE Videos (
    id varchar(50) not null unique,
    file_name varchar(255) not null,
    watched_folder not null,
    CONSTRAINT FK_watchedFolder 
        FOREIGN KEY (watched_folder) 
        REFERENCES WatchedFolders(path)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)