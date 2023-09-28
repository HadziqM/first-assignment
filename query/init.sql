CREATE TABLE kategori (
  id serial,
  name text not null,
  PRIMARY KEY(id)
);
CREATE TABLE status (
  id serial,
  name text not null,
  PRIMARY KEY(id)
);
CREATE TABLE produk (
  id serial,
  name text not null,
  harga int not null,
  kategori_id int,
  status_id int,
  PRIMARY KEY(id),
  CONSTRAINT fk_kategori FOREIGN KEY(kategori_id) REFERENCES kategori(id),
  CONSTRAINT fk_status FOREIGN KEY(status_id) REFERENCES status(id)
);
