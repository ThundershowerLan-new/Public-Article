package main

import (
	"database/sql"
	"errors"
	"net/http"
	"os"
	"strconv"

	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/middleware"
	_ "modernc.org/sqlite"
)

var e = echo.New()

func login(c *echo.Context) error {
	name, password := c.FormValue("name"), c.FormValue("password")

	if name == "" || name == "Administrator" || password == "" {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	var user *user
	var err error

	if user, err = verifyUser(name, password); err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			if user, err = createUser(name, password); err != nil {
				return c.JSON(http.StatusInternalServerError, response{
					Code: http.StatusInternalServerError,
					Data: "Internal Server Error",
				})
			}

			c.SetCookie(&http.Cookie{
				Name:     "password",
				Value:    user.Password,
				HttpOnly: true,
			})

			return c.JSON(http.StatusCreated, response{
				Code: http.StatusCreated,
				Data: user,
			})
		}

		return c.JSON(http.StatusUnauthorized, response{
			Code: http.StatusUnauthorized,
			Data: "Unauthorized",
		})
	}

	c.SetCookie(&http.Cookie{
		Name:     "password",
		Value:    user.Password,
		HttpOnly: true,
	})

	return c.JSON(http.StatusOK, response{
		Code: http.StatusOK,
		Data: user,
	})
}

func create(c *echo.Context) error {
	title, body, name := c.FormValue("title"), c.FormValue("body"), c.FormValue("name")
	creator, err := strconv.ParseUint(c.FormValue("creator"), 10, 64)
	if title == "" || body == "" || name == "" || err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	password, err := c.Cookie("password")
	if err != nil {
		return c.JSON(http.StatusUnauthorized, response{
			Code: http.StatusUnauthorized,
			Data: "Unauthorized",
		})
	}

	if _, err = verifyUser(name, password.Value); err != nil {
		return c.JSON(http.StatusUnauthorized, response{
			Code: http.StatusUnauthorized,
			Data: "Unauthorized",
		})
	}

	article, err := createArticle(title, body, creator)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, response{
			Code: http.StatusInternalServerError,
			Data: "Internal Server Error",
		})
	}

	return c.JSON(http.StatusCreated, response{
		Code: http.StatusCreated,
		Data: article,
	})
}

func getUser(c *echo.Context) error {
	id, err := strconv.ParseUint(c.FormValue("id"), 10, 64)

	if err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	user, err := selectUser(id)

	if err != nil {
		return c.JSON(http.StatusNotFound, response{
			Code: http.StatusNotFound,
			Data: "Not Found",
		})
	}

	user.Password = ""

	return c.JSON(http.StatusOK, response{
		Code: http.StatusOK,
		Data: user,
	})
}

func getArticle(c *echo.Context) error {
	id, err := strconv.ParseUint(c.FormValue("id"), 10, 64)

	if err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	article, err := selectArticle(id)

	if err != nil {
		return c.JSON(http.StatusNotFound, response{
			Code: http.StatusNotFound,
			Data: "Not Found",
		})
	}

	return c.JSON(http.StatusOK, response{
		Code: http.StatusOK,
		Data: article,
	})
}

func removeArticle(c *echo.Context) error {
	name := c.FormValue("name")
	password, err := c.Cookie("password")
	if name == "" || err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	user, err := verifyUser(name, password.Value)
	if err != nil {
		return c.JSON(http.StatusUnauthorized, response{
			Code: http.StatusUnauthorized,
			Data: "Unauthorized",
		})
	}

	id, err := strconv.ParseUint(c.Param("id"), 10, 64)
	if err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	if id != user.Id {
		return c.JSON(http.StatusUnauthorized, response{
			Code: http.StatusUnauthorized,
			Data: "Unauthorized",
		})
	}

	if err := deleteArticle(id); err != nil {
		return c.JSON(http.StatusNotFound, response{
			Code: http.StatusNotFound,
			Data: "Not Found",
		})
	}

	return c.JSON(http.StatusOK, response{
		Code: http.StatusOK,
		Data: "OK",
	})
}

func recommend(c *echo.Context) error {
	var articles []article

	rows, err := database.Query(`SELECT id, title, body, creator FROM articles`)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, response{
			Code: http.StatusInternalServerError,
			Data: "Internal Server Error",
		})
	}

	rows.Next()
	for rows.Next() {
		var article article
		err := rows.Scan(&article.Id, &article.Title, &article.Body, &article.Creator)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, response{
				Code: http.StatusInternalServerError,
				Data: "Internal Server Error",
			})
		}
		articles = append(articles, article)
	}

	return c.JSON(http.StatusOK, response{
		Code: http.StatusOK,
		Data: articles,
	})
}

func main() {
	e.Static("/static", "static")
	e.Use(middleware.RequestLogger())

	e.File("/", "html/index.html")
	e.File("/error", "html/error.html")
	e.File("/login", "html/login.html")
	e.File("/user/:id", "html/user.html")
	e.File("/create", "html/create.html")
	e.File("/article/:id", "html/article.html")

	e.POST("/login", login)
	e.POST("/user", getUser)
	e.POST("/create", create)
	e.POST("/article", getArticle)
	e.POST("/recommend", recommend)
	e.POST("/database/users", func(c *echo.Context) error {
		if c.FormValue("password") != os.Getenv("DATABASE") {
			return c.JSON(http.StatusUnauthorized, response{
				Code: http.StatusUnauthorized,
				Data: "Bad Request",
			})
		}

		rows, err := database.Query("SELECT id, name, password FROM users")
		if err != nil {
			return c.JSON(http.StatusInternalServerError, response{
				Code: http.StatusInternalServerError,
				Data: "Internal Server Error",
			})
		}

		var users []user

		for rows.Next() {
			var user user

			if err := rows.Scan(&user.Id, &user.Name, &user.Password); err != nil {
				return c.JSON(http.StatusInternalServerError, response{
					Code: http.StatusInternalServerError,
					Data: "Internal Server Error",
				})
			}

			users = append(users, user)
		}

		return c.JSON(http.StatusOK, response{
			Code: http.StatusOK,
			Data: users,
		})
	})
	e.POST("/database/articles", func(c *echo.Context) error {
		if c.FormValue("password") != os.Getenv("DATABASE") {
			return c.JSON(http.StatusUnauthorized, response{
				Code: http.StatusUnauthorized,
				Data: "Bad Request",
			})
		}

		rows, err := database.Query("SELECT id, title, body, creator FROM articles")
		if err != nil {
			return c.JSON(http.StatusInternalServerError, response{
				Code: http.StatusInternalServerError,
				Data: "Internal Server Error",
			})
		}

		var articles []article

		for rows.Next() {
			var article article

			if err := rows.Scan(&article.Id, &article.Title, &article.Body, &article.Creator); err != nil {
				return c.JSON(http.StatusInternalServerError, response{
					Code: http.StatusInternalServerError,
					Data: "Internal Server Error",
				})
			}

			articles = append(articles, article)
		}

		return c.JSON(http.StatusOK, response{
			Code: http.StatusOK,
			Data: articles,
		})
	})

	e.DELETE("/article/:id", removeArticle)

	e.HTTPErrorHandler = func(c *echo.Context, err error) {
		_ = c.Redirect(http.StatusSeeOther, "/error")
	}

	if err := InitializeDatabase(); err != nil {
		e.Logger.Error("Failed to initialize database", "error", err)
	}

	if err := e.Start("0.0.0.0:" + os.Getenv("PORT")); err != nil {
		e.Logger.Error("Failed to start server", "error", err)
	}
}
