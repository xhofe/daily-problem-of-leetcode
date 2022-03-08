package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"path/filepath"
	"regexp"
	"strings"
)

const head = `
# daily-problem-of-leetcode

![ranking](https://leetcode-badge.haozibi.dev/v1cn/ranking/xhofe.svg?logo=leetcode&color=4299E1)
![solved](https://leetcode-badge.haozibi.dev/v1cn/solved/xhofe.svg)
![accepted-rate](https://leetcode-badge.haozibi.dev/v1cn/accepted-rate/xhofe.svg?color=9F7AEA)

![records](https://leetcode-badge.haozibi.dev/v1cn/chart/submission-calendar/xhofe.svg)

`

func problemStr(year, month, filename string) string {
	log.Printf("generate problem: %s\n", filename)
	day := filename[:2]
	name := filename[3:]
	name = name[:strings.LastIndex(name, ".")]
	return fmt.Sprintf(`- %s: [%s](./%s/%s/%s) [![leetcode](https://img.shields.io/badge/-link-38B2AC?logo=leetcode)](https://leetcode-cn.com/problems/%s/)
`,
		day, name, year, month, filename, name)
}

func monthStr(year, month string) string {
	log.Printf("generate month: %s\n", month)
	str := "### " + month + "\n"
	problems, err := ioutil.ReadDir(filepath.Join(year, month))
	if err != nil {
		log.Fatal(err)
	}
	for _, problem := range problems {
		str += problemStr(year, month, problem.Name())
	}
	return str
}

func yearStr(year string) string {
	log.Printf("generate year: %s\n", year)
	str := "## " + year + "\n"
	months, err := ioutil.ReadDir(year)
	if err != nil {
		log.Fatal(err)
	}
	for _, month := range months {
		str += monthStr(year, month.Name())
	}
	return str
}

func main() {
	readme := head
	years, err := ioutil.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}
	yearRegexp := regexp.MustCompile("^\\d{4}$")
	for _, year := range years {
		if year.IsDir() && yearRegexp.Match([]byte(year.Name())) {
			readme += yearStr(year.Name())
		}
	}
	err = ioutil.WriteFile("README.md", []byte(readme), 0644)
	if err != nil {
		log.Fatal(err)
	}
}
