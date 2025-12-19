export type Book = {
  id: number;
  name: String;
};

export type NewBook = {
  name: String | null;
};

export type Customer = {
  id: number;
  name: String;
  age: number;
  sex: number;
  crimes: number;
};

export type NewCustomer = {
  name: String | null;
  age: String | null;
  sex: String | null;
  crimes: String | null;
};

export type BorrowJoined = {
  id: number;
  book_id: number;
  book_name: String;
  customer_id: number;
  customer_name: String;
  duration: number;
};

export type Borrow = {
  id: number;
  book_id: String | null;
  customer_id: String | null;
  duration: String | null;
};

export type NewBorrow = {
  book_id: String | null;
  customer_id: String | null;
  duration: String | null;
};
