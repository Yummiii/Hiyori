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
    thumb bigint,
    primary key (id),
    foreign key (thumb) references ImagesData(id)
);

create table Books (
    id varchar(24) not null,
    title varchar(255) not null,
    cover bigint not null,
    collection varchar(24) not null,
    primary key (id),
    foreign key (cover) references ImagesData(id),
    foreign key (collection) references Collections(id)
);
create index IX_books_collection on Books(collection);
create index IX_books_title on Books(title);

create table Images (
    id varchar(24) not null,
    book varchar(24) not null,
    page int not null,
    file_name varchar(255) not null,
    data bigint not null,
    primary key (id),
    foreign key (book) references Books(id) on delete cascade,
    foreign key (data) references ImagesData(id) on delete cascade
);
create index IX_images_book on Images(book);
create index IX_images_page on Images(page);
create index IX_images_file_name on Images(file_name);