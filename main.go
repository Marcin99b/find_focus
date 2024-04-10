package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

type SetMoodRequest struct {
	Name string `json:"name" validate:"required"`
}

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Test")
	})
	e.POST("/", func(c echo.Context) (err error) {
		r := new(SetMoodRequest)
		if err = c.Bind(r); err != nil {
			return echo.NewHTTPError(http.StatusBadRequest, err.Error())
		}
		return c.JSON(http.StatusOK, r)
	})
	e.Logger.Fatal(e.Start(":8888"))
}
