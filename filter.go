package main

import (
	"io"
	"log"
	"os"
	"os/exec"
	"sort"
	"strings"
)

func choose(strs []string, f func(string) bool) []string {
	var r []string
	for _, str := range strs {
		if f(str) {
			r = append(r, str)
		}
	}
	return r
}

func contains(strs []string, str string) bool {
	for _, s := range strs {
		if s == str {
			return true
		}
	}
	return false
}

func uniq(strs []string) []string {
	var r []string
	for _, str := range strs {
		if !contains(r, str) {
			r = append(r, str)
		}
	}
	return r
}

type byLength []string

func (s byLength) Len() int {
	return len(s)
}
func (s byLength) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}
func (s byLength) Less(i, j int) bool {
	return len(s[i]) < len(s[j])
}

func unescape(str string) string {
	out, err := exec.Command("sh", "-c", "echo "+str).Output()
	if err != nil {
		log.Fatal(err)
	}
	return strings.TrimRight(string(out), "\r\n")
}

func mapStrs(strs []string, f func(str string) string) []string {
	var r []string
	for _, str := range strs {
		r = append(r, f(str))
	}
	return r
}

type readable interface {
	read() ([]byte, error)
}

type stdin struct {
	in io.Reader
}

func (r stdin) read() ([]byte, error) {
	char := make([]byte, 1)
	_, err := r.in.Read(char)
	return char, err
}

type filter struct {
	in   readable
	str  string
	buf  []byte
	mask []byte
}

func (f filter) read() ([]byte, error) {
	for {
		char, err := f.in.read()
		if err == io.EOF {
			return f.dump(), err
		}
		f.buf = append(f.buf, char...)
		if f.isSecure() {
			f.clear()
			return f.mask, err
		} else if f.canDump() {
			return f.dump(), err
		}
	}
}

func (f filter) isSecure() bool {
	return string(f.buf) == f.str
}

func (f filter) canDump() bool {
	return !strings.HasPrefix(f.str, string(f.buf))
}

func (f filter) dump() []byte {
	chars := f.buf
	f.clear()
	return chars
}

func (f filter) clear() {
	f.buf = []byte{}
}

type writer struct {
	in  readable
	out io.Writer
}

func (w writer) run() {
	for {
		char, err := w.in.read()
		if err == io.EOF {
			break
		} else if err != nil {
			log.Fatalf("Error reading from input: %s", err)
		} else {
			w.out.Write(char)
		}
	}
}

func env(key string) string {
  return os.Getenv(key)
}

func strs() []string {
	strs := os.Args[1:]
  strs = mapStrs(strs, env)
	strs = choose(strs, func(str string) bool { return len(str) > 2 })
	strs = append(strs, mapStrs(strs, unescape)...)
	strs = uniq(strs)
	sort.Sort(sort.Reverse(byLength(strs)))
	return strs
}

func filtered(i io.Reader, o io.Writer, strs []string, mask string) writer {
	in := readable(stdin{in: i})
	for _, str := range strs {
		in = filter{in: in, str: str, mask: []byte(mask)}
	}
	return writer{out: o, in: in}
}

func main() {
	filtered(os.Stdin, os.Stdout, strs(), "[secure]").run()
}
