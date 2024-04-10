package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

const (
	Focused    byte = 0
	Distracted byte = 1
	Idle       byte = 2
	Tired      byte = 3
	Bored      byte = 4
)

type SetMoodRequest struct {
	MoodId byte `json:"moodId" validate:"required"`
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
		return c.NoContent(http.StatusOK)
	})
	e.Logger.Fatal(e.Start(":8888"))
}
