-- Add migration script here

create table ImagesData (
    id bigint not null auto_increment,
    mime varchar(255) not null,
    content longblob not null,
    primary key (id)
);

create table Collections (
    id varchar(24) not null,
    name varchar(255) not null,
    thumbnail_id bigint,
    primary key (id),
    foreign key (thumbnail_id) references ImagesData (id) on delete set null
);

create table Books (
    id varchar(24) not null,
    name varchar(255) not null,
    collection_id varchar(24) not null,
    primary key (id),
    foreign key (collection_id) references Collections (id) on delete cascade
);

create table BookImages (
    id varchar(24) not null,
    book_id varchar(24) not null,
    page_number int not null,
    image_id bigint not null,
    file_name varchar(255) not null,
    primary key (id),
    foreign key (book_id) references Books (id) on delete cascade,
    foreign key (image_id) references ImagesData (id) on delete cascade
);