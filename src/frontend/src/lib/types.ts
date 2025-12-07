export type Book = {
  id: Number;
  name: String;
};

export type NewBook = {
  name: String | null;
};

export type Customer = {
  id: Number;
  name: String;
  age: number;
  sex: number;
  crimes: number;
};

export type NewCustomer = {
  name: String | null;
  age: String | null;
  sex: String | null;
};

export type Borrow = {
  id: Number;
  book_id: Number;
  customer_id: Number;
  duration: Number;
};

export type NewBorrow = {
  book_id: String | null;
  customer_id: String | null;
  duration: String | null;
};
