package main

type user struct {
	Id       uint64    `json:"id"`
	Name     string    `json:"name"`
	Password string    `json:"password,omitempty"`
	Articles []article `json:"articles"`
}

type article struct {
	Id      uint64 `json:"id"`
	Title   string `json:"title"`
	Body    string `json:"body"`
	Creator uint64 `json:"creator"`
}

type response struct {
	Code int         `json:"code"`
	Data interface{} `json:"data"`
}
