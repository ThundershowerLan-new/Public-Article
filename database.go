package main

import (
	"database/sql"
	"errors"

	_ "modernc.org/sqlite"
)

const path string = "database.db"

var database *sql.DB

func InitializeDatabase() error {
	var err error

	if database, err = sql.Open("sqlite", path); err != nil {
		return err
	}

	return nil
}

func createUser(name string, password string) (*user, error) {
	if _, err := database.Exec(`INSERT INTO users (name, password) VALUES (?, ?)`, name, password); err != nil {
		return nil, err
	}

	var resultId uint64
	if err := database.QueryRow(`SELECT last_insert_rowid()`).Scan(&resultId); err != nil {
		return nil, err
	}

	return &user{
		Id:       resultId,
		Name:     name,
		Password: password,
		Articles: nil,
	}, nil
}

func verifyUser(name string, password string) (*user, error) {
	var resultId uint64
	var resultPassword string

	if err := database.QueryRow("SELECT id, password FROM users WHERE name = ?", name).Scan(&resultId, &resultPassword); err != nil {
		return nil, err
	}

	if resultPassword != password {
		return nil, errors.New("wrong password")
	}

	articles, err := getArticles(resultId)
	if err != nil {
		return nil, err
	}

	return &user{
		Id:       resultId,
		Name:     name,
		Password: password,
		Articles: articles,
	}, nil
}

func getUser(id uint64) (*user, error) {
	var resultName, resultPassword string

	if err := database.QueryRow("SELECT name, password FROM users WHERE id = ?", id).Scan(&resultName, &resultPassword); err != nil {
		return nil, err
	}

	articles, err := getArticles(id)
	if err != nil {
		return nil, err
	}

	return &user{
		Id:       id,
		Name:     resultName,
		Password: resultPassword,
		Articles: articles,
	}, nil
}

func getArticles(id uint64) ([]article, error) {
	rows, err := database.Query("SELECT id, title, body FROM articles WHERE creator = ?", id)
	if err != nil {
		return nil, err
	}

	var articles []article

	for rows.Next() {
		var article article

		if err := rows.Scan(&article.Id, &article.Title, &article.Body); err != nil {
			return nil, err
		}
		article.Creator = id

		articles = append(articles, article)
	}

	return articles, nil
}

func createArticle(title string, body string, creator uint64) (*article, error) {
	if _, err := database.Exec(`INSERT INTO articles (title, body, creator)  VALUES (?, ?, ?)`, title, body, creator); err != nil {
		return nil, err
	}

	var resultId uint64
	if err := database.QueryRow(`SELECT last_insert_rowid()`).Scan(&resultId); err != nil {
		return nil, err
	}

	return &article{
		Id:      resultId,
		Title:   title,
		Body:    body,
		Creator: creator,
	}, nil
}

func getArticle(id uint64) (*article, error) {
	var resultTitle, resultBody string
	var resultCreator uint64

	if err := database.QueryRow(`SELECT title, body, creator FROM articles WHERE id = ?`, id).Scan(&resultTitle, &resultBody, &resultCreator); err != nil {
		return nil, err
	}

	return &article{
		Id:      id,
		Title:   resultTitle,
		Body:    resultBody,
		Creator: resultCreator,
	}, nil
}
